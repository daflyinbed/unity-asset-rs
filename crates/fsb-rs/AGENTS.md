# AGENTS.md

## 项目目标

`unity-asset-rs` workspace 的成员 crate（`crates/fsb-rs`，供 `unity-asset` 的
`export` feature 经 `audio_export.rs` 调用）。将 `../../../../Fmod5Sharp` 的 FSB5
容器解析层与（非 Vorbis 的）codec 重建层移植到 Rust。当前 `parse(&[u8])` 解析容器
并零拷贝暴露每个 sample 的编码字节；
`Sample::rebuild` / `SoundBank::rebuild_sample` 进一步把 PCM / GCADPCM / IMAADPCM
/ FADPCM 解码并重封装成 WAV，并通过内嵌 setup-header CRC 查找表把 Vorbis packet
重建、分页为 Ogg。

## 项目结构

- `src/lib.rs`：公开入口 `parse(&[u8])`、name-table 解析、模块导出
- `src/header.rs`：FSB5 header 与 sample-header 区域
- `src/metadata.rs`：sample 位字段和频率表
- `src/audio_type.rs`：FMOD codec 枚举（`#[repr(u32)]`，未知 discriminant
  经 `AudioType::Unknown` 原样保留）
- `src/chunk.rs`、`src/chunk_data.rs`：chunk header 与 payload
- `src/reader.rs`：bounds-checked 字节读取器
- `src/sample.rs`、`src/sound_bank.rs`：公开容器类型
- `src/error.rs`：`FsbError`（基于 `thiserror`），所有错误路径返回此枚举
- `src/rebuild/`：codec 解码 + WAV 重封装层（对照
  `../../../../Fmod5Sharp/Fmod5Sharp/CodecRebuilders/` 与 `../../../../NAudio` 的
  `WaveFileWriter`/`WaveFormat`）
  - `mod.rs`：入口分发（`Sample::rebuild` / `SoundBank::rebuild_sample`），
    `RebuiltAudio` 枚举
  - `error.rs`：`RebuildError`（`UnsupportedCodec` / `MissingDspCoeff` /
    `SampleDataTooShort` / `InvalidChannelCount`）
  - `wav.rs`：最小 PCM WAV writer（照 NAudio，但正确回填 RIFF/data size）
  - `pcm.rs`、`gcadpcm.rs`、`ima_adpcm.rs`、`fadpcm.rs`：逐 codec 解码器
  - `vorbis.rs`、`vorbis_headers_converted.json`：Vorbis packet 重建、setup-header
    CRC 查找与 Ogg 分页
- `examples/dump_all.rs`：`cargo run --example dump_all`，遍历 fixtures 打印结构
- `tests/parse.rs`：真实 fixtures 的结构测试及损坏输入测试
- `tests/rebuild.rs`：解码后的 sample 字节与 `tests/golden/*.body.bin`（由上游
  Fmod5Sharp 重建器生成）逐字节比对的回归测试
- `tests/golden/*.body.bin`：上游 `FmodPcmRebuilder` / `FmodGcadPcmRebuilder` /
  `FmodImaAdPcmRebuilder` 输出的 WAV audio body（`wav[38..]`），作为解码正确性
  的 golden reference

## 实现约束

- 保持 `parse` 零拷贝：`Sample::sample_bytes` 应继续借用输入切片。
- 所有普通多字节字段使用小端；GCADPCM DSP 系数使用大端 `i16`。
- 未知 audio/chunk 类型必须保留原始 discriminant 或 payload，不能仅因未知类型
  拒绝整个 bank。
- 文件中的 count、size、offset 和 chunk 链均不可信。读取必须限制在各自声明的
  sample-header、name-table 和 data 区域内；损坏输入应返回 `FsbError`，不能
  panic 或触发不受控的大额分配。
- `size_of_sample_headers` 可能包含对齐填充，因此只要求解析不得越过该区域，不
  要求 metadata/chunk 恰好消费全部字节。
- 容器行为应对照 `../../../../Fmod5Sharp/Fmod5Sharp/FsbLoader.cs` 和
  `../../../../Fmod5Sharp/Fmod5Sharp/FmodTypes/`；不需要复刻原实现对损坏输入的异常或
  静默越界行为。
- 重建层（`src/rebuild/`）：解码后的 PCM 样本必须与上游 Fmod5Sharp 的输出逐字节
  一致（由 `tests/rebuild.rs` 的 golden 比对保证）；但 WAV 头部的 RIFF/data size
  字段需正确回填——NAudio 在部分 rebuilder 路径因
  `MemoryStream.GetBuffer()`/`ToArray()` 的 dispose 顺序问题会把这些 size 留成
  0，本仓库有意修正这一缺陷，故 golden 只比对 `wav[38+8..]` 的样本区，不比对头部。
- GCADPCM 解码依赖 sample 的 `DSPCOEFF` chunk（16 个大端 `i16` 系数/声道）；缺失
  返回 `RebuildError::MissingDspCoeff`。
- 不支持的 codec 返回 `RebuildError::UnsupportedCodec`，不得 panic；当前包括
  一切非 PCM/GCADPCM/IMAADPCM/FADPCM/Vorbis 类型。
- 不可信的下标（如 GCADPCM 的 4-bit predictor、ADPCM 的 step/数组偏移）在索引前
  必须 bounds-check，损坏输入返回 `RebuildError::SampleDataTooShort`。

## 验证

提交前运行：

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

新增格式支持或边界处理时，应在 `tests/parse.rs` 中添加 fixture 断言或最小合成
输入回归测试。

新增 codec 重建支持或解码边界处理时，应在 `tests/rebuild.rs` 中添加 golden 字节
比对（用上游 Fmod5Sharp 重建器生成 `.body.bin`）或最小合成输入回归测试。由于上游
没有 FADPCM / PCM8 / PCM32 的 fixture，这些 codec 目前只能靠合成输入自测。
