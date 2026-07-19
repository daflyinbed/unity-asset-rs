import { join } from "node:path";
import process from "node:process";
import { app, BrowserWindow } from "electron";
import { WorkspaceDatabase } from "./database/index.js";
import { registerIpc } from "./ipc.js";

let database: WorkspaceDatabase | null = null;

async function createWindow() {
  const window = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 960,
    minHeight: 640,
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
  database = new WorkspaceDatabase(
    join(app.getPath("userData"), "workspace.sqlite3"),
  );
  registerIpc(database);
  await createWindow();

  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) void createWindow();
  });
});

app.on("before-quit", () => {
  database?.close();
  database = null;
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") app.quit();
});
