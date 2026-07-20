import { realpath, stat } from "node:fs/promises";
import { basename, dirname, relative } from "node:path";
import process from "node:process";
import { BrowserWindow, dialog, ipcMain } from "electron";
import type {
  IpcChannel,
  IpcRequest,
  IpcResponse,
  IpcResult,
  ProjectSnapshot,
  SelectedPath,
} from "../shared/ipc.js";
import type { WorkspaceDatabase } from "./database/index.js";
import type { RustService } from "./rust-service.js";
import type { IpcMainInvokeEvent, OpenDialogOptions } from "electron";

type Handler<C extends IpcChannel> = (
  request: IpcRequest<C>,
  event: IpcMainInvokeEvent,
) => IpcResponse<C> | Promise<IpcResponse<C>>;

export function handleIpc<C extends IpcChannel>(
  channel: C,
  handler: Handler<C>,
) {
  ipcMain.handle(channel, async (event, request: IpcRequest<C>) => {
    try {
      return {
        ok: true,
        value: await handler(request, event),
      } satisfies IpcResult<IpcResponse<C>>;
    } catch (error) {
      return {
        ok: false,
        error: {
          code: "IPC_HANDLER_ERROR",
          message: error instanceof Error ? error.message : String(error),
        },
      } satisfies IpcResult<IpcResponse<C>>;
    }
  });
}

export function registerIpc(database: WorkspaceDatabase, rust: RustService) {
  const grantedPaths = new Set<string>();
  let activeProjectId: number | null = null;
  handleIpc("app:info", () => ({
    platform: process.platform,
    versions: {
      electron: process.versions.electron ?? "unknown",
      chrome: process.versions.chrome ?? "unknown",
      node: process.versions.node,
    },
  }));
  handleIpc("database:status", () => ({
    path: database.path,
    version: database.version,
  }));
  handleIpc("workspace:load", ({ rootPath }) =>
    database.loadWorkspace(rootPath),
  );
  handleIpc("workspace:save", ({ name, rootPath, state }) =>
    database.saveWorkspace(name, rootPath, state),
  );
  handleIpc("api:request", (request) => proxyRustRequest(request, rust));
  handleIpc("window:control", ({ action }, event) => {
    const window = BrowserWindow.fromWebContents(event.sender);
    if (!window) throw new Error("No window is associated with this request");

    switch (action) {
      case "minimize":
        window.minimize();
        break;
      case "toggle-maximize":
        if (window.isMaximized()) window.unmaximize();
        else window.maximize();
        break;
      case "open-devtools": {
        const focusDevTools = () => event.sender.devToolsWebContents?.focus();
        if (event.sender.isDevToolsOpened()) focusDevTools();
        else {
          event.sender.once("devtools-opened", focusDevTools);
          event.sender.openDevTools({ mode: "detach" });
        }
        break;
      }
      case "close":
        window.close();
        break;
    }

    return { maximized: !window.isDestroyed() && window.isMaximized() };
  });
  handleIpc("files:pick", async (_request, event) => {
    const window = BrowserWindow.fromWebContents(event.sender);
    const options: OpenDialogOptions = {
      title: "Open Unity AssetBundles",
      buttonLabel: "Load",
      properties: ["openFile", "openDirectory", "multiSelections"],
    };
    const result = window
      ? await dialog.showOpenDialog(window, options)
      : await dialog.showOpenDialog(options);
    if (result.canceled) return [];
    return inspectPaths(result.filePaths, grantedPaths);
  });
  handleIpc("files:inspect", ({ paths }) => inspectPaths(paths, grantedPaths));
  handleIpc("projects:list", () => database.listProjects());
  handleIpc("projects:create", async ({ name, sources }) => {
    if (sources.length === 0)
      throw new Error("A project needs at least one source");
    const rootPath = commonProjectRoot(sources);
    const project = database.createProject(
      name.trim() || sources[0].name,
      rootPath,
      sources,
    );
    try {
      await activateProject(project, rust);
    } catch (error) {
      database.deleteProject(project.id);
      throw error;
    }
    activeProjectId = project.id;
    return project;
  });
  handleIpc("projects:open", async ({ projectId }) => {
    const project = database.getProject(projectId);
    if (!project) throw new Error(`Project ${projectId} does not exist`);
    await activateProject(project, rust);
    activeProjectId = project.id;
    return database.touchProject(project.id);
  });
  handleIpc("projects:add-sources", async ({ projectId, sources }) => {
    if (sources.length === 0) {
      const project = database.getProject(projectId);
      if (!project) throw new Error(`Project ${projectId} does not exist`);
      return project;
    }
    activeProjectId = await ensureActiveProject(
      database,
      rust,
      projectId,
      activeProjectId,
    );
    const before = database.getProject(projectId);
    if (!before) throw new Error(`Project ${projectId} does not exist`);
    const project = database.addSources(projectId, sources);
    const oldIds = new Set(before.sources.map((source) => source.id));
    const added = project.sources.filter((source) => !oldIds.has(source.id));
    try {
      for (const source of added) {
        await requestRust(
          rust,
          "POST",
          "/environment/sources",
          sourceBody(source),
        );
      }
    } catch (error) {
      for (const source of added) database.removeSource(projectId, source.id);
      await activateProject(before, rust);
      throw error;
    }
    return project;
  });
  handleIpc("projects:remove-source", async ({ projectId, sourceId }) => {
    activeProjectId = await ensureActiveProject(
      database,
      rust,
      projectId,
      activeProjectId,
    );
    const before = database.getProject(projectId);
    if (!before) throw new Error(`Project ${projectId} does not exist`);
    await requestRust(
      rust,
      "DELETE",
      `/environment/sources/${encodeURIComponent(String(sourceId))}`,
    );
    try {
      return database.removeSource(projectId, sourceId);
    } catch (error) {
      await activateProject(before, rust);
      throw error;
    }
  });
  handleIpc("projects:refresh-source", async ({ projectId, sourceId }) => {
    activeProjectId = await ensureActiveProject(
      database,
      rust,
      projectId,
      activeProjectId,
    );
    await requestRust(
      rust,
      "POST",
      `/environment/sources/${encodeURIComponent(String(sourceId))}/refresh`,
    );
    return database.touchProject(projectId);
  });
}

async function ensureActiveProject(
  database: WorkspaceDatabase,
  rust: RustService,
  projectId: number,
  activeProjectId: number | null,
): Promise<number> {
  if (activeProjectId === projectId) return projectId;
  const project = database.getProject(projectId);
  if (!project) throw new Error(`Project ${projectId} does not exist`);
  await activateProject(project, rust);
  return projectId;
}

async function proxyRustRequest(
  request: IpcRequest<"api:request">,
  rust: RustService,
) {
  if (!request.path.startsWith("/") || request.path.startsWith("//")) {
    throw new Error("Rust API path must start with one slash");
  }
  if (
    (request.path === "/environment" && request.method !== "GET") ||
    (request.path.startsWith("/environment/sources") &&
      request.method !== "GET")
  ) {
    throw new Error("Environment mutations must use project IPC");
  }
  const baseUrl = process.env.UNITY_ASSET_RS_URL ?? rust.baseUrl;
  const response = await fetch(new URL(request.path, baseUrl), {
    method: request.method,
    headers: request.headers,
    body: request.body,
  });
  return {
    status: response.status,
    headers: Object.fromEntries(response.headers.entries()),
    body: await response.arrayBuffer(),
  };
}

function sourceBody(source: ProjectSnapshot["sources"][number]) {
  return {
    id: String(source.id),
    path: source.path,
    kind: source.kind,
    arknights: source.arknights,
  };
}

async function activateProject(project: ProjectSnapshot, rust: RustService) {
  await requestRust(rust, "PUT", "/environment", {
    sources: project.sources.map(sourceBody),
  });
}

async function requestRust(
  rust: RustService,
  method: string,
  path: string,
  body?: unknown,
) {
  const baseUrl = process.env.UNITY_ASSET_RS_URL ?? rust.baseUrl;
  const response = await fetch(new URL(path, baseUrl), {
    method,
    headers:
      body === undefined ? undefined : { "content-type": "application/json" },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  if (response.ok) return response;
  const text = await response.text();
  throw new Error(
    `Rust ${method} ${path} failed (${response.status}): ${text}`,
  );
}

function commonProjectRoot(sources: SelectedPath[]) {
  const roots = sources.map((source) =>
    source.kind === "directory" ? source.path : dirname(source.path),
  );
  let candidate = roots[0];
  for (const root of roots.slice(1)) {
    while (relative(candidate, root).startsWith("..")) {
      const parent = dirname(candidate);
      if (parent === candidate) return candidate;
      candidate = parent;
    }
  }
  return candidate;
}

async function inspectPaths(
  paths: string[],
  grantedPaths: Set<string>,
): Promise<SelectedPath[]> {
  const selected: SelectedPath[] = [];
  for (const path of [...new Set(paths)]) {
    const canonical = await realpath(path);
    const info = await stat(canonical);
    if (!info.isFile() && !info.isDirectory()) continue;
    grantedPaths.add(canonical);
    selected.push({
      arknights: false,
      path: canonical,
      name: basename(canonical),
      kind: info.isDirectory() ? "directory" : "file",
    });
  }
  return selected;
}
