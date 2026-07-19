# Electron 应用设计与落地计划

> 本文记录桌面端的进程边界、项目/会话模型和分阶段落地计划。界面原型以
> `app/pages/zed.vue` 为准；Rust REST 端点以 `src/server/` 和运行时生成的
> `/openapi.json` 为准。

## 产品结构

桌面端是项目驱动的 Unity 对象工作台，主体保持三行：

1. 顶栏：菜单、项目名、当前 bundle 与 Unity 版本 chip、窗口按钮。
2. 工作区：标签页 + 可停靠对象抽屉；标签页在 Preview 和 Dump 间切换；底部可开日志面板。
3. 状态栏：对象统计与 left/bottom/right 三个面板控制按钮。

抽屉支持表格和 container path 树。树按 `dyn/art/...` 分层并允许折叠；搜索和类型过滤同时作用于两种视图。Preview 使用棋盘格和元数据条；Dump 使用等宽字体、行号和轻量语法着色。

## 进程边界

```text
Vue renderer
    │ 类型安全 IPC（不暴露 Node）
    ▼
Electron preload
    ▼
Electron main
    ├── RustServiceManager ── localhost 随机端口 ── Rust serve
    ├── ApiProxy
    └── WorkspaceDatabase ── SQLite
```

- renderer 不直接访问 Rust 监听地址，不持有 Node/SQLite 权限。
- preload 只暴露共享 contract 中声明的白名单调用。
- main 负责启动 Rust、探活、代理 HTTP、捕获 stdout/stderr 和退出清理。
- Rust 继续只保存解析状态；SQLite 不复制 Unity 对象或解析结果。

### Rust 服务常驻

Electron 生命周期内只启动一个 Rust 服务。项目切换不重启服务：

1. main 将旧项目工作区写入 SQLite。
2. 调用 `DELETE /bundles` 清空 Rust 内存中的全部 Environment。
3. 根据新项目记录逐个 `POST /bundles`。
4. collection/object/container 就绪后恢复标签页。

`serve --root` 仍是安全边界。桌面端启动服务时应使用用户允许打开项目的共同根目录；如果未来需要同时打开互不相干的文件系统根，应把 Rust 的“允许根目录”改为显式动态白名单，而不是让 renderer 传任意绝对路径。

Rust 异常退出时，main 重启同一服务并按当前项目的 bundle 清单恢复内存状态。切换项目期间 UI 进入只读 loading 状态，避免旧请求落到新项目。

## 对象身份

对象的稳定身份是：

```text
(bundle_id, collection_id, path_id)
```

`path_id` 只在 collection 内唯一，正式状态模型不得沿用 mock 的单个 `id`。标签页、当前对象、container 映射和 PNG/Dump 缓存都使用上述复合键。

## SQLite 状态模型

better-sqlite3 只在 main 使用。首版 schema：

- `projects`：项目名、根路径、最近打开时间。
- `project_bundles`：项目内 bundle id、相对路径、Arknights codec 选项和排序。
- `workspace_sessions`：按项目保存版本化 JSON 工作区快照。
- `schema_migrations`：已执行 migration 及时间。

工作区 JSON 包含：

- 打开的标签页及顺序、活动标签、每个标签的 Preview/Dump 模式；
- 抽屉的 left/right/hidden、宽度、table/tree、搜索和类型过滤；
- 日志面板开关和高度；
- 当前 bundle。

界面状态 debounce 保存，并在关闭标签、切换项目和窗口关闭前立即 flush。日志内容不持久化，只在内存保留有限条数。

## OpenAPI 与请求链路

Rust 的 `ApiDoc` 是 REST 类型的唯一事实来源：

1. `cargo run -- openapi -o app/api/openapi.json` 生成 schema。
2. `openapi-typescript` 生成 `app/api/schema.d.ts`。
3. renderer 使用 `openapi-fetch`，但其 `fetch` 实现由 IPC adapter 提供。
4. main 校验 method/path 后转发给本机 Rust 服务，将 JSON 或二进制响应返回 renderer。

生成的 JSON 和 TypeScript 声明应提交，CI/本地检查重新生成后确认无 diff，避免 Rust DTO 已变更但 renderer 仍使用旧类型。

## 类型安全 IPC

共享 contract 位于 `shared/ipc.ts`，只包含结构化克隆可传输的数据，不依赖 Electron、DOM 或 Node 类型。contract 以 channel 映射输入和输出；main 注册 helper、preload bridge 和 renderer 调用端都由同一个映射推导泛型。

错误不依赖 Electron 默认的跨进程 Error 序列化。可预期业务错误使用可判别结果：

```ts
type Result<T> =
  | { ok: true; value: T }
  | { ok: false; error: { code: string; message: string } }
```

## 分阶段计划

### 基础设施

- Rust 增加清空全部 bundle 的端点和离线 OpenAPI 输出命令。
- main 初始化 better-sqlite3 并按顺序执行 migration。
- 建立共享 IPC contract、main handler helper 和 preload bridge。
- 配置 openapi-typescript/openapi-fetch，并通过 IPC adapter 代理请求。

### 项目生命周期

- 增加项目选择、最近项目和 bundle 清单操作。
- Rust 启动、探活、清理、加载和崩溃恢复由一个状态机串行化。
- 恢复 session 时丢失的 bundle/对象保留错误标签，不静默删除。

### 真实工作台

- 用 collections/objects/containers 替换 `app/mock`。
- 接入 Sprite/Texture2D PNG Preview 和 full detail Dump。
- 完成表格、container 树、过滤、停靠、标签排序和快捷键。
- 将 Rust stdout/stderr 与代理错误送入可调整高度的日志面板。

### 后端展示能力补全

当前 API 尚未直接提供对象字节大小、图片尺寸和统一结构化 typed dump。首版不从 Debug 字符串猜这些字段；后续直接扩展 server DTO，再由 OpenAPI 生成链同步到前端。
