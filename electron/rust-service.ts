import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { createServer } from "node:net";
import { join, parse } from "node:path";
import process from "node:process";
import type { Buffer } from "node:buffer";
import type { ChildProcessWithoutNullStreams } from "node:child_process";

export class RustService {
  #child: ChildProcessWithoutNullStreams | null = null;
  #baseUrl = "";
  #startError: Error | null = null;

  get baseUrl() {
    if (!this.#baseUrl) throw new Error("Rust service is not ready");
    return this.#baseUrl;
  }

  async start() {
    if (this.#child) return;
    const port = await availablePort();
    const address = `127.0.0.1:${port}`;
    const configuredBinary = process.env.UNITY_ASSET_RS_BIN;
    const debugBinary = join(
      process.cwd(),
      "target",
      "debug",
      process.platform === "win32" ? "unity-asset-rs.exe" : "unity-asset-rs",
    );
    const binary = configuredBinary ?? debugBinary;
    const root = parse(process.cwd()).root;
    const command =
      configuredBinary || existsSync(debugBinary) ? binary : "cargo";
    const args =
      command === "cargo"
        ? ["run", "--quiet", "--", "serve", "-a", address, "-d", root]
        : ["serve", "-a", address, "-d", root];

    this.#startError = null;
    this.#child = spawn(command, args, {
      cwd: process.cwd(),
      env: process.env,
      stdio: "pipe",
    });
    this.#baseUrl = `http://${address}`;
    this.#child.stdout.on("data", (chunk: Buffer) =>
      process.stdout.write(`[rust] ${chunk.toString()}`),
    );
    this.#child.stderr.on("data", (chunk: Buffer) =>
      process.stderr.write(`[rust] ${chunk.toString()}`),
    );
    this.#child.once("error", (error) => {
      this.#startError = error;
      this.#child = null;
      this.#baseUrl = "";
    });
    this.#child.once("exit", () => {
      this.#child = null;
      this.#baseUrl = "";
    });

    for (let attempt = 0; attempt < 100; attempt++) {
      if (this.#startError) throw this.#startError;
      if (!this.#child) throw new Error("Rust service exited during startup");
      try {
        const response = await fetch(`${this.#baseUrl}/health`);
        if (response.ok) return;
      } catch {
        // The listener is not ready yet.
      }
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
    this.stop();
    throw new Error("Timed out waiting for the Rust service");
  }

  stop() {
    this.#child?.kill();
    this.#child = null;
    this.#baseUrl = "";
  }
}

async function availablePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const server = createServer();
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      if (!address || typeof address === "string") {
        server.close();
        reject(new Error("Could not allocate a local port"));
        return;
      }
      server.close((error) => (error ? reject(error) : resolve(address.port)));
    });
  });
}
