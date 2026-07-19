import type { ElectronBridge } from "../shared/ipc";

export {};

declare global {
  interface Window {
    electron: ElectronBridge;
  }
}
