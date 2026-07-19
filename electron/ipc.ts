import process from "node:process";
import { ipcMain } from "electron";
import type {
  IpcChannel,
  IpcRequest,
  IpcResponse,
  IpcResult,
} from "../shared/ipc.js";
import type { WorkspaceDatabase } from "./database/index.js";

type Handler<C extends IpcChannel> = (
  request: IpcRequest<C>,
) => IpcResponse<C> | Promise<IpcResponse<C>>;

export function handleIpc<C extends IpcChannel>(
  channel: C,
  handler: Handler<C>,
) {
  ipcMain.handle(channel, async (_event, request: IpcRequest<C>) => {
    try {
      return { ok: true, value: await handler(request) } satisfies IpcResult<
        IpcResponse<C>
      >;
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

export function registerIpc(database: WorkspaceDatabase) {
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
  handleIpc("api:request", proxyRustRequest);
}

async function proxyRustRequest(request: IpcRequest<"api:request">) {
  if (!request.path.startsWith("/") || request.path.startsWith("//")) {
    throw new Error("Rust API path must start with one slash");
  }
  const baseUrl = process.env.UNITY_ASSET_RS_URL ?? "http://127.0.0.1:8080";
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
