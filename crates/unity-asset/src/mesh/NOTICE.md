# AssetRipper 移植来源

本目录 Mesh 解码算法移植自 AssetRipper 项目，版权归 AssetRipper contributors。
上游：https://github.com/AssetRipper/AssetRipper
参考提交：925e09a91892dc046159162616bb8947a4afab16。

源文件位于 Source/AssetRipper.SourceGenerated.Extensions：
VertexDataBlob.cs、MeshHelper.cs、MeshExtensions.cs、SubMeshExtensions.cs、
CompressedMeshExtensions.cs、PackedFloatVectorExtensions.cs、PackedIntVectorExtensions.cs、UVInfo.cs。

上游许可证为 GNU GPL version 3；完整文本保留于仓库 licenses/AssetRipper-GPL-3.0.txt。
本移植保留上述许可。Rust 版本增加有界读取、溢出检查，并保留 UV 原始维数。
