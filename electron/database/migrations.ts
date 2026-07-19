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
  {
    version: 2,
    name: "project sources",
    sql: `
      CREATE TABLE project_sources (
        id INTEGER PRIMARY KEY,
        project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        kind TEXT NOT NULL CHECK (kind IN ('file', 'directory')),
        name TEXT NOT NULL,
        absolute_path TEXT NOT NULL,
        arknights INTEGER NOT NULL DEFAULT 0 CHECK (arknights IN (0, 1)),
        sort_order INTEGER NOT NULL DEFAULT 0,
        UNIQUE(project_id, absolute_path)
      );

      CREATE INDEX project_sources_project_order
      ON project_sources(project_id, sort_order, id);
    `,
  },
  {
    version: 3,
    name: "allow multiple projects per root",
    sql: `
      CREATE TEMP TABLE saved_projects AS SELECT * FROM projects;
      CREATE TEMP TABLE saved_project_bundles AS SELECT * FROM project_bundles;
      CREATE TEMP TABLE saved_workspace_sessions AS SELECT * FROM workspace_sessions;
      CREATE TEMP TABLE saved_project_sources AS SELECT * FROM project_sources;

      DROP TABLE project_bundles;
      DROP TABLE workspace_sessions;
      DROP TABLE project_sources;
      DROP TABLE projects;

      CREATE TABLE projects (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        root_path TEXT NOT NULL,
        last_opened_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
      );

      CREATE INDEX projects_recent ON projects(last_opened_at DESC, id DESC);
      CREATE INDEX projects_root ON projects(root_path);

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

      CREATE TABLE project_sources (
        id INTEGER PRIMARY KEY,
        project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
        kind TEXT NOT NULL CHECK (kind IN ('file', 'directory')),
        name TEXT NOT NULL,
        absolute_path TEXT NOT NULL,
        arknights INTEGER NOT NULL DEFAULT 0 CHECK (arknights IN (0, 1)),
        sort_order INTEGER NOT NULL DEFAULT 0,
        UNIQUE(project_id, absolute_path)
      );

      CREATE INDEX project_sources_project_order
      ON project_sources(project_id, sort_order, id);

      INSERT INTO projects SELECT * FROM saved_projects;
      INSERT INTO project_bundles SELECT * FROM saved_project_bundles;
      INSERT INTO workspace_sessions SELECT * FROM saved_workspace_sessions;
      INSERT INTO project_sources SELECT * FROM saved_project_sources;

      DROP TABLE saved_projects;
      DROP TABLE saved_project_bundles;
      DROP TABLE saved_workspace_sessions;
      DROP TABLE saved_project_sources;
    `,
  },
];
