# 架构决策记录 (ADR)

本目录记录 unity-asset-rs 的重要架构决策。每份 ADR 只记一个决策，含背景、备选方案、后果与质量权衡。

## 索引

| 编号 | 标题 | 状态 | 日期 |
| --- | --- | --- | --- |
| [ADR-0001](0001-codegen-from-typetree-dumps.md) | 基于 TypeTreeDumps 的离线代码生成构建类型化 Rust 结构与 Parser | Accepted（核心已落地；见修订附录） | 2026-06-19 |
| [ADR-0002](0002-python-bindings-pyo3-inheritance-projection.md) | Python 绑定采用 pyo3 + 继承投影 codegen，类型来源限定 tpk 与 AB typetree | Proposed（未实施；见修订附录） | 2026-06-20 |
| [ADR-0003](0003-file-loading-and-decompression-layer.md) | 文件加载与解压层 —— 递归再派发 + 三轴差异化扩展 | Accepted（解压核心 + 多容器 + gzip/brotli + Loader + 错误隔离已落地；CN 加密未实现，见修订附录） | 2026-06-20 |

> **状态说明**：各 ADR 正文记录**决策当时**的原始论证（历史快照），文末「修订」附录记录
> 实现与决策的偏差。权威现状文档见 `docs/io-layer.md`、`docs/object-layer.md`、
> `docs/codegen.md`——冲突时以现状文档为准。
