import Database from "better-sqlite3";
import { MIGRATIONS } from "./migrations.js";
import type {
  JsonValue,
  ProjectSnapshot,
  ProjectSource,
  SelectedPath,
  WorkspaceSnapshot,
} from "../../shared/ipc.js";

interface WorkspaceRow {
  project_id: number;
  state_json: string;
  updated_at: string;
}

interface ProjectRow {
  id: number;
  name: string;
  root_path: string;
  last_opened_at: string;
}

interface SourceRow {
  id: number;
  project_id: number;
  kind: "file" | "directory";
  name: string;
  absolute_path: string;
  arknights: number;
  sort_order: number;
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
         WHERE p.root_path = ?
         ORDER BY p.last_opened_at DESC, p.id DESC
         LIMIT 1`,
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
      let project = this.#database
        .prepare(
          `SELECT id FROM projects WHERE root_path = ?
           ORDER BY last_opened_at DESC, id DESC LIMIT 1`,
        )
        .get(rootPath) as { id: number } | undefined;
      if (project) {
        this.#database
          .prepare(
            `UPDATE projects SET name = ?, last_opened_at = CURRENT_TIMESTAMP
             WHERE id = ?`,
          )
          .run(name, project.id);
      } else {
        const inserted = this.#database
          .prepare("INSERT INTO projects (name, root_path) VALUES (?, ?)")
          .run(name, rootPath);
        project = { id: Number(inserted.lastInsertRowid) };
      }
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

  listProjects(): ProjectSnapshot[] {
    const rows = this.#database
      .prepare(
        `SELECT id, name, root_path, last_opened_at
         FROM projects ORDER BY last_opened_at DESC, id DESC`,
      )
      .all() as ProjectRow[];
    return rows.map((row) => this.#projectSnapshot(row));
  }

  getProject(projectId: number): ProjectSnapshot | null {
    const row = this.#database
      .prepare(
        `SELECT id, name, root_path, last_opened_at
         FROM projects WHERE id = ?`,
      )
      .get(projectId) as ProjectRow | undefined;
    return row ? this.#projectSnapshot(row) : null;
  }

  createProject(
    name: string,
    rootPath: string,
    sources: SelectedPath[],
  ): ProjectSnapshot {
    const create = this.#database.transaction(() => {
      const inserted = this.#database
        .prepare("INSERT INTO projects (name, root_path) VALUES (?, ?)")
        .run(name, rootPath);
      const project = { id: Number(inserted.lastInsertRowid) };
      this.#insertSources(project.id, sources);
      return project.id;
    });
    const project = this.getProject(create());
    if (!project) throw new Error("Project was not persisted");
    return project;
  }

  addSources(projectId: number, sources: SelectedPath[]): ProjectSnapshot {
    this.#database.transaction(() => {
      this.#assertProject(projectId);
      this.#insertSources(projectId, sources);
      this.#touchProject(projectId);
    })();
    const project = this.getProject(projectId);
    if (!project) throw new Error(`Project ${projectId} does not exist`);
    return project;
  }

  removeSource(projectId: number, sourceId: number): ProjectSnapshot {
    const changed = this.#database
      .prepare("DELETE FROM project_sources WHERE id = ? AND project_id = ?")
      .run(sourceId, projectId).changes;
    if (!changed) throw new Error(`Source ${sourceId} does not exist`);
    this.#touchProject(projectId);
    const project = this.getProject(projectId);
    if (!project) throw new Error(`Project ${projectId} does not exist`);
    return project;
  }

  touchProject(projectId: number): ProjectSnapshot {
    this.#assertProject(projectId);
    this.#touchProject(projectId);
    const project = this.getProject(projectId);
    if (!project) throw new Error(`Project ${projectId} does not exist`);
    return project;
  }

  deleteProject(projectId: number) {
    this.#database.prepare("DELETE FROM projects WHERE id = ?").run(projectId);
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

  #projectSnapshot(row: ProjectRow): ProjectSnapshot {
    const sources = this.#database
      .prepare(
        `SELECT id, project_id, kind, name, absolute_path, arknights, sort_order
         FROM project_sources WHERE project_id = ? ORDER BY sort_order, id`,
      )
      .all(row.id) as SourceRow[];
    return {
      id: row.id,
      name: row.name,
      rootPath: row.root_path,
      lastOpenedAt: row.last_opened_at,
      sources: sources.map((source): ProjectSource => ({
        id: source.id,
        projectId: source.project_id,
        kind: source.kind,
        name: source.name,
        path: source.absolute_path,
        arknights: source.arknights === 1,
        sortOrder: source.sort_order,
      })),
    };
  }

  #insertSources(projectId: number, sources: SelectedPath[]) {
    const next = this.#database
      .prepare(
        "SELECT COALESCE(MAX(sort_order), -1) + 1 AS value FROM project_sources WHERE project_id = ?",
      )
      .get(projectId) as { value: number };
    const insert = this.#database.prepare(
      `INSERT INTO project_sources
       (project_id, kind, name, absolute_path, arknights, sort_order)
       VALUES (?, ?, ?, ?, ?, ?)
       ON CONFLICT(project_id, absolute_path) DO NOTHING`,
    );
    for (const [index, source] of sources.entries())
      insert.run(
        projectId,
        source.kind,
        source.name,
        source.path,
        source.arknights ? 1 : 0,
        next.value + index,
      );
  }

  #assertProject(projectId: number) {
    const found = this.#database
      .prepare("SELECT 1 FROM projects WHERE id = ?")
      .get(projectId);
    if (!found) throw new Error(`Project ${projectId} does not exist`);
  }

  #touchProject(projectId: number) {
    this.#database
      .prepare(
        "UPDATE projects SET last_opened_at = CURRENT_TIMESTAMP WHERE id = ?",
      )
      .run(projectId);
  }
}
