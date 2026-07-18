import xwbx from "@xwbx/eslint-config";

export default xwbx({
  type: "app",
  vue: true,
  typescript: true,
  unocss: true,
  formatters: true,
  ignores: [
    "dist",
    "node_modules",
    "target",
    "example",
    "crates",
    "docs",
    "public",
  ],
});
