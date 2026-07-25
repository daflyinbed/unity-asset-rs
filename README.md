# unity-asset-rs

Rust 库 + CLI + Electron/Vue 桌面端，用于解析 Unity AssetBundle（UnityFS 格式）。
长期目标：typed、高性能的 [UnityPy](https://github.com/K0lb3/UnityPy) / AssetStudio 替代，含 Python 绑定。

- **Rust workspace**：`crates/unity-asset`（主库）+ `unity-core`（共享原语）+ `generated`（codegen 产物）+ `codegen`（离线 TPK→Rust）+ `lz4inv`（Arknights LZ4 变体）。
- **CLI**（根 bin `unity-asset-rs`）：`list` / `object` / `export` / `containers` / `serve` / `openapi`，含 `--arknights`、`--no-sprites`、`--only-sprites`。
- **HTTP 服务**：`serve` 子命令用 axum + utoipa 把 list/object/export/container 能力通过 REST 暴露，Scalar UI 挂 `/`，OpenAPI JSON 在 `/openapi.json`。
- **桌面端**：`app/`（Vue 3 + Vite + UnoCSS）+ `electron/`（main/preload）+ `shared/`（IPC contract）。renderer 经类型安全 IPC 请求 main，main 代理常驻 Rust `serve`；REST 类型由 Rust OpenAPI 自动生成。

## 快速开始

```bash
# Rust
cargo build --workspace
cargo run -- example/char_002_amiya.ab list
cargo run -- --arknights example/chararts_char_002_amiya.ab export -o /tmp/out
cargo run -- serve -a 127.0.0.1:8080 -d example/   # HTTP 服务

# 桌面端
pnpm install
pnpm dev          # Vite dev server
pnpm start        # 启动 Electron
```

## 库用法

```rust
use unity_asset::{Loader, Environment};

let report = Loader::builder().path("main.ab").build()?; // 顶层 IO 错误在此返回
let env: &Environment = report.environment();
for d in report.diagnostics() { eprintln!("{d}"); }       // 局部失败（可恢复）
```

## 文档

架构与设计理由见 [`docs/architecture.md`](docs/architecture.md)；各层现状见
`docs/*-layer.md`；架构决策见 [`docs/adr/`](docs/adr/)；桌面端设计见
[`docs/electron-app.md`](docs/electron-app.md)。维护者/agent 约定见 [`AGENTS.md`](AGENTS.md)。

Edition 2024（Rust >= 1.85）。无 CI / pre-commit / rustfmt 配置。
