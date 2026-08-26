# fsb-rs

`fsb-rs` 是一个 FMOD 5 Sound Bank（FSB5）容器解析库，由
[Fmod5Sharp](https://github.com/Samboy063/Fmod5Sharp) 的容器解析部分移植而来。

当前支持：

- FSB5 文件头和音频类型
- sample 位打包元数据
- sample chunks（包括 Vorbis、频率、声道、循环和 GCADPCM DSP 系数）
- sample name table
- 对原始编码 sample data 的零拷贝切片
- 损坏输入的区域及偏移校验
- PCM8/16/32、GCADPCM、IMA ADPCM、FADPCM 解码并重封装为 WAV
- 通过内嵌 setup-header CRC 表重建 Vorbis packet 并封装为 Ogg

## 使用

```rust
let bytes = std::fs::read("sound_bank.fsb")?;
let bank = fsb_rs::parse(&bytes)?;

println!(
    "{:?}: {} samples",
    bank.header.audio_type,
    bank.samples.len()
);

for sample in &bank.samples {
    println!(
        "{:?}: {} Hz, {} channels, {} encoded bytes",
        sample.name,
        sample.metadata.frequency(),
        sample.metadata.channels,
        sample.sample_bytes.len()
    );
}

let rebuilt = bank.rebuild_sample(0)?;
let extension = match &rebuilt {
    fsb_rs::RebuiltAudio::Wav(_) => "wav",
    fsb_rs::RebuiltAudio::Ogg(_) => "ogg",
};
std::fs::write(format!("sample.{extension}"), rebuilt.into_bytes())?;

# Ok::<(), Box<dyn std::error::Error>>(())
```

返回的 `SoundBank` 借用输入字节；使用 `sample_bytes` 期间需要保留原始 buffer。

## 开发

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

`tests/fixtures/` 中的测试文件来自 Fmod5Sharp 的测试资源，覆盖 Vorbis、PCM16、
GCADPCM 和 IMA ADPCM；Vorbis 输出还会通过 Ogg CRC/分页检查及实际解码验证。
`tests/golden/*.body.bin` 由上游 Fmod5Sharp（commit `28951c6`）的
FmodPcm/FmodGcadPcm/FmodImaAdPcm rebuilder 输出 dump 而来（WAV audio body，
即 `wav[38..]`）；测试自包含，运行不依赖上游仓库。

## License

MIT
