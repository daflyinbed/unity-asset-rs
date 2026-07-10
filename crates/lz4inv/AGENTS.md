# lz4inv crate

明日方舟（Arknights）自定义 LZ4 变体解压。纯 Rust，零依赖。

## 与标准 LZ4 的两处差异

1. **Token nibble 对调**：标准 LZ4 高 4 位 = literal 长度 / 低 4 位 = match 长度；明日方舟反过来。
2. **Offset 字节序反转**：标准 LZ4 offset 为 little-endian u16；明日方舟为 big-endian u16。

其余（literal 拷贝、match 重叠拷贝、0xFF 长度扩展、最小 match = 4）完全一致。

## 来源

从外部 `../../lz4inv` crate（PyO3 绑定版本）复制而来，去掉了 pyo3 依赖，保留纯 Rust
`decompress` 函数。原始仓库：<https://github.com/MooncellWiki/lz4inv>

## 接入方式

本 crate 不依赖主 crate，也不内置游戏特定注册逻辑。主 crate 的 `Registry` 在
`src/decompress.rs` 通过 `register(CompressionType(4), ...)` 将此 crate 的 `decompress`
注册到 flag 4/5 槽位（LZHAM 死槽位）。
