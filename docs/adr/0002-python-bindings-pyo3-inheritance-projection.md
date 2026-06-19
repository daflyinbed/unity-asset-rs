# ADR-0002: Python 绑定采用 pyo3 + 继承投影 codegen，类型来源限定 tpk 与 AB typetree

> Status: Proposed
> Date: 2026-06-20
> Deciders: unity-asset-rs 项目维护者

## 背景 (Context)

ADR-0001 已确定 Rust core 采用 tpk 离线代码生成，用**组合式 `parent`** 模拟 Unity 继承（Rust 无继承）。本决策解决下一个问题：**如何把这套强类型暴露给 Python 开发者**（替代 UnityPy 的场景）。

驱动该决策的实测痛点（来自 `torappu` 项目对 UnityPy 的实际使用，`enemy_spine.py`）：

- UnityPy 的 `MonoBehaviour` / `PPtr` 字段是弱类型，开发者满屏 `cast("TextAsset", data.skeletonJSON.read())` 与 `# type: ignore`，连字段名都要 `getattr(..., None)` 试探。
- 根因有二：`PPtr` 解引用返回泛化 `Object`（类型丢失）；`MonoBehaviour` 是游戏自定义脚本，无 schema。

关键约束与事实：

- **Python 原生支持继承**，无需沿用 Rust 的 `parent: Box<T>` 组合——开发者看到的应是 `class Texture2D(Texture)`，而非 `tex.parent.parent.m_Name`。
- AB 是否内嵌 typetree 是分水岭：保留时可直接读（权威），剥离时（游戏发布常态，尤其手游）原生类只能靠 tpk、MonoBehaviour 无来源。tpk 不含 MonoBehaviour。
- 版本特化数据（见 ADR-0001 背景与版本分布分析）：patch 间变化高度集中在 `PlayerSettings` 等 editor-only 类（不在游戏 AB 内），核心运行时类（GameObject/MonoBehaviour/Transform = 0 变化，Texture2D/Mesh = 个位数）patch 间极稳定。

## 决策 (Decision)

采用三层结构暴露 Python 绑定：

1. **pyo3 后端层**：将 Rust core 暴露为**扁平 pyclass**——每个 Unity 类型一个 `#[pyclass]`，字段 getter 在 Rust 内穿透 `parent` 组合链（编译期确定、零成本），对外呈现为扁平字段集合。
2. **codegen CLI 生成 Python wrapper 与 `.pyi`**：独立工具 `unity-asset gen --target <UnityVersion> --python`，以 tpk 的 `Base` 继承元数据驱动，产出**真继承**的 wrapper（`class Texture2D(Texture)`，property 委托到扁平 pyo3 后端）+ 配套强类型 `.pyi`。版本特化到精确 patch：字段**非 `Optional`**，仅含该版本真实存在的字段。
3. **类型来源限定为两条**：(a) **tpk 按版本生成**（原生类，开发期一次性生成，全量兜底）；(b) **AB 内嵌 typetree**（运行时补 MonoBehaviour，能力降级）。**不支持 dll / IL2CPP 解析**——MonoBehaviour 在 AB 剥离 typetree 时降级为动态 dict。

运行时配合：`load()` 后 `assert_version(env, target)` 一次性断言实际版本匹配，wrapper 解包 `Option` 安全；MonoBehaviour 按 `AB typetree > 动态 dict` 降级。

## 备选方案 (Alternatives Considered)

| 方案 | 优点 | 缺点 | 主要影响维度 |
| --- | --- | --- | --- |
| A. pyo3 直接暴露 Rust 组合 `parent`（不做继承投影） | 一层，codegen 简单 | Python 用户访问 `obj.parent.parent.m_Name`，体验差；`isinstance` 失效；cast 不消除 | 可维护性:-, 可靠性:- |
| B. 运行时反射动态构建 Python 类（无预生成 pyi） | 无需 codegen 二次输出 | 无静态类型/`.pyi`，IDE 无补全，mypy 失效，回到 UnityPy 弱类型痛点 | 可维护性:-, 可靠性:- |
| C. pyo3 原生 `#[pyclass(extends=)]` 机制 | Python 侧真继承，单层 | 每个字段 getter 需在 Rust 内手工穿透 base 链，pyo3 `extends` 对字段访问支持别扭，与扁平后端设计冲突 | 可维护性:-, 成本:- |
| D. 额外支持 dll / IL2CPP 解析 MonoBehaviour | MonoBehaviour 全场景强类型 | IL2CPP 需 Cpp2IL 类逆向工具链，实现复杂度极高；Mono 后端需 Mono.Cecil 等价物 | 成本:-, 可维护性:- |

**否决理由**：

- **A**：直接违背"开发者见 Python 继承、消灭 cast"的核心目标；暴露 Rust 组合是语言局限的泄漏。
- **B**：放弃静态类型即放弃相对 UnityPy 的主要优势；开发者仍需手写 cast/注释。
- **C**：pyo3 `extends` 的字段穿透写法繁琐且与 ADR-0001 的扁平/组合核心模型不符；扁平 pyclass + Python wrapper 投影更清晰、codegen 更可控。
- **D**：首期投入产出比低——IL2CPP 逆向是一整套子项目；MonoBehaviour 在 AB 有 typetree 时已强类型，剥离时降级 dict 可接受（原生类仍由 tpk 完美覆盖，占游戏资产主体）。留作未来 ADR。

## 后果 (Consequences)

### 正面
- 开发者拿到 Python **原生继承**的强类型：`isinstance(obj, Texture2D)` 成立、IDE 完整补全、`PPtr[T].resolve()` 返回具体类型——`enemy_spine.py` 中的 `cast` 在原生类路径上全部消除。
- 版本特化 `.pyi` 字段精确（非 `Optional`），同时锁定该版本字段类型，连带消除 ADR-0001 提及的"同名字段跨版本类型变化"歧义。
- Rust core（组合）与 Python（继承）各得其所，**同源于 tpk 元数据**，继承链只在一处定义、多语言投影。
- Rust 引擎编译一次、通用全版本；不同游戏的开发者各跑一次 `gen --target` 得到该游戏版本的精确 wrapper，无需重编译 Rust。

### 负面
- 三层结构（Rust core / pyo3 后端 / Python wrapper）增加理解与调试成本；codegen 多一条 Python 输出需维护。
- MonoBehaviour 在 AB 剥离 typetree 时退回动态 dict——cast 问题在该场景下**不彻底解决**（受限于本期不做 dll/IL2CPP）。
- pyo3 绑定与 CPython ABI 版本耦合，发版需为多个 Python 版本构建 wheel。

### 中性 / 待观察
- 版本断言采用精确 patch 匹配（游戏版本是 AB header 中的已知常数），断言失败早报错；是否放宽到 minor 取决于实际遇到的版本漂移频率。
- wrapper 性能：property 委托为单次属性查找，必要时可构造时预解包缓存，权衡脱离 arena 的拷贝成本。

## 质量权衡 (Quality Trade-offs)

| 维度 | 影响 | 说明 |
| --- | --- | --- |
| 可维护性 | ↑ | 继承链单一来源（tpk），Rust/Python/pyi 同源生成；wrapper 纯生成不手写。代价：三层结构增加初次理解成本 |
| 可靠性 | ↑ | 强类型 + 运行时版本断言（早失败）；`PPtr` 类型化解引用消除误 cast。代价：MonoBehaviour 降级路径行为不一致 |
| 性能 | → | pyo3 扁平 getter 在 Rust 内穿透 parent 零成本；Python wrapper property 为单次委托，可预解包优化 |
| 安全性 | → | 不涉及特权/网络/密码学；内存安全由 Rust 保证；pyo3 绑定无 unsafe 暴露面 |
| 成本 | ↑（一次性）/ →（长期） | codegen Python 输出 + pyo3 绑定为一次性投入；不做 dll/IL2CPP 显著压低首期成本 |

## 合规与风险 (Compliance & Risks)

- **pyo3 / CPython ABI**：wheel 需按 Python 版本（3.8–3.14）与平台矩阵发布；缺失预编译时用户需本地工具链构建。缓解：CI 矩阵构建 + `maturin`/`pyo3-pack` 标准化发布。
- **MonoBehaviour 降级一致性**：AB 剥离 typetree 时退回动态 dict，与有 typetree 时的强类型体验不一致。缓解：文档明确标注降级行为；`.pyi` 中 MonoBehaviour 标注为动态字段容器。
- **法律**：Rust core 与 wrapper 均为自有实现；tpk 为 MIT 公开数据；无许可证冲突。

## 实施备注 (Implementation Notes)

1. **crate 拆分**：`crates/core`（ADR-0001）、`crates/pyo3`（扁平 pyclass + getter 穿透）、`crates/codegen`（扩展 `--python` 输出，复用 tpk 读取）。
2. **pyo3 扁平后端**：每个 Unity 类型一个 `#[pyclass]`，`#[getter]` 在 Rust 内沿 `parent` 组合链取值（如 `self.handle.t2d().base.base.m_Name`），对外扁平。
3. **继承投影 codegen**：遍历 tpk `ClassInformation` 的 `Base` 字段构建继承链，生成 `class Child(Parent): ...`，property 体为 `return self._rust.<field>`（委托扁平后端）。
4. **版本特化 pyi**：`gen --target 2019.4.15f1` 查 tpk 变更点（`≤ target` 的最后布局）得精确字段集；该版本不存在的字段不生成，存在的字段类型非 `Optional`。
5. **PPtr 强类型**：tpk 的 `PPtr<T>` 目标类型驱动生成 `PPtr[Texture2D]` 泛型 + `.pyi` overload，`resolve()` 返回具体类型。
6. **运行时版本断言**：`load()` 后 `assert_version(env, target)`；不匹配抛 `VersionMismatch`（精确 patch）。
7. **MonoBehaviour 降级**：核心库 `parse` 检测 AB 的 `m_EnableTypeTree`——有则运行时解释该 Tree 填 `MonoFields::Dynamic`，无则 `MonoFields::Raw`；Python 侧 MonoBehaviour 暴露为动态字段容器（`__getattr__`），`.pyi` 标注为 `dict[str, Any]`。
8. **验收基线**：用 `torappu/enemy_spine.py` 等样本重写——原生类路径 `cast` 全消除；MonoBehaviour 在 AB 有 typetree 时字段可点访问、无 typetree 时明确降级为 dict。
