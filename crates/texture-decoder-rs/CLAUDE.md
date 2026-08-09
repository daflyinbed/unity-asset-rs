# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 这个 crate 是什么

`texture-decoder-rs`：`#![no_std]`、无分配的 GPU 块压缩纹理解码器——ASTC LDR、ETC1/ETC2/EAC、BC1–BC7，外加两个可选的 Crunch 容器解码器。唯一依赖是 `half`（BC6H 用）。

它是 `unity-asset-rs` workspace 的 member，被 `crates/unity-asset` 的 `export` feature 以 `features = ["crn", "crn-unity"]` 引入（见 `crates/unity-asset/src/texture_export.rs` 的 5 路分派）。**改公共 API 前先看那个调用方**。

Workspace 级约定（跨 crate 规则、参考实现位置）见仓库根 [`AGENTS.md`](../../AGENTS.md)；本 crate 自身的对外文档是 [`README.md`](README.md) 与 [`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md)。

## Commands

```bash
# 日常三件套（在本目录下跑；--all-features 才会编到 crn / crn-unity）
cargo fmt --check
cargo test --all-features --all-targets
cargo doc --all-features --no-deps

# 从 workspace 根定向本 crate
cargo test -p texture-decoder-rs --all-features --all-targets
cargo clippy -p texture-decoder-rs --all-features --all-targets

# 单个集成测试文件 / 单个测试函数
cargo test --test bcn_end_to_end
cargo test --all-features --test crn_unity_end_to_end -- --exact decodes_etc2a_fixture_to_reference_blocks
cargo test --lib                      # 只跑各 decoder.rs 底部的 #[cfg(test)] 单测
```

测试完全 hermetic：不联网、不调用任何参考实现，只读 `tests/data/` 下已提交的 fixture。根 `Cargo.toml` 的 `[profile.dev]/[profile.test] opt-level = 1` 对本 crate 同样生效。

fixture / 表格再生（仅在有意更新时跑，需 `git submodule update --init --recursive`）：

| 目标 | 命令 | 前置 |
| --- | --- | --- |
| `src/bcn/tables.rs` | `python3 tools/gen_bcn_tables.py` | Python 3 + `vendor/compressonator` |
| BC1–BC7 golden | `tools/gen_bcn_golden.sh` | `vendor/compressonator` 静态库已构建 |
| BC6H 舍入 golden | `tools/gen_bc6h_bcdec.sh` | `vendor/bcdec` |
| CRN golden | `tools/gen_crn_golden.sh` | `vendor/crunch` 的 `crnlib/crunch` CLI |
| Unity-CRN golden | `tools/gen_crn_unity_golden.sh` | `vendor/unity-crunch`（`unity` 分支）CLI |

## 架构

```
lib.rs (#![no_std])          error.rs → DecodeError（4 个变体，全 crate 共用）
 ├ astc/  decoder.rs + tables.rs
 ├ etc/   decoder.rs
 ├ bcn/   decoder.rs + tables.rs（tables.rs 由脚本生成）
 ├ crn/       decoder.rs   feature = "crn"        → extern crate alloc
 └ crn_unity/ decoder.rs   feature = "crn-unity"  → extern crate alloc
```

**每个 `mod.rs` 只做两件事**：模块文档（写明该族的输出约定）+ 显式 `pub use decoder::{...}` 白名单。`decoder.rs` 里的一切默认私有——新增/删除 entry point 必须同步改 `mod.rs` 的 re-export 列表，否则外部看不到。

### 三族解码器共享的形状（astc / etc / bcn）

1. **公共 API 由 macro 展开**，一条 macro 同时生成 `..._block`（单块）和同名 image 版：`footprint_api!`（astc，14 种 footprint）、`image_api_8!` / `image_api_16!`（etc，按块字节数分）、`image_api!`（bcn，块字节数作 macro 参数）。加新格式 = 加一行 macro 调用 + 一个私有 `decode_*_inner` + 一行 re-export。
2. **image 版都走各自的 `decode_image`**：先 `checked_mul` 算 output/input 长度（溢出 → `SizeOverflow`，不足 → `OutputTooShort` / `InputTooShort`），再按 block 行主序遍历，解到栈上的临时块缓冲，最后 `copy_from_slice` 按 `min(block_dim, 剩余像素)` 裁掉右/下边缘。etc 与 bcn 的版本是 `const BLOCK_BYTES` 泛型 + 函数指针；astc 的因为 footprint 可变（临时缓冲固定 `12*12*4`）单独写了一份。
3. **输出契约恒定**：紧密排列的行主序 RGBA8，`output` 可以比需要的长（只写前 `w*h*4` 字节）；所有可恢复失败返回 `DecodeError`，绝不 panic、绝不部分写入后再报 `OutputTooShort`（长度检查在写之前）。

### crn / crn_unity（例外的两个）

这两个模块**打破上面所有契约**：输入是压缩容器而不是裸块，输出是**原始物理块字节**（BC1/BC3/ETC1/ETC2A）而不是 RGBA8，错误类型是自己的 `CrnDecodeError` / `CrnUnityDecodeError`，且需要 `alloc`。返回结构是 `texture.faces[face].levels[level]`——2D 一个 face，cubemap 六个按 `+X,-X,+Y,-Y,+Z,-Z`，level 0 最大。调用方要 RGBA8 时自己再喂给 `bcn` / `etc`。

两个模块内部结构几乎平行（header 解析 → `BitReader` → `HuffmanTable` → 调色板/选择器解码 → `unpack_*` 重组块），**但 bitstream 互不兼容**：upstream Crunch 用 "chunk encoding"，Unity fork 换成了 "reference encoding" 并加了 ETC1/ETC2A。所以它们是有意的两份独立实现——在一边修的 bug 不要想当然搬到另一边，也不要为了"去重"把它们抽象合并。`unity-asset` 侧按 Unity 版本 + format 分派（`ver >= 2017.3` 或 format ∈ {64,65} → Unity 变体）。

## 硬性约定

- **`no_std`**：库代码只能用 `core`；`alloc` 仅在 `crn` / `crn-unity` feature 下可用（`lib.rs` 里是 `#[cfg(any(feature = "crn", feature = "crn-unity"))] extern crate alloc`）。集成测试用 `std` 只为读 fixture。
- **不做任何色彩变换**：没有 sRGB / gamma / 曝光 / tone mapping。sRGB 块直接用对应的 RGB entry point。BC4/BC5 SNORM 未实现。BC6H 的 f16→u8 是 `<=0 → 0`、`>=1 → 255`、否则 `floor(v*255+0.5)`——改这个映射会同时打穿 compressonator 与 bcdec 两套 golden。
- **`src/bcn/tables.rs` 是生成物**，改它要跑 `tools/gen_bcn_tables.py` 并 review diff，别手写。`src/astc/tables.rs` 是手工整理的（对照 astc-encoder），不由脚本生成。
- **测试里不许调用参考实现**：新增覆盖 = 用 `tools/` 的生成器产出 `.bcn`/`.rgba`、`.crn`/`.bc` 之类的 pair 并提交，测试只做字节级比对。fixture 的来源与许可证记录在 [`tests/data/README.md`](tests/data/README.md)，动 fixture 时同步更新。
- **`vendor/` 是只读 submodule**，只用于再生表格和 fixture，不参与编译、不进链接；各自保留上游许可证（ETCPACK 是 Ericsson SLA，unity-crunch 是 ZLIB，其余 MIT/Apache/public domain）。
- 已知不对称：`bcn::decode_image` 对 `width == 0 || height == 0` 有提前返回（注释解释了为什么——否则 `blocks_y` 会空转近 `usize::MAX` 次），**astc 与 etc 的 `decode_image` 没有这个 guard**，`width = 0` 配上极大 height 会挂死。碰到相关改动时顺手补上。
