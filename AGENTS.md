# AGENTS.md

## What this repo is

A Rust library + CLI for parsing Unity AssetBundles (UnityFS format)。长期目标：typed、高性能的 UnityPy/AssetStudio 替代，含 Python 绑定。**现为混合仓库**：Rust workspace（`src/` + `crates/`）+ Electron/Vue 3 桌面端（`electron/` + `app/`，pnpm/Vite）。renderer 通过类型安全 IPC 请求 main，main 代理 CLI `serve` 暴露的 REST；REST 类型由 Rust OpenAPI 自动生成。

架构细节、设计理由、非显然约定见 [`docs/architecture.md`](docs/architecture.md);各层现状见 `docs/*-layer.md`。

## Workspace 结构（5 crate + Electron/Vue）

Rust 依赖 `unity-core ← generated ← unity-asset`；`codegen` / `lz4inv` 独立。各 Rust crate 有自己的 `AGENTS.md`，改某 crate 前先读对应文件。

<!-- prettier-ignore -->
| crate / 目录                      | 职责                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/`（根 bin `unity-asset-rs`） | io + 对象解析 + 纹理/Sprite 导出 + bin（`main.rs`：`list`/`object`/`export`/`serve`/`openapi`，含 `--no-sprites`/`--only-sprites`/`--arknights`）。根 `lib.rs` 分 `pub mod io`/`typetree`/`types`/`export` 门面 + 顶层直转推荐入口(`Loader`/`Environment`/`CollectionId`/`ClassId`/`DecodedImage`)。`serve` 子命令在 `src/server/` 下,用 axum 0.8 把 list/object/export 能力通过 REST 暴露 + utoipa 自动生成 OpenApi 3.1(Scalar UI 挂 `/`,JSON 在 `/openapi.json`)。`main.rs` `#[tokio::main]` async;阻塞 IO/CPU 走 `spawn_blocking`;bundle 运行时按请求加载,`Arc<RwLock<HashMap<id, Arc<Environment>>>>` 共享;`DELETE /bundles` 清空全部内存状态供 Electron 切项目复用常驻服务;路径参数用 `{bid}` 语法(axum 0.8 + utoipa 一致);DTO 手写 + `#[derive(ToSchema)]`,错误响应 `ApiErrorBody` 带 schema,零 serde 侵入库 |
| `crates/unity-asset`              | 主库：解析 + Environment + 导出（`lib.rs` 门面 + `tests/`）                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `crates/unity-core`               | 共享原语（零依赖）：`UnityVersion`/`Endian`/`EndianReader`/`Parse`/`PPtr`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `crates/codegen`                  | 离线工具：TPK→Rust，只 emit 引用，不 link                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `crates/generated`                | codegen 产物：`UnityObject` enum + 扁平 struct + `ClassIDType`（勿手改，codegen 重生）                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `crates/lz4inv`                   | Arknights LZ4 变体(逆序)解压——供 CLI `AkLz4Decompressor` 用,不进库                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `app/`（前端，pnpm）              | Vue 3 + Vite + UnoCSS + vue-router 文件路由。入口 `index.html`→`app/main.ts`→`app/App.vue`（仅 `<RouterView/>`）。页面在 `app/pages/`，组件在 `app/components/`，路由实例在 `app/router/index.ts`。详见下方「前端」节                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `electron/`                      | Electron main/preload。main 初始化 better-sqlite3、执行 `electron/database/migrations.ts` 顺序 migration、注册 IPC 并代理 Rust HTTP；preload 只暴露共享 contract 白名单。better-sqlite3 是原生模块，安装后须按 Electron ABI rebuild                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `shared/`                        | main/preload/renderer 共用的纯类型与 IPC contract；不得依赖 Electron、Node 或 DOM。`shared/ipc.ts` 的 channel 映射是 IPC 输入/输出唯一事实来源                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `app/api/`                       | Rust OpenAPI 生成物与 typed client：`openapi.json` + `schema.d.ts` 自动生成（需提交、勿手改），`client.ts` 用 openapi-fetch + IPC fetch adapter，经 main 转发请求                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

## Commands

### Rust

```bash
cargo build --workspace           # 全部 crate
cargo test                        # 全部测试
cargo test --test parse_bundle    # io 层
cargo test --test environment     # 聚合层
cargo test --test texture_export  # 纹理导出（export feature）
cargo test sprite_export          # Sprite（字节级单测在 src/sprite_export.rs）
cargo run                         # 遍历 example/，对象 + 解析状态 + 汇总（失败退出码 1）
cargo run -- --arknights example/chararts/char_002_amiya.ab export -o /tmp/out
cargo run -- serve -a 127.0.0.1:8080 -d example/   # axum HTTP 服务(POST /bundles 加载,GET .../objects/{pid}/png 导出 PNG,Scalar UI 在 /,OpenApi JSON 在 /openapi.json)
cargo run -- openapi -o app/api/openapi.json       # 离线输出与运行时 /openapi.json 相同的 schema
cargo test --bin unity-asset-rs server::           # serve 子命令单元测试(含 OpenApi/Scalar 可达性,oneshot 打 router)
# 重新生成 typed structs（改完 codegen 后必跑）：
rm -rf crates/generated/src && mkdir -p crates/generated/src
cargo run -p unity-asset-codegen -- crates/codegen/tests/fixtures/type_tree.tpk crates/generated/src
cargo clippy --all-targets
```

Edition `2024` → Rust >= 1.85。Rust 侧无 CI / pre-commit / rustfmt.toml。

### 前端（`app/`，pnpm）

```bash
pnpm install        # 装依赖
pnpm dev            # Vite dev server（HMR，文件路由自动注册）
pnpm build          # vue-tsc + renderer/main/preload production build
pnpm start          # 启动已构建的 Electron 应用
pnpm preview        # 预览构建产物
pnpm api:generate   # Rust OpenAPI JSON → app/api/schema.d.ts（改 server DTO/route 后必跑）
pnpm native:rebuild # 按当前 Electron ABI 重编 better-sqlite3
pnpm lint           # eslint .（只扫前端，见下）
pnpm lint:fix       # eslint . --fix
```

## 桌面端（Electron + Vue 3）

详细设计与后续阶段见 [`docs/electron-app.md`](docs/electron-app.md)。Electron 生命周期内保持一个 Rust 服务；切换项目时先 `DELETE /bundles` 清空内存，再按 SQLite 项目记录重新加载，不因普通项目切换重启 Rust。

- **进程边界**：renderer 不直连 localhost、不接触 Node/SQLite；`app/api/client.ts` 的 openapi-fetch 通过 IPC adapter 请求 main，main 再转发 Rust HTTP。开发时代理地址可由 `UNITY_ASSET_RS_URL` 指定，默认 `http://127.0.0.1:8080`。
- **IPC**：新增/修改 channel 先改 `shared/ipc.ts` 的 `IpcContract` 与 `IPC_CHANNELS`，再在 `electron/ipc.ts` 注册 handler；只传 structured-clone 可传输值。业务错误走 `IpcResult<T>`，不要依赖 Electron 默认 Error 序列化。
- **SQLite**：better-sqlite3 仅允许在 main 使用。schema 变更以新版本追加到 `electron/database/migrations.ts`，已发布 migration 不回写；migration 必须事务执行。SQLite 只持久化项目/bundle 配置与 workspace JSON，不保存 Rust 解析对象或日志。
- **OpenAPI**：`ApiDoc`/server routes 是 REST 类型唯一事实来源。改 route、DTO 或 `ToSchema` 后运行 `pnpm api:generate`，提交 `app/api/openapi.json` 和 `app/api/schema.d.ts`；两者均为生成物，勿手改。
- **原生依赖**：pnpm 10 通过 `pnpm-workspace.yaml` 的 `allowBuilds` 允许 better-sqlite3 install script；`postinstall` 会跑 Electron rebuild。Electron 大版本变化或 ABI 报错时手动跑 `pnpm native:rebuild`。

### Renderer（`app/`）

- **文件路由**：`vite.config.ts` 用 `VueRouter({ routesFolder: "app/pages" })`，页面即 `app/pages/*.vue`（当前 `index.vue`→`/`、`zed.vue`→`/zed`，根页只是 Zed 工作台入口）。`app/router/index.ts` 用 `vue-router` v5 的 `vue-router/auto-routes` 实验性 API + `handleHotUpdate`。**改路由 = 增删 `app/pages/` 下文件**，不要手写路由表。
- **`typed-router.d.ts` 是自动生成的**（`@ts-nocheck`、标 `DO NOT MODIFY`、需提交）。改页面文件后 `pnpm dev`/`pnpm build` 会重生；勿手改。
- **UnoCSS**（`uno.config.ts`）：语义色组 `accent`/`neutral`（各 1..10 + `contrast`），全部指向 CSS 变量 `--my-color-{group}-{n}`（值定义在 `app/style.css` `:root`，前端唯一主题源头）。**动态拼接的类名（如 `bg-accent-5`、`text-neutral-10`）必须在 `SAFELIST` 里登记**，否则 UnoCSS 扫描器漏检类不生效——新增动态类模式时记得扩 `SAFELIST`。另有独立背景色 `background`/`panel`/`surface`/`overlay`。
- **图标**：`unplugin-icons`（`compiler: "vue3"`、`defaultClass: "icon"`、`autoInstall: false`）+ `@iconify-json/lucide`。用法 `<i-lucide-xxx />`，类型由 `app/icons.d.ts`（`/// <reference types="unplugin-icons/types/vue3" />`）提供。要新图标集需装对应 `@iconify-json/*`。
- **ESLint**：`eslint.config.ts` 用 `@xwbx/eslint-config`（`type: "app"`，开 vue/typescript/unocss/formatters）。扫描 `app/`、`electron/`、`shared/` 与前端配置；Rust/生成的 OpenAPI 文件等在 `ignores` 里。Rust 走 `cargo clippy`，别用 ESLint 之外的 prettier 单独跑（formatter 已并入 eslint 配置）。
- **TS**：`tsconfig.app.json` 管 `app/**` + `shared/**` + 自动生成声明；`tsconfig.node.json` 管 `electron/**` + `shared/**` + Vite 配置。严格项 `noUnusedLocals`/`noUnusedParameters`/`erasableSyntaxOnly`/`noFallthroughCasesInSwitch` 全开。
- 约定：SFC 用 `<script setup lang="ts">`；属性传递走 `defineProps<{...}>()`（参考 `app/components/Swatch.vue`）；颜色优先用语义类（`bg-accent-6` 等），不要硬编码十六进制（主题色统一在 `app/style.css` 的 CSS 变量里）。

## ⚠️ 硬性规则（改代码前必读）

**Rust**

- **不在库里硬编码游戏特定 codec**——走 `Registry` 扩展点（`src/decompress.rs`），如 Arknights LZ4 变体放 CLI 层 `AkLz4Decompressor`。
- **大端 cursor 不要换 `byteorder`**——`src/bundle.rs` 内联 cursor 是有意的。
- **`Cursor` 是 crate-private**(`mod cursor`,非 `pub`)——seek/align/remaining 已做边界检查(越界返回 `Truncated` 错误)。新增功能不要重新 pub Cursor;用户面走 `EndianReader` trait 与高级解析入口(`SerializedFile::parse`/`ObjectInfo::parse`)。
- **勿降 `opt-level=1`**（根 `Cargo.toml` `[profile.dev]`/`[profile.test]`）——降回 0 会在深嵌套 typetree 下溢出 2MB 测试线程栈。
- **不要删/改 fixture** `example/char_002_amiya.ab` / `example/DownloadCommonUI.unity3d`。期望字节值（`EXPECTED_FIRST16` 等）是 **bit-for-bit 对照 UnityPy** 的，改=改 parser 语义。
- **代码为准**——`Registry`/`Decompressor` vs ADR-0003 的 `DecompressorRegistry` 命名漂移，不迁就文档改名。
- **拿不准格式/语义**：先看 `docs/`，再 grep 父目录参考实现（尤其 AssetRipper），不要自创逻辑。
- 公共 API 刻意精简（`src/lib.rs`），export 遵循既有范围，勿随意扩 `pub`。
- **改了 codegen 必须重生 `crates/generated/`**（见 Commands），不要手改生成产物，也不要对生成目录运行 `cargo fmt --all` 制造全量格式化 diff。

**桌面端**

- 不要手改 `typed-router.d.ts`（自动生成）。
- 不要手改 `app/api/openapi.json` / `app/api/schema.d.ts`；改 Rust schema 后跑 `pnpm api:generate`。
- renderer 禁止直接 import Node/Electron/better-sqlite3 或直连 Rust localhost；走 preload IPC。
- IPC channel 必须先登记在 `shared/ipc.ts`，preload 不得暴露任意 `ipcRenderer.send/invoke`。
- migration 只追加不修改；数据库访问只放 Electron main。
- 新增动态拼接的 UnoCSS 类名必须同步登记到 `uno.config.ts` 的 `SAFELIST`。
- 主题色统一在 `app/style.css` 的 CSS 变量里，组件用语义类引用，勿散落硬编码颜色。
- ESLint 扫 `app/`、`electron/`、`shared/` 与前端配置；Rust 目录勿加 ESLint 例外或 prettier 配置。

## 参考实现 & 跨项目文档

父目录 `/home/xwbx/Documents/unity/` 是多项目 workspace（非 git repo），含参考实现 + 跨项目文档。拿不准时优先 grep。

- **[`../AGENTS.md`](../AGENTS.md)** — workspace 级指引（PPtr/`.resS` 跨项目语义差异）。**做跨项目对照前先读。**
- **[`../docs/`](../docs/)** — 跨项目分析（中文）：`ress-file-handling.md` / `tpk-class-parser-generation.md` / `assetripper-monobehaviour-handling.md` / `astc-hdr-alpha-handling.md`。

参考实现（多数只读 upstream 镜像，**改动前先与用户确认**；git 操作定向到具体子目录）：

| 目录                            | 角色                                                             |
| ------------------------------- | ---------------------------------------------------------------- |
| `../AssetRipper/` (C# .NET 10)  | 全功能 importer/exporter；本仓库 PPtr/`.resS`/命名**刻意对齐**它 |
| `../UnityPy/` (Python)          | byte-level parity 默认对照目标                                   |
| `../AssetStudio/` (C#)          | UnityPy 原型，读取逻辑可参考                                     |
| `../Tpk/` + `../TypeTreeDumps/` | TPK 库 + 全 Unity 版本 struct dump                               |

**本仓库 `docs/`（中文，权威现状，冲突时以它们为准）**：
[`architecture.md`](docs/architecture.md)（架构+约定） /
[`io-layer.md`](docs/io-layer.md) /
[`object-layer.md`](docs/object-layer.md) /
[`environment-layer.md`](docs/environment-layer.md) /
[`codegen.md`](docs/codegen.md) /
[`memory-notes.md`](docs/memory-notes.md) /
[`electron-app.md`](docs/electron-app.md) /
[`adr/`](docs/adr/)（0001 codegen / 0002 pyo3 / 0003 io 层）。
