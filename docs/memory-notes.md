# 内存分配审查笔记

> 记录一次内存审查（解析完成后内存是否充分释放）的发现。
> 现状：Issue #1 已修复；Issue #2-#4 待处理。

---

## 0. 总体结论

**无内存泄漏**——所有分配都有明确的 owner，drop 链路完整。问题集中在：

- 一处 **Arc pinning 导致的过度驻留**（已修复，见 Issue #1）
- 若干 **瞬态分配** 的优化空间（Issue #2-#4）

### 解析完成后的生命周期快照

`Environment::from_bundle` 完成后，`Environment` 持有：

| 字段 | 内容 | 引用 blob？ |
|---|---|---|
| `objects` | 全部 owned 对象（typed `UnityObject` / dynamic `Value`） | 否 |
| `collections` | 元数据 + `HashMap<i64, usize>` 依赖表 | 否 |
| `resources` | `.resS` 的 `Bytes` | **是**（见 Issue #1） |

typed struct 和 `Value` 树都是自包含的——解析后不引用原始字节，
`SerializedFile`（含其 TypeNode 树、blob 节点、string buffer）在每次循环末尾 drop。

---

## Issue #1（已修复）：blob Arc pinning

### 问题

`src/bundle.rs` 把所有 `BundleFile.bytes` 切为 `blob_bytes.slice(off..sz)`——
全部共享同一 `Arc<[u8]>`。`Bytes::slice` **永远共享底层分配**：只要任何一个
slice 存活，整个 blob 不会释放。

在 `Environment::from_bundle` 中：
- SerializedFile 的 .assets 区域解析成 owned 对象后不再需要
- .resS 资源文件必须保留（供 `streaming_data()` 零拷贝取数）

两者共享同一 blob Arc → **整个 blob（含已不需要的 .assets 区域）100% 驻留**。

### 修复

`src/environment.rs` 收集 `ResourceFile` 时用 `Bytes::copy_from_slice(&bf.bytes)`
detach 到独立分配。这样所有 SerializedFile 解析完毕后 blob Arc refcount 归零，
整个解压 blob 被释放。

代价：拷贝一次资源数据（O(资源大小)）。收益：释放整个 blob（O(blob 大小)），
含不再需要的 .assets 区域。

### 影响范围

纯数据 bundle（无 .resS）不受影响——所有 Bytes slice 在 SF drop 后释放，
blob Arc 天然归零。修复只对**混合 bundle**（SerializedFile + .resS）有实际收益。

---

## Issue #2（待处理）：`ref_type_table()` 深拷贝 TypeNode 树

**位置**：`src/serialized_file.rs:99-112`

```rust
pub fn ref_type_table(&self) -> RefTypeTable {
    for rt in &self.ref_types {
        t.push(c.clone(), n.clone(), a.clone(), node.clone());  // 整棵树 clone
    }
}
```

每次调用都深拷贝 ref type 的 `TypeNode` 树。`Environment::from_bundle` 每个
SerializedFile 调用一次（`environment.rs:143`），用于该 SF 全部对象解析后即 drop。

**性质**：瞬态分配，会被释放——非泄漏，但解析期间产生一份重复的树。若
`[SerializeReference]` 多，解析期短暂内存翻倍。

**修复方向**：
- `RefTypeTable` 持 `&TypeNode` 引用（加生命周期参数）
- 或 `SerializedFile.ref_types` 改用 `Arc<TypeNode>`，`RefTypeTable` 持 `Arc` clone
  （廉价 refcount bump，非深拷贝）

---

## Issue #3（待处理）：逐块解压的 double-buffer

**位置**：`src/bundle.rs:231-238`

```rust
let out = registry.get(comp_ct)?.decompress(comp_bytes, ...)?;  // 分配 Vec<u8>
blob.extend_from_slice(&out);                                    // 再拷贝进 blob
// out drop
```

每个压缩块先解压进临时 `Vec<u8>`，再 `extend_from_slice` 拷贝进 blob。
峰值内存 = blob 全量 + 最大单块大小。

`IdentityDecompressor`（NONE 压缩，`decompress.rs:30-32`）尤甚：`input.to_vec()`
先拷贝一次，`extend_from_slice` 再拷贝一次——**无压缩块被拷贝两次**。

**修复方向**：`Decompressor` trait 增加
`decompress_into(&self, input: &[u8], out: &mut Vec<u8>, expected: usize)`
变体，让解压器直接写入 blob 尾部。LZ4/LZMA 的库 API（`lz4_flex::decompress_into`、
`lzma_rs` 的 streaming writer）均支持。

---

## Issue #4（待处理）：解释器数组预分配上限 4096

**位置**：`src/typetree/interpret.rs:102`

```rust
let mut out = Vec::with_capacity((size as usize).min(4096));
```

对大数组（如 Mesh 顶点数组可能上万元素），4096 上限导致多次 reallocation。
设计意图是防恶意/异常大 `size` 导致 OOM，但 4096 对常见场景偏保守。

**修复方向**：改为 `size.min(合理上限)`（如 `1 << 20`），兼顾安全与减少 realloc。

---

## 做得好的地方

- **`Bytes` 贯穿管线**——资源数据全程 Arc 共享，`streaming_data()` 的 slice O(1)。
- **解析产物自包含**——`UnityObject` / `Value` 不持有原始字节引用。
- **SF 逐文件 drop**——`environment.rs` 循环中每个 SF 解析完即 drop。
- **`Vec::with_capacity` 预分配**——blob、nodes、objects、types 均预分配。
- **错误隔离**——`ParseFailure` 收集错误而非中断，避免部分解析后的悬挂状态。
