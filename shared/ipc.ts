export type JsonValue =
  null | boolean | number | string | JsonValue[] | { [key: string]: JsonValue };

export type IpcResult<T> =
  | { ok: true; value: T }
  | { ok: false; error: { code: string; message: string } };

export interface ApiProxyRequest {
  method: string;
  path: string;
  headers: Record<string, string>;
  body?: string;
}

export interface ApiProxyResponse {
  status: number;
  headers: Record<string, string>;
  body: ArrayBuffer;
}

export interface WorkspaceSnapshot {
  projectId: number;
  state: JsonValue;
  updatedAt: string;
}

export type WindowControlAction =
  "minimize" | "toggle-maximize" | "open-devtools" | "close";

export interface SelectedPath {
  arknights: boolean;
  kind: "file" | "directory";
  name: string;
  path: string;
}

export interface ProjectSource {
  id: number;
  projectId: number;
  kind: "file" | "directory";
  name: string;
  path: string;
  arknights: boolean;
  sortOrder: number;
}

export interface ProjectSnapshot {
  id: number;
  name: string;
  rootPath: string;
  sources: ProjectSource[];
  lastOpenedAt: string;
}

export interface IpcContract {
  "app:info": {
    request: Record<string, never>;
    response: {
      platform: string;
      versions: { electron: string; chrome: string; node: string };
    };
  };
  "api:request": { request: ApiProxyRequest; response: ApiProxyResponse };
  "database:status": {
    request: Record<string, never>;
    response: { path: string; version: number };
  };
  "workspace:load": {
    request: { rootPath: string };
    response: WorkspaceSnapshot | null;
  };
  "workspace:save": {
    request: { name: string; rootPath: string; state: JsonValue };
    response: WorkspaceSnapshot;
  };
  "window:control": {
    request: { action: WindowControlAction };
    response: { maximized: boolean };
  };
  "files:pick": {
    request: Record<string, never>;
    response: SelectedPath[];
  };
  "files:inspect": {
    request: { paths: string[] };
    response: SelectedPath[];
  };
  "projects:list": {
    request: Record<string, never>;
    response: ProjectSnapshot[];
  };
  "projects:create": {
    request: { name: string; sources: SelectedPath[] };
    response: ProjectSnapshot;
  };
  "projects:open": {
    request: { projectId: number };
    response: ProjectSnapshot;
  };
  "projects:add-sources": {
    request: { projectId: number; sources: SelectedPath[] };
    response: ProjectSnapshot;
  };
  "projects:remove-source": {
    request: { projectId: number; sourceId: number };
    response: ProjectSnapshot;
  };
  "projects:refresh-source": {
    request: { projectId: number; sourceId: number };
    response: ProjectSnapshot;
  };
}

export type IpcChannel = keyof IpcContract;
export type IpcRequest<C extends IpcChannel> = IpcContract[C]["request"];
export type IpcResponse<C extends IpcChannel> = IpcContract[C]["response"];

export interface ElectronBridge {
  invoke: <C extends IpcChannel>(
    channel: C,
    request: IpcRequest<C>,
  ) => Promise<IpcResult<IpcResponse<C>>>;
  getPathForFile: (file: unknown) => string;
}

export const IPC_CHANNELS = [
  "app:info",
  "api:request",
  "database:status",
  "workspace:load",
  "workspace:save",
  "window:control",
  "files:pick",
  "files:inspect",
  "projects:list",
  "projects:create",
  "projects:open",
  "projects:add-sources",
  "projects:remove-source",
  "projects:refresh-source",
] as const satisfies readonly IpcChannel[];
