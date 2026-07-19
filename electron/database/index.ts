import Database from "better-sqlite3";
import { MIGRATIONS } from "./migrations.js";
import type { JsonValue, WorkspaceSnapshot } from "../../shared/ipc.js";

interface WorkspaceRow {
  project_id: number;
  state_json: string;
  updated_at: string;
}

export class WorkspaceDatabase {
  readonly path: string;
  readonly #database: Database.Database;

  constructor(path: string) {
    this.path = path;
    this.#database = new Database(path);
    this.#database.pragma("journal_mode = WAL");
    this.#database.pragma("foreign_keys = ON");
    this.#migrate();
  }

  get version(): number {
    const row = this.#database
      .prepare(
        "SELECT COALESCE(MAX(version), 0) AS version FROM schema_migrations",
      )
      .get() as { version: number };
    return row.version;
  }

  loadWorkspace(rootPath: string): WorkspaceSnapshot | null {
    const row = this.#database
      .prepare(
        `SELECT s.project_id, s.state_json, s.updated_at
         FROM workspace_sessions s
         JOIN projects p ON p.id = s.project_id
         WHERE p.root_path = ?`,
      )
      .get(rootPath) as WorkspaceRow | undefined;
    return row ? this.#snapshot(row) : null;
  }

  saveWorkspace(
    name: string,
    rootPath: string,
    state: JsonValue,
  ): WorkspaceSnapshot {
    const save = this.#database.transaction(() => {
      this.#database
        .prepare(
          `INSERT INTO projects (name, root_path)
           VALUES (?, ?)
           ON CONFLICT(root_path) DO UPDATE SET
             name = excluded.name,
             last_opened_at = CURRENT_TIMESTAMP`,
        )
        .run(name, rootPath);
      const project = this.#database
        .prepare("SELECT id FROM projects WHERE root_path = ?")
        .get(rootPath) as { id: number };
      this.#database
        .prepare(
          `INSERT INTO workspace_sessions (project_id, state_json)
           VALUES (?, ?)
           ON CONFLICT(project_id) DO UPDATE SET
             state_json = excluded.state_json,
             updated_at = CURRENT_TIMESTAMP`,
        )
        .run(project.id, JSON.stringify(state));
      return this.#database
        .prepare(
          `SELECT project_id, state_json, updated_at
           FROM workspace_sessions WHERE project_id = ?`,
        )
        .get(project.id) as WorkspaceRow;
    });
    return this.#snapshot(save());
  }

  close() {
    this.#database.close();
  }

  #migrate() {
    this.#database.exec(`
      CREATE TABLE IF NOT EXISTS schema_migrations (
        version INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );
    `);
    const applied = new Set(
      (
        this.#database
          .prepare("SELECT version FROM schema_migrations")
          .all() as Array<{
          version: number;
        }>
      ).map((row) => row.version),
    );
    for (const migration of MIGRATIONS) {
      if (applied.has(migration.version)) continue;
      this.#database.transaction(() => {
        this.#database.exec(migration.sql);
        this.#database
          .prepare(
            "INSERT INTO schema_migrations (version, name) VALUES (?, ?)",
          )
          .run(migration.version, migration.name);
      })();
    }
  }

  #snapshot(row: WorkspaceRow): WorkspaceSnapshot {
    return {
      projectId: row.project_id,
      state: JSON.parse(row.state_json) as JsonValue,
      updatedAt: row.updated_at,
    };
  }
}
