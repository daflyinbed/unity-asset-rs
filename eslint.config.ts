import xwbx from "@xwbx/eslint-config";

export default xwbx({
  type: "app",
  vue: true,
  typescript: true,
  unocss: true,
  formatters: true,
  ignores: [
    "dist",
    "dist-electron",
    ".codex",
    "node_modules",
    "target",
    "example",
    "crates",
    "docs",
    "public",
    "opencode.jsonc",
    "app/api/openapi.json",
    "app/api/schema.d.ts",
  ],
});
