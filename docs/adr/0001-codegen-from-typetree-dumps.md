# ADR-0001: 基于 TypeTreeDumps 的离线代码生成构建类型化 Rust 结构与 Parser

> Status: Accepted（核心决策已落地；实现偏差见文末「修订」附录）
> Date: 2026-06-19
> Deciders: unity-asset-rs 项目维护者

## 背景 (Context)

目标是从 Unity AssetBundle 中提取资产，核心难点是：Unity 的原生类（Texture2D / Mesh / AudioClip … 共 412 个 TypeID）在不同引擎版本下**字段布局不同**。对 TypeTreeDumps 仓库（`AssetRipper/TypeTreeDumps`，17GB JSON，覆盖 Unity 3.4 → 6000.x 共 ~1395 个版本）的实测显示：

- 412 个 TypeID 累计出现 429,026 次，但**真正独特的布局只有 4,163 种**（跨版本去重压缩比 103:1，99% 是版本间重复）。
- 变化几乎全是**字段单调增删**：相邻版本对中，1076 个完全相同、316 个增删字段、仅 12 个类型变化、2 个纯重排。
- PlayerSettings 最极端（261 字段、165 种布局），但其差异也 99% 是增删。
- 23/412 个 class 的**父类类型跨版本变化**（如 AudioClip: `NamedObject → SampleClip`）。
- 814 个 (class,字段) 出现过"类型变化"，但绝大多数是 Unity 改命名约定（`SInt32↔int`、`UInt8↔bool`），真语义变化极少。

约束：需要覆盖全部版本、保证类型安全与解析性能，且维护成本可控。

## 决策 (Decision)

**采用离线代码生成**：编写一个独立的 Rust codegen 工具，以 AssetRipper 的 `type_tree.tpk`（已跨版本去重 + 节点池的二进制容器，brotli 压缩后仅 187KB）为输入，为 366 个有字段的 Unity 原生类生成强类型 `struct` + 按 `UnityVersion` 门控的 `parse` 方法，统一收纳进 `enum UnityObject`（按 class 分派，不按布局变体分）。生成产物约 2 万行 Rust，通过 `build.rs` 在构建期编译进主 crate。具体形态：

- **类型组织**：`enum UnityObject` 共 ~366 个变体（一个 class 一个），版本差异在 `parse` 内部用 `match version` / `Option<T>` 处理。
- **继承**：parser 按继承链组合复用（`base.parse()` 先执行，天然匹配 Unity 扁平字节流顺序）。389 个父类稳定的 class 用 `parent: ConcreteType`；23 个父类变化的 class 用 `parent: ParentEnum`（2–3 分支）或扁平化。
- **版本门控**：5,019 个字段并集中 2,818 个需门控，用 `Option<T>` + `if ver >= X` 表达，**不**使用 per-version enum 变体。
- **类型归一化**：在生成器内置一张别名表，消除 `SInt32↔int` 等"假"类型变化。
- **内存**：对象用 arena（`bumpalo`）分配；`PPtr` 用 `(file_id, path_id) → 索引` 的 `HashMap` 解析跨引用。

## 备选方案 (Alternatives Considered)

| 方案 | 优点 | 缺点 | 主要影响维度 |
| --- | --- | --- | --- |
| A. 运行时 TypeTree 解释器（UnityPy 做法） | 一个通用函数读全部类型，实现简单，~1k 行 | 弱类型返回 `Value`、字段靠字符串 key；性能差（UnityPy 实测一半时间耗在 typetree 解析，需 C 扩展补救） | 性能:-, 可维护性:- |
| B. 手写所有 struct + parser | 类型安全、可读性好 | 4,163 种唯一布局 / PlayerSettings 261 字段，手写不现实。AssetStudio 也只手写 ~30 个核心类 | 可维护性:-, 成本:- |
| C. 混合：手写核心 + 解释器兜底（AssetStudio 策略） | 核心类型性能好、兜底全覆盖 | 既然代码生成已能全覆盖且性能更优，混合方案的兜底路径失去意义，维护两套机制反而更重 | 成本:-, 可维护性:- |
| D. per-version enum 变体 | 每个版本布局类型精确 | PlayerSettings 等会有几百个变体，字段跨变体大量重复定义，访问任何字段都要先 `match` 解构 | 可维护性:- |

**否决理由**：

- **A**：牺牲类型安全与性能，与项目"类型化 + 高性能"目标冲突；Rust 下弱类型 `Value` 抵消了语言优势。
- **B**：工作量不可控，无法覆盖 4,163 种布局；即便只覆盖核心类型，版本门控的手写 `if` 仍极繁琐。
- **C**：codegen 方案已含其全部优点（核心类型强类型 + 全覆盖），引入解释器只会增加第二条维护路径。
- **D**：与"变化是单调增删"的数据形态不符；离散变体套渐变字段会导致变体爆炸与重复定义。

## 后果 (Consequences)

### 正面
- 强类型 + 编译期检查 + IDE 补全，错误前移到构建期。
- 解析性能最优：无运行时解释、无字符串比较、parser 按继承链组合（基类 parse 代码复用）。
- 全版本（3.4 → 6000.x）全覆盖，366 个类无遗漏。
- 人类只维护生成器（~1–2k 行一次性投入），生成产物（~20k 行）不手动修改。

### 负面
- 生成代码量约 2 万行，仓库体积膨胀；diff 噪声大。
- 生成器本身有开发与维护成本，调试生成代码时需追溯到生成器逻辑。
- 新 Unity 版本发布后需重跑 dump + 生成（TPK 的 CI 每周自动更新，但本仓库 codegen 需手动触发重建）。

### 中性 / 待观察
- 版本门控字段的写法（helper 宏 vs 展开 `if`）影响可读性与行数，可后续调优。
- 若未来需要双向读写（Write/保存），parse 部分行数翻倍（当前只生成读路径）。

## 质量权衡 (Quality Trade-offs)

| 维度 | 影响 | 说明 |
| --- | --- | --- |
| 可维护性 | ↑ | 生成器集中维护，类型定义单一来源（TPK）；相比手写/解释器，长期维护面更小 |
| 可靠性 | ↑ | 编译期类型检查；全版本覆盖；类型归一化表消除命名差异导致的解析错误 |
| 性能 | ↑ | 零运行时解释、零字符串比较、组合式 parser 复用基类代码；arena 分配连续 cache 友好 |
| 安全性 | → | 不涉及特权/网络/密码学；内存安全由 Rust 类型系统保证；TPK 为公开 dump 数据，无敏感输入 |
| 成本 | ↑（一次性投入）/ ↓（长期） | 生成器开发是一次性成本；相比手写数千布局或维护解释器+C 扩展，长期成本更低 |

## 合规与风险 (Compliance & Risks)

- **法律**：TypeTreeDumps / TPK 均为 MIT 协议公开数据；本仓库生成代码为自有实现，无许可证冲突风险。
- **上游依赖风险**：依赖 AssetRipper 的 TPK 格式与 TypeTreeDumps 的 JSON 结构；若上游格式变更，生成器需同步适配。缓解：生成器仅依赖 TPK 的稳定公开格式（`TpkFile` magic `TPK*`），不耦合其打包工具内部。
- **版本滞后风险**：TPK CI 每周更新，新 Unity 版本最多延迟一周即可支持；本仓库需建立"更新 TPK → 重跑 codegen → 提交"的常规流程。

## 实施备注 (Implementation Notes)

1. **生成器 crate**：新建 `crates/codegen`（独立 bin），依赖 `AssetRipper.Tpk` 等价的 Rust TPK 读取逻辑（自行实现 `TpkFile` 解析，约 200 行）。
2. **输入**：读取 `type_tree.tpk`（brotli），解出 `TpkTypeTreeBlob`，遍历 `ClassInformation`（每个 classID 的 `[(版本, 布局)]` 变更点列表）。
3. **类型归一化表**：内置别名映射（`SInt32→i32`、`UInt8→bool/u8`、`UInt32→u32`、`SInt64→i64`、`unsigned int→u32` 等 ~10 对），在生成期统一。
4. **生成产物**：输出到 `src/generated/`，通过主 crate 的 `build.rs` 在 `OUT_DIR` 编译；生成产物加入 `.gitattributes` 标记为生成文件。
5. **继承处理**：检测每个 class 的 `Base` 是否跨版本变化——稳定则生成 `parent: ConcreteType`，否则生成 `parent: XxxParentEnum`（枚举各历史父类）或扁平化。
6. **版本门控**：对每个 gated 字段生成 `let x = if ver >= V_X { Some(r.read_t()?) } else { None };`，字段类型用 `Option<T>`。
7. **arena + PPtr**：`World { objects: Vec<UnityObject>, by_path: HashMap<(i32,i64), usize> }`，`PPtr::resolve` 通过索引查表。
8. **验收基线**：生成产物约 1.8–2.3 万行；用 TypeTreeDumps 的若干样本 JSON 做解析回放测试（读入 → 比对字段值）。

---

## 修订（2026-07-09，对齐实现）

本决策已落地（`crates/codegen` + `crates/generated` + 运行时 dispatch），但实现与正文有若干偏差。权威现状见 [`docs/codegen.md`](../codegen.md)（codegen）与 [`docs/object-layer.md`](../object-layer.md)（运行时 dispatch）。

| 维度 | 正文（2026-06-19 原决策） | 实际实现（2026-07-09） |
|---|---|---|
| **继承** | 「parser 按继承链组合复用」，389 父类稳定的 class 用 `parent: ConcreteType`，23 父类变化的用 `parent: ParentEnum` 或扁平化 | **改为完全扁平 struct，无 parent 字段、无 parent enum**。TPK 的 release_root 已含全部继承链字段，p05 字段并集自动处理父类变化（差异体现为 `Option<T>` + VersionGate）。原因：abstract 父类的 release_root 为空、字段已平铺到子类，嵌套 parent 实际无意义 |
| **内存** | 「arena（bumpalo）分配」 | 首版**按需 parse、无 arena**。对象由调用方逐个 `parse_object`，owned。bumpalo 列入路线图 |
| **MonoBehaviour** | 未明说是否生成 | **完全不生成**（p04 跳过 classID=114）。运行时由 TypeTree 解释器处理 |
| **生成规模** | 「约 2 万行」 | 实测 **108,279 行 / 940 文件**（361 main class struct + 574 subclass struct + 特殊文件），远超估算（含 subclass + 版本枚举） |
| **类型变化** | 「类型归一化表消除命名差异」，真冲突落 Option + 版本 match | 新增 **`VersionedEnum`**：字段类型或 align 跨版本变化时生成版本枚举 + `impl Parse`，按版本选变体 |
| **shared crate** | 未提 unity-core | 抽出 [`crates/unity-core`](../../crates/unity-core)（~150 行，零依赖），统一 `UnityVersion` / `Endian` / `EndianReader` / `Parse` / `PPtr` |
| **字段顺序** | 未提 | 新增 **`version_layouts`**：字段顺序跨版本不兼容时（如新字段插在旧字段前）按版本分段读取（锚点 LCS） |
| **产物分发**（实施备注 4） | 「输出到 `src/generated/`，通过主 crate 的 `build.rs` 在 `OUT_DIR` 编译」 | **改为 commit 生成产物**：独立 crate [`crates/generated`](../../crates/generated)（源码提交进仓库、独立编译缓存），不再用 `build.rs`/`OUT_DIR`。codegen 只 emit 引用 `unity-core`/`generated` 的源码字符串，自身不 link 它们（见 [`docs/codegen.md`](../codegen.md)） |

正文其余部分（离线 codegen 决策、TPK 单一来源、全版本全覆盖、可维护性/性能收益）与实现一致，仍然成立。
