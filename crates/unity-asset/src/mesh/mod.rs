//! Unity 几何解码。算法移植自 AssetRipper（来源与许可证见 NOTICE.md）。
//! 保留 Unity 坐标与 SubMesh，不生成模型文件、不合成缺失法线。
mod compressed;
mod vertex;

use crate::{Endian, Environment, EnvironmentObject, ParsedObject, UnityVersion};
#[cfg(feature = "export")]
use generated::subclasses::sprite_render_data::*;
use generated::{UnityObject, classes::mesh::*, subclasses::sub_mesh::SubMesh};
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(display("Mesh 解码失败: {reason}"))]
pub struct MeshDecodeError {
    pub reason: String,
}
type Result<T> = std::result::Result<T, MeshDecodeError>;
pub(crate) fn invalid(reason: impl Into<String>) -> MeshDecodeError {
    MeshDecodeError {
        reason: reason.into(),
    }
}
pub(crate) fn check(ok: bool, reason: &str) -> Result<()> {
    if ok { Ok(()) } else { Err(invalid(reason)) }
}

/// UV 保留 1..4 维；每个顶点剩余分量为零。
#[derive(Debug, Clone)]
pub struct TexCoordSet {
    pub dimension: u8,
    pub values: Vec<[f32; 4]>,
}
#[derive(Debug, Clone, Default)]
pub struct SkinWeight {
    pub weights: [f32; 4],
    pub indices: [u32; 4],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshTopology {
    Triangles,
    TriangleStrip,
    Quads,
    Lines,
    LineStrip,
    Points,
}
#[derive(Debug, Clone)]
pub struct DecodedSubMesh {
    pub topology: MeshTopology,
    pub indices: Vec<u32>,
}
#[derive(Debug, Clone, Default)]
pub struct DecodedMesh {
    pub positions: Vec<[f32; 3]>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub tangents: Option<Vec<[f32; 4]>>,
    pub colors: Option<Vec<[f32; 4]>>,
    pub tex_coords: [Option<TexCoordSet>; 8],
    pub skin: Option<Vec<SkinWeight>>,
    /// Unity 列主序矩阵，未执行坐标系转换。
    pub bind_poses: Vec<[f32; 16]>,
    pub submeshes: Vec<DecodedSubMesh>,
}
impl DecodedSubMesh {
    /// 三角形、条带、四边形统一为三角形；线/点不能用于 Sprite 光栅化。
    pub fn triangles(&self) -> Result<Vec<[u32; 3]>> {
        let mut out = Vec::new();
        match self.topology {
            MeshTopology::Triangles => {
                check(
                    self.indices.len().is_multiple_of(3),
                    "三角形索引数不是 3 的倍数",
                )?;
                for c in self.indices.chunks_exact(3) {
                    out.push([c[0], c[1], c[2]]);
                }
            }
            MeshTopology::TriangleStrip => {
                for (i, c) in self.indices.windows(3).enumerate() {
                    if c[0] != c[1] && c[1] != c[2] && c[0] != c[2] {
                        out.push(if i % 2 == 0 {
                            [c[0], c[1], c[2]]
                        } else {
                            [c[1], c[0], c[2]]
                        });
                    }
                }
            }
            MeshTopology::Quads => {
                check(
                    self.indices.len().is_multiple_of(4),
                    "四边形索引数不是 4 的倍数",
                )?;
                for c in self.indices.chunks_exact(4) {
                    out.extend([[c[0], c[1], c[2]], [c[0], c[2], c[3]]]);
                }
            }
            _ => return Err(invalid("线/点拓扑不能转换为三角形")),
        }
        Ok(out)
    }
}
fn read_indices(data: &[u8], size: usize, endian: Endian) -> Result<Vec<u32>> {
    check(data.len().is_multiple_of(size), "index buffer 长度未对齐")?;
    Ok(data
        .chunks_exact(size)
        .map(|b| vertex::uint(b, endian))
        .collect())
}
fn submeshes(
    indices: &[u32],
    subs: &[SubMesh],
    size: usize,
    vertices: usize,
) -> Result<Vec<DecodedSubMesh>> {
    subs.iter()
        .map(|s| {
            let topology = match s
                .topology
                .unwrap_or(i32::from(s.is_tri_strip.unwrap_or(0) != 0))
            {
                0 => MeshTopology::Triangles,
                1 => MeshTopology::TriangleStrip,
                2 => MeshTopology::Quads,
                3 => MeshTopology::Lines,
                4 => MeshTopology::LineStrip,
                5 => MeshTopology::Points,
                n => return Err(invalid(format!("未知拓扑 {n}"))),
            };
            check(
                (s.first_byte as usize).is_multiple_of(size),
                "SubMesh firstByte 未对齐",
            )?;
            let start = s.first_byte as usize / size;
            let end = start
                .checked_add(s.index_count as usize)
                .ok_or_else(|| invalid("索引范围溢出"))?;
            let values = indices
                .get(start..end)
                .ok_or_else(|| invalid("SubMesh 索引范围越界"))?
                .iter()
                .map(|i| {
                    let i = i
                        .checked_add(s.base_vertex.unwrap_or(0))
                        .ok_or_else(|| invalid("baseVertex 溢出"))?;
                    check((i as usize) < vertices, "顶点索引越界")?;
                    Ok(i)
                })
                .collect::<Result<Vec<_>>>()?;
            if topology == MeshTopology::Lines {
                check(values.len().is_multiple_of(2), "线段索引数不是 2 的倍数")?;
            }
            let out = DecodedSubMesh {
                topology,
                indices: values,
            };
            if matches!(topology, MeshTopology::Triangles | MeshTopology::Quads) {
                out.triangles()?;
            }
            Ok(out)
        })
        .collect()
}
impl Environment {
    /// 解码已加载的 typed Mesh；外部顶点流经 Environment 资源表读取。
    pub fn decode_mesh(&self, object: &EnvironmentObject) -> Result<DecodedMesh> {
        let ParsedObject::Typed(UnityObject::Mesh(mesh)) = &object.object else {
            return Err(invalid("对象不是 typed Mesh"));
        };
        let col = self
            .collections()
            .get(object.collection.to_index())
            .ok_or_else(|| invalid("collection 不存在"))?;
        decode_mesh(self, mesh, col.unity_version, col.endian)
    }
}
fn decode_mesh(
    env: &Environment,
    m: &Mesh,
    version: UnityVersion,
    endian: Endian,
) -> Result<DecodedMesh> {
    let mut out = DecodedMesh::default();
    if let Some(v) = &m.m_vertices {
        out.positions = v.iter().map(|v| [v.x, v.y, v.z]).collect();
    }
    // 旧版字段存在但数组为空表示未提供属性，不能阻止压缩数据回退。
    out.normals = m
        .m_normals
        .as_ref()
        .filter(|v| !v.is_empty())
        .map(|v| v.iter().map(|v| [v.x, v.y, v.z]).collect());
    out.tangents = m
        .m_tangents
        .as_ref()
        .filter(|v| !v.is_empty())
        .map(|v| v.iter().map(|v| [v.x, v.y, v.z, v.w]).collect());
    out.colors = m.m_colors.as_ref().filter(|v| !v.is_empty()).map(|v| {
        v.iter()
            .map(|v| v.rgba.to_le_bytes().map(|c| c as f32 / 255.))
            .collect()
    });
    for (i, uv) in [&m.m_uv, &m.m_uv1].into_iter().enumerate() {
        if let Some(uv) = uv.as_ref().filter(|v| !v.is_empty()) {
            out.tex_coords[i] = Some(TexCoordSet {
                dimension: 2,
                values: uv.iter().map(|v| [v.x, v.y, 0., 0.]).collect(),
            });
        }
    }
    if let Some(vd) = &m.m_vertex_data {
        let external =
            if vd.m_data.is_empty() {
                if let Some(sd) = &m.m_stream_data {
                    if !sd.path.is_empty() {
                        Some(env.streaming_data(sd).ok_or_else(|| {
                            invalid(format!("外部顶点数据缺失或越界: {}", sd.path))
                        })?)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
        vertex::decode(
            vd,
            external.as_deref().unwrap_or(&vd.m_data),
            version,
            endian,
            &mut out,
        )?;
    }
    let packed_indices = compressed::decode(&m.m_compressed_mesh, &mut out)?;
    if out.skin.is_none()
        && let Some(MeshMSkinVariant::Default(v) | MeshMSkinVariant::V2017_1(v)) = &m.m_skin
        && !v.is_empty()
    {
        out.skin = Some(
            v.iter()
                .map(|b| SkinWeight {
                    weights: [b.weight_0, b.weight_1, b.weight_2, b.weight_3],
                    indices: [
                        b.bone_index_0 as u32,
                        b.bone_index_1 as u32,
                        b.bone_index_2 as u32,
                        b.bone_index_3 as u32,
                    ],
                })
                .collect(),
        );
    }
    if out.bind_poses.is_empty() {
        let (MeshMBindPoseVariant::Default(v) | MeshMBindPoseVariant::V2017_1(v)) = &m.m_bind_pose;
        out.bind_poses = v.iter().map(matrix).collect();
    }
    let size = match m.m_index_format {
        Some(0) => 2,
        Some(1) => 4,
        Some(n) => return Err(invalid(format!("未知 index format {n}"))),
        None => {
            if m.m_use16_bit_indices == Some(0) {
                4
            } else {
                2
            }
        }
    };
    let indices = match packed_indices {
        Some(v) => v,
        None => read_indices(&m.m_index_buffer, size, endian)?,
    };
    let (MeshMSubMeshesVariant::Default(subs) | MeshMSubMeshesVariant::V2017_1(subs)) =
        &m.m_sub_meshes;
    out.submeshes = submeshes(&indices, subs, size, out.positions.len())?;
    validate(&out)?;
    Ok(out)
}
fn matrix(m: &generated::subclasses::matrix4x4f::Matrix4x4f) -> [f32; 16] {
    [
        m.e00, m.e10, m.e20, m.e30, m.e01, m.e11, m.e21, m.e31, m.e02, m.e12, m.e22, m.e32, m.e03,
        m.e13, m.e23, m.e33,
    ]
}
fn validate(m: &DecodedMesh) -> Result<()> {
    let n = m.positions.len();
    for count in [
        m.normals.as_ref().map(Vec::len),
        m.tangents.as_ref().map(Vec::len),
        m.colors.as_ref().map(Vec::len),
        m.skin.as_ref().map(Vec::len),
    ]
    .into_iter()
    .flatten()
    .chain(m.tex_coords.iter().flatten().map(|v| v.values.len()))
    {
        check(count == n, "顶点属性数量不一致")?;
    }
    check(
        m.positions.iter().flatten().all(|x| x.is_finite()),
        "位置包含非有限浮点",
    )
}
#[cfg(feature = "export")]
pub(crate) fn decode_sprite(
    rd: &SpriteRenderData,
    version: UnityVersion,
    endian: Endian,
) -> Result<DecodedMesh> {
    let mut out = DecodedMesh::default();
    if let Some(
        SpriteRenderDataMVertexDataVariant::Default(v)
        | SpriteRenderDataMVertexDataVariant::V2018_1(v),
    ) = &rd.m_vertex_data
    {
        vertex::decode(v, &v.m_data, version, endian, &mut out)?;
    } else if let Some(v) = &rd.vertices {
        out.positions = v.iter().map(|v| [v.pos.x, v.pos.y, v.pos.z]).collect();
        if v.iter().all(|v| v.uv.is_some()) {
            out.tex_coords[0] = Some(TexCoordSet {
                dimension: 2,
                values: v
                    .iter()
                    .map(|v| {
                        let uv = v.uv.as_ref().unwrap();
                        [uv.x, uv.y, 0., 0.]
                    })
                    .collect(),
            });
        }
    }
    let indices = if let Some(b) = &rd.m_index_buffer {
        read_indices(b, 2, endian)?
    } else {
        rd.indices
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|i| *i as u32)
            .collect()
    };
    if let Some(
        SpriteRenderDataMSubMeshesVariant::Default(s)
        | SpriteRenderDataMSubMeshesVariant::V2017_1(s),
    ) = &rd.m_sub_meshes
    {
        out.submeshes = submeshes(&indices, s, 2, out.positions.len())?;
    } else {
        check(
            indices.iter().all(|i| (*i as usize) < out.positions.len()),
            "Sprite 索引越界",
        )?;
        let s = DecodedSubMesh {
            topology: MeshTopology::Triangles,
            indices,
        };
        s.triangles()?;
        out.submeshes.push(s);
    }
    validate(&out)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn topology_and_index_endian() {
        assert_eq!(read_indices(&[0, 1, 0, 2], 2, Endian::Big).unwrap(), [1, 2]);
        assert_eq!(
            read_indices(&[1, 0, 1, 0], 4, Endian::Little).unwrap(),
            [65537]
        );
        let s = DecodedSubMesh {
            topology: MeshTopology::TriangleStrip,
            indices: vec![0, 1, 2, 3],
        };
        assert_eq!(s.triangles().unwrap(), [[0, 1, 2], [2, 1, 3]]);
        let s = DecodedSubMesh {
            topology: MeshTopology::Quads,
            indices: vec![0, 1, 2, 3],
        };
        assert_eq!(s.triangles().unwrap(), [[0, 1, 2], [0, 2, 3]]);
        assert!(
            DecodedSubMesh {
                topology: MeshTopology::Triangles,
                indices: vec![0, 1]
            }
            .triangles()
            .is_err()
        );
    }
    #[test]
    fn base_vertex_and_first_byte() {
        use generated::subclasses::{aabb::Aabb, vector3f::Vector3f};
        let s = SubMesh {
            first_byte: 2,
            index_count: 3,
            is_tri_strip: None,
            topology: Some(0),
            triangle_count: None,
            first_vertex: 99,
            base_vertex: Some(2),
            vertex_count: 3,
            local_aabb: Aabb {
                m_center: Vector3f {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                m_extent: Vector3f {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
            },
        };
        assert_eq!(
            submeshes(&[99, 0, 1, 2], std::slice::from_ref(&s), 2, 5).unwrap()[0].indices,
            [2, 3, 4]
        );
        assert!(submeshes(&[99, 0, 1, 2], std::slice::from_ref(&s), 2, 4).is_err());
        let mut bad = s;
        bad.first_byte = 1;
        assert!(submeshes(&[0, 1, 2], &[bad], 2, 5).is_err());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::{
        AssetCollection, Bytes, ClassId, CollectionId, ProviderError, ResourceProvider,
        ResourceRequest,
    };
    use generated::{
        Parse,
        subclasses::{
            channel_info::ChannelInfo,
            streaming_info::{StreamingInfo, StreamingInfoOffsetVariant},
            sub_mesh::SubMesh,
            vector3f::Vector3f,
            vertex_data::{VertexData, VertexDataMChannelsVariant},
        },
    };
    use std::sync::Arc;
    fn fixture(version: UnityVersion) -> (Environment, Mesh, Vec<u8>) {
        // 零值序列化构造完整 generated 对象，测试只填目标字段。
        let zeros = vec![0; 4096];
        let mut cursor = crate::cursor::Cursor::new(&zeros, Endian::Little);
        let mut mesh = Mesh::parse(&mut cursor, &version).unwrap();
        let data: Vec<u8> = [0f32, 0., 0., 1., 0., 0., 0., 1., 0.]
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect();
        let vertex_data = VertexData {
            m_current_channels: None,
            m_vertex_count: 3,
            m_channels: Some(VertexDataMChannelsVariant::V2017_1(vec![ChannelInfo {
                stream: 0,
                offset: 0,
                format: 0,
                dimension: 3,
            }])),
            m_streams_0: None,
            m_streams_1: None,
            m_streams_2: None,
            m_streams_3: None,
            m_streams: None,
            m_data: data.clone(),
        };
        if mesh.m_vertex_data.is_some() {
            mesh.m_vertex_data = Some(vertex_data);
        } else {
            mesh.m_vertices = Some(
                [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]
                    .into_iter()
                    .map(|[x, y, z]| Vector3f { x, y, z })
                    .collect(),
            );
            mesh.m_use16_bit_indices = Some(1);
        }
        mesh.m_index_buffer = [0u16, 1, 2]
            .into_iter()
            .flat_map(u16::to_le_bytes)
            .collect();
        let mut cursor = crate::cursor::Cursor::new(&zeros, Endian::Little);
        let mut sub = SubMesh::parse(&mut cursor, &version).unwrap();
        sub.index_count = 3;
        sub.vertex_count = 3;
        let (MeshMSubMeshesVariant::Default(subs) | MeshMSubMeshesVariant::V2017_1(subs)) =
            &mut mesh.m_sub_meshes;
        subs.push(sub);
        let mut env = Environment::empty();
        env.collections.push(AssetCollection {
            name: "mesh.assets".into(),
            name_fixed: "mesh.assets".into(),
            unity_version: version,
            endian: Endian::Little,
            dependencies: vec![Some(CollectionId::from_index(0))],
            assets: Default::default(),
            externals: vec![],
        });
        (env, mesh, data)
    }
    #[test]
    fn legacy_empty_attributes_are_absent() {
        for version in [UnityVersion::new(3, 4, 0), UnityVersion::new(2017, 4, 0)] {
            let (env, mesh, _) = fixture(version);
            let decoded = decode_mesh(&env, &mesh, version, Endian::Little).unwrap();
            assert_eq!(decoded.positions.len(), 3);
            assert_eq!(decoded.submeshes[0].triangles().unwrap(), [[0, 1, 2]]);
            assert!(decoded.skin.is_none());
            assert!(decoded.normals.is_none());
            assert!(decoded.tangents.is_none());
            assert!(decoded.colors.is_none());
            assert!(decoded.tex_coords.iter().all(Option::is_none));
        }
    }
    #[test]
    fn legacy_empty_attributes_fall_back_to_compressed_data() {
        use generated::subclasses::{
            packed_bit_vector_int32::PackedBitVector_Int32,
            packed_bit_vector_single::PackedBitVector_Single,
        };
        let version = UnityVersion::new(3, 4, 0);
        let (env, mut mesh, _) = fixture(version);
        let constant = |count, value| PackedBitVector_Single {
            m_num_items: count,
            m_bit_size: 0,
            m_data: vec![],
            m_start: value,
            m_range: 0.,
        };
        let c = &mut mesh.m_compressed_mesh;
        c.m_normals = constant(6, 0.);
        c.m_normal_signs = PackedBitVector_Int32 {
            m_num_items: 3,
            m_bit_size: 1,
            m_data: vec![0b111],
        };
        c.m_tangents = constant(6, 0.);
        c.m_tangent_signs = PackedBitVector_Int32 {
            m_num_items: 6,
            m_bit_size: 1,
            m_data: vec![0b111111],
        };
        c.m_colors = Some(PackedBitVector_Int32 {
            m_num_items: 3,
            m_bit_size: 32,
            m_data: [255, 0, 0, 255].repeat(3),
        });
        c.m_uv = constant(12, 0.25);
        let decoded = decode_mesh(&env, &mesh, version, Endian::Little).unwrap();
        assert_eq!(decoded.normals.unwrap(), [[0., 0., 1.]; 3]);
        assert_eq!(decoded.tangents.unwrap(), [[0., 0., 1., 1.]; 3]);
        assert_eq!(decoded.colors.unwrap(), [[1., 0., 0., 1.]; 3]);
        for uv in &decoded.tex_coords[..2] {
            let uv = uv.as_ref().unwrap();
            assert_eq!(uv.dimension, 2);
            assert_eq!(uv.values, [[0.25, 0.25, 0., 0.]; 3]);
        }
        // 非空但长度错误的属性仍然拒绝，不能用压缩回退掩盖损坏数据。
        mesh.m_normals = Some(vec![Vector3f {
            x: 1.,
            y: 0.,
            z: 0.,
        }]);
        assert!(decode_mesh(&env, &mesh, version, Endian::Little).is_err());
    }
    #[test]
    fn compressed_bind_poses_match_resident_column_major_layout() {
        use generated::subclasses::{
            matrix4x4f::Matrix4x4f, packed_bit_vector_single::PackedBitVector_Single,
        };
        let version = UnityVersion::new(4, 7, 0);
        let (env, mut mesh, _) = fixture(version);
        // 平移和非对称元素揭示转置错误；两个矩阵验证逐矩阵处理。
        let raw = [
            [1u8, 2, 3, 5, 0, 1, 4, 6, 0, 0, 1, 7, 0, 0, 0, 1],
            [2, 0, 0, 10, 0, 3, 0, 20, 0, 0, 4, 30, 0, 0, 0, 1],
        ];
        let (MeshMBindPoseVariant::Default(poses) | MeshMBindPoseVariant::V2017_1(poses)) =
            &mut mesh.m_bind_pose;
        for values in raw {
            let bytes: Vec<u8> = values
                .into_iter()
                .flat_map(|v| (v as f32).to_le_bytes())
                .collect();
            let mut cursor = crate::cursor::Cursor::new(&bytes, Endian::Little);
            poses.push(Matrix4x4f::parse(&mut cursor, &version).unwrap());
        }
        let resident = decode_mesh(&env, &mesh, version, Endian::Little).unwrap();
        assert_eq!(
            resident.bind_poses,
            [
                [1., 0., 0., 0., 2., 1., 0., 0., 3., 4., 1., 0., 5., 6., 7., 1.],
                [2., 0., 0., 0., 0., 3., 0., 0., 0., 0., 4., 0., 10., 20., 30., 1.],
            ]
        );
        let (MeshMBindPoseVariant::Default(poses) | MeshMBindPoseVariant::V2017_1(poses)) =
            &mut mesh.m_bind_pose;
        poses.clear();
        mesh.m_compressed_mesh.m_bind_poses = Some(PackedBitVector_Single {
            m_num_items: 32,
            m_bit_size: 8,
            m_data: raw.into_iter().flatten().collect(),
            m_start: 0.,
            m_range: 255.,
        });
        let packed = decode_mesh(&env, &mesh, version, Endian::Little).unwrap();
        assert_eq!(packed.bind_poses, resident.bind_poses);
    }
    struct Provider(Vec<u8>);
    impl ResourceProvider for Provider {
        fn open(
            &self,
            req: &ResourceRequest<'_>,
        ) -> std::result::Result<Option<Bytes>, ProviderError> {
            assert_eq!(req.original_path, "mesh.resS");
            Ok(Some(Bytes::copy_from_slice(&self.0)))
        }
    }
    #[test]
    fn environment_mesh_and_external_provider() {
        let (mut env, mut mesh, data) = fixture(UnityVersion::new(2020, 3, 0));
        let object = |mesh| EnvironmentObject {
            collection: CollectionId::from_index(0),
            path_id: 1,
            class_id: ClassId::from(generated::ClassIDType::Mesh),
            object: ParsedObject::Typed(UnityObject::Mesh(mesh)),
        };
        let resident = env.decode_mesh(&object(mesh.clone())).unwrap();
        assert_eq!(
            resident.positions,
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]
        );
        assert_eq!(resident.submeshes[0].triangles().unwrap(), [[0, 1, 2]]);
        mesh.m_vertex_data.as_mut().unwrap().m_data.clear();
        mesh.m_stream_data = Some(StreamingInfo {
            offset: StreamingInfoOffsetVariant::V2020_1(4),
            size: data.len() as u32,
            path: "mesh.resS".into(),
        });
        assert!(env.decode_mesh(&object(mesh.clone())).is_err());
        let mut resource = vec![99; 4];
        resource.extend(data);
        env.resource_provider = Some(Arc::new(Provider(resource)));
        env.objects.push(object(mesh));
        env.prefetch_resources();
        let external = env.decode_mesh(&env.objects[0]).unwrap();
        assert_eq!(external.positions, resident.positions);
    }
}
