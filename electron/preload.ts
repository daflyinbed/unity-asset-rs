import { contextBridge, ipcRenderer } from "electron";
import { IPC_CHANNELS } from "../shared/ipc.js";
import type { ElectronBridge, IpcChannel, IpcRequest } from "../shared/ipc.js";

const allowed = new Set<IpcChannel>(IPC_CHANNELS);

const bridge: ElectronBridge = {
  invoke(channel, request) {
    if (!allowed.has(channel))
      throw new Error(`IPC channel is not allowed: ${channel}`);
    return ipcRenderer.invoke(channel, request as IpcRequest<typeof channel>);
  },
};

contextBridge.exposeInMainWorld("electron", bridge);
