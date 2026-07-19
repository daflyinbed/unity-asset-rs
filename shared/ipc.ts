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
}

export type IpcChannel = keyof IpcContract;
export type IpcRequest<C extends IpcChannel> = IpcContract[C]["request"];
export type IpcResponse<C extends IpcChannel> = IpcContract[C]["response"];

export interface ElectronBridge {
  invoke: <C extends IpcChannel>(
    channel: C,
    request: IpcRequest<C>,
  ) => Promise<IpcResult<IpcResponse<C>>>;
}

export const IPC_CHANNELS = [
  "app:info",
  "api:request",
  "database:status",
  "workspace:load",
  "workspace:save",
] as const satisfies readonly IpcChannel[];
