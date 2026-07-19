import { join } from "node:path";
import process from "node:process";
import { app, BrowserWindow, Menu } from "electron";
import { WorkspaceDatabase } from "./database/index.js";
import { registerIpc } from "./ipc.js";
import { RustService } from "./rust-service.js";

let database: WorkspaceDatabase | null = null;
const rustService = new RustService();

async function createWindow() {
  const window = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 960,
    minHeight: 640,
    autoHideMenuBar: true,
    frame: false,
    show: false,
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      preload: join(import.meta.dirname, "preload.cjs"),
      sandbox: true,
    },
  });

  window.once("ready-to-show", () => window.show());

  if (process.env.VITE_DEV_SERVER_URL) {
    await window.loadURL(process.env.VITE_DEV_SERVER_URL);
  } else {
    await window.loadFile(join(import.meta.dirname, "../dist/index.html"));
  }
}

app.whenReady().then(async () => {
  Menu.setApplicationMenu(null);
  database = new WorkspaceDatabase(
    join(app.getPath("userData"), "workspace.sqlite3"),
  );
  await rustService.start();
  registerIpc(database, rustService);
  await createWindow();

  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) void createWindow();
  });
});

app.on("before-quit", () => {
  rustService.stop();
  database?.close();
  database = null;
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") app.quit();
});
