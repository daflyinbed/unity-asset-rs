# Mesh 解码与 Tight Sprite 实施计划

日期：2026-10-02。范围：Rust Mesh 几何解码、Sprite 共用顶点/索引读取、Tight PNG；不做 glTF/OBJ、3D UI、动画或场景重建。不引入第三方 Unity Mesh 库。

## 参考与边界

以本地 AssetRipper `925e09a91892dc046159162616bb8947a4afab16` 为依据：
`VertexDataBlob.cs`、`MeshHelper.cs`、`MeshExtensions.cs`、`CompressedMeshExtensions.cs`、packed vector、Sprite 扩展。
移植文件记录来源并保留上游许可证。generated 结构不手改。
AssetRipper 当前 SpriteConverter 返回整张纹理，并不实现独立 Tight PNG；该部分在已有 Sprite 导出管线中实现 CPU 三角形裁切/采样，不能声称是上游已有光栅器的直接移植。

## 实施顺序

- [x] 1. 建立内部 mesh 模块及精简公共解码结果/API，保留 Unity 坐标。
- [x] 2. 移植版本化顶点格式、channel/stream 布局、大小端、旧版数组与索引/SubMesh 拓扑读取；严格检查边界和溢出。
- [x] 3. 移植 packed bit vector 和 CompressedMesh（位置、UV、法线、切线、颜色、权重、索引）；接入 Mesh 外部流资源预取。
- [x] 4. SpriteRenderData 复用几何读取；实现 Tight 三角形图像重建，保持 atlas、rotation、downscale、透明度语义。
- [x] 5. 增加格式/边界/拓扑/压缩单元测试及现有 fixture 集成回归，更新现状文档。
- [x] 6. 运行相关测试、无 export feature 检查及 clippy，记录实际覆盖和剩余限制。

## 验证要点

多 stream、旧版 stream、Float16/SNorm、大小端、16/32 位索引、非零 baseVertex、条带/四边形、packed 跨字节与截断、压缩法线符号、外部资源、Tight 透明区域/UV/旋转。真实 fixture 不修改；用户未跟踪样本不纳入提交。

## 进度记录

- 已完成上述实现。`Environment::decode_mesh(&EnvironmentObject)` 是公共入口，Mesh 不依赖 export feature；Sprite 经既有 decode_image / PNG 入口自动支持 Tight。
- AssetRipper 移植来源与 GPL-3.0 文本见 `crates/unity-asset/src/mesh/NOTICE.md`、`licenses/AssetRipper-GPL-3.0.txt`。
- 测试：主库 148 个单元/集成测试与 2 个 doctest 通过；关闭 export feature 的 16 个 Mesh 测试通过。自包含测试覆盖外置顶点 provider、跨 collection atlas、全部 packing rotation、alpha 合并、Tight UV 采样，以及压缩/非压缩 bind pose 列主序布局一致性。
- 本地游戏样本：1 个 Mesh、377 个 Tight Sprite 解码成功（均有非透明像素，抽查 PNG 显示正常）；可选样本测试在根 `tests/mesh_sprite.rs`，游戏 codec 保持在 CLI 边界，样本未加入版本控制。
- `cargo clippy --all-targets` 通过（保留既有 generated / format / object_parser 警告）；严格 `-D warnings` 会被已有 generated 警告阻断，不修改生成物。
- 服务端 `cargo test --bin unity-asset-rs server::`：23 个测试通过。完成日期：2026-10-03。

## 当前边界

- 本次解码几何与四权重蒙皮数据，不重建骨骼层级、动画、BlendShape 应用、材质或模型文件；原始生成结构仍保留相关字段。
- 支持三角形、条带、四边形以及保留线/点拓扑；仅三角形类拓扑能用于 Tight 光栅化。
- packed vector 单次最多 64M 元素，截断/非法格式/越界返回错误；与上游相比补上 0-bit 常量和 32-bit 解包边界处理，UV 保留 1..4 维。
- CPU 光栅化使用像素中心、最近邻采样；不承诺与其他工具的抗锯齿或边缘像素 bit-for-bit 一致。没有运行 AssetRipper 二进制做全版本差分；已对照固定提交源码，并用确定性测试和当前真实样本验证。
