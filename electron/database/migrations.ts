export interface Migration {
  version: number;
  name: string;
  sql: string;
}

export const MIGRATIONS: readonly Migration[] = [
  {
    version: 1,
    name: "initial workspace schema",
    sql: `
      CREATE TABLE projects (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        root_path TEXT NOT NULL UNIQUE,
        last_opened_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE TABLE project_bundles (
        id INTEGER PRIMARY KEY,
        project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        bundle_id TEXT NOT NULL,
        relative_path TEXT NOT NULL,
        arknights INTEGER NOT NULL DEFAULT 0 CHECK (arknights IN (0, 1)),
        sort_order INTEGER NOT NULL DEFAULT 0,
        UNIQUE(project_id, bundle_id),
        UNIQUE(project_id, relative_path)
      );

      CREATE TABLE workspace_sessions (
        project_id INTEGER PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
        state_json TEXT NOT NULL,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );
    `,
  },
];
