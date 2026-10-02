# Electron 应用设计与落地计划

> 本文记录桌面端的进程边界、项目/会话模型和分阶段落地计划。界面原型以
> `app/pages/index.vue` 为准；Rust REST 端点以 `src/server/` 和运行时生成的
> `/openapi.json` 为准。

## 产品结构

桌面端是项目驱动的 Unity 对象工作台，主体保持三行：

1. 顶栏：菜单、项目名、source 数量与 Unity 版本 chip、窗口按钮。
2. 工作区：标签页 + 可停靠对象抽屉；标签页在 Preview 和 Dump 间切换；底部可开日志面板。
3. 状态栏：对象统计与 left/bottom/right 三个面板控制按钮。

抽屉支持表格和 container path 树。树按 `dyn/art/...` 分层并允许折叠；搜索和类型过滤同时作用于两种视图。Preview 使用棋盘格和元数据条；Dump 使用等宽字体、行号和轻量语法着色。

## 主题颜色语义

主题的唯一值来源是 `app/style.css` 中的 `--my-color-*` CSS 变量，组件通过
`uno.config.ts` 暴露的语义类使用它们。不要把编号当作任意的明暗色阶；编号表示组件角色和交互状态。

独立背景色的职责如下：

| 变量 | 用途 |
| --- | --- |
| `--my-color-background` | 应用主背景 |
| `--my-color-panel` | 卡片、表格、抽屉、菜单、popover、dialog 等容器背景 |
| `--my-color-surface` | 输入框、select、checkbox 等表单控件背景 |
| `--my-color-overlay` | Dialog 等模态层的遮罩 |

桌面工作区还保留 `titlebar`、`statusbar`、`tabbar` 和 `drawer` 四个专用层级色。它们用于区分固定的
工作区 chrome 区域，不替代菜单、卡片、popover 等通用 `panel` 语义。

`accent`、`neutral` 和 `danger` 每组都有 `1..10 + contrast`。Accent 用于品牌和主要操作，Neutral
用于无强调的界面结构，Danger 只用于错误及危险操作。每组编号的固定含义是：

| Variant | 用途 |
| --- | --- |
| `1` | outline/ghost hover 背景 |
| `2` | outline/ghost active 背景；surface 或 neutral disabled 背景 |
| `3` | surface hover 背景 |
| `4` | surface active 背景 |
| `5` | outline/ghost 默认边框 |
| `6` | outline/ghost hover 边框；neutral disabled 边框 |
| `7` | solid 默认背景；neutral disabled 文字 |
| `8` | solid hover 背景 |
| `9` | solid active 背景 |
| `10` | 普通文字和图标颜色 |
| `contrast` | solid 背景上的文字和图标颜色 |

弱化文字仍使用 `*-10`，通过透明度表达层级，例如 `text-neutral-10/60`；不要改用较小编号。
普通边框使用 `*-5`，hover 边框使用 `*-6`。Ghost 控件按 `*-1` hover、`*-2` active；solid
控件按 `*-7` 默认、`*-8` hover、`*-9` active，并使用 `*-contrast` 作为前景色。

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

1. main 从 SQLite 读取项目的 `project_sources`。
2. 用一次 `PUT /environment` 原子构建并替换 Rust 中唯一的活动 Environment。
3. renderer 按响应 revision 拉取 collection/object/container 全量快照。
4. source 新增、删除、刷新分别走 `/environment/sources` 的增量端点；旧 source 不重新解析。
5. collection/object/container 就绪后，renderer 按新句柄协调仍存在的标签页。

`serve --root` 仍是安全边界。桌面端启动服务时应使用用户允许打开项目的共同根目录；如果未来需要同时打开互不相干的文件系统根，应把 Rust 的“允许根目录”改为显式动态白名单，而不是让 renderer 传任意绝对路径。

Rust 异常退出时，main 重启同一服务并按当前项目的 source 清单恢复内存状态。切换项目期间 UI 进入只读 loading 状态，避免旧请求落到新项目。

## 对象身份

对象的稳定身份是：

```text
(environment_revision, collection_id, path_id)
```

`path_id` 只在 collection 内唯一。source 变更后 collection id 允许重排，因此旧 revision
下的句柄不得用于新 Environment；REST 在带过期 `revision` 的查询上返回 409。

## SQLite 状态模型

better-sqlite3 只在 main 使用。首版 schema：

- `projects`：项目名、根路径、最近打开时间。
- `project_sources`：项目内文件/目录的绝对路径、类型、Arknights codec 选项和排序。
- `project_bundles`：migration 1 的兼容表；新项目逻辑不再写入。
- `workspace_sessions`：按项目保存版本化 JSON 工作区快照。
- `schema_migrations`：已执行 migration 及时间。

工作区 JSON 包含：

- 打开的标签页及顺序、活动标签、每个标签的 Preview/Dump 模式；
- 抽屉的 left/right/hidden、宽度、table/tree、搜索和类型过滤；
- 日志面板开关和高度；
- 当前 source / collection。

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

### 已落地基础设施

- Rust 提供单一 `/environment`、增量 source mutation、revision 防陈旧查询和离线 OpenAPI。
- main 初始化 better-sqlite3 并按顺序执行 migration。
- 建立共享 IPC contract、main handler helper 和 preload bridge。
- 配置 openapi-typescript/openapi-fetch，并通过 IPC adapter 代理请求。

### 已落地项目生命周期

- 增加新建项目、最近项目和文件/目录 source 清单操作。
- project 只存在于 Electron/SQLite；Rust 只接收不透明 source id。
- project 新建/追加失败时 main 执行 SQLite/Rust 补偿，避免留下半更新状态。
- Rust 启动、探活、清理、加载和崩溃恢复由一个状态机串行化。
- 恢复 session 时丢失的 bundle/对象保留错误标签，不静默删除。

### 真实工作台

- 用 collections/objects/containers 替换 `app/mock`。
- 接入 Sprite/Texture2D PNG Preview 和 full detail Dump。
- 完成表格、container 树、过滤、停靠、标签排序和快捷键。
- 将 Rust stdout/stderr 与代理错误送入可调整高度的日志面板。

### 后端展示能力补全

typed、dynamic 和 MonoBehaviour 已统一提供结构化 Dump。对象详情返回有界检查视图；
数组通过 `/environment/collections/{cid}/objects/{pid}/inspect` 按字段路径分页，
每页最多 256 项，字节与长字符串也支持翻页。typed 检查直接访问生成字段，
不会先复制整个对象再截断。切换对象或模式后丢弃旧分页响应。

菜单 Export Current Object / Ctrl+E 经 `files:save` IPC 打开原生保存对话框。
Texture2D/Sprite 保存 PNG，音频保存当前 sample，其余专用资源沿用原格式；
无专用导出器的对象保存完整 JSON。JSON 预览使用有界详情，完整导出仅在保存时请求。
状态栏列出 `/environment/failures`，对象级失败可经 `/raw` 保存原始序列化字节。
导出 worker 在持有环境读锁时校验 revision，过期请求返回 409。

对象字节大小、图片尺寸等展示字段仍需继续补全 DTO，不从 Debug 字符串猜测。
