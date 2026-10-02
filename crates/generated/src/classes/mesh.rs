//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;
use crate::subclasses::blend_shape_data::BlendShapeData;
use crate::subclasses::bone_weights4::BoneWeights4;
use crate::subclasses::color_rgba32::ColorRgba32;
use crate::subclasses::compressed_mesh::CompressedMesh;
use crate::subclasses::matrix4x4f::Matrix4x4f;
use crate::subclasses::mesh_blend_shape::MeshBlendShape;
use crate::subclasses::mesh_blend_shape_vertex::MeshBlendShapeVertex;
use crate::subclasses::mesh_lod_info::MeshLodInfo;
use crate::subclasses::min_max_aabb::MinMaxAabb;
use crate::subclasses::streaming_info::StreamingInfo;
use crate::subclasses::sub_mesh::SubMesh;
use crate::subclasses::variable_bone_count_weights::VariableBoneCountWeights;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::vector4f::Vector4f;
use crate::subclasses::vertex_data::VertexData;

/// 版本枚举：MeshMSubMeshesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshMSubMeshesVariant {
    Default(Vec<SubMesh>),
    V2017_1(Vec<SubMesh>),
}

impl crate::Parse for MeshMSubMeshesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubMesh::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubMesh::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MeshMSubMeshesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：MeshMMeshCompressionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshMMeshCompressionVariant {
    Default(u8),
    V4_0(u8),
}

impl crate::Parse for MeshMMeshCompressionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MeshMMeshCompressionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：MeshMBindPoseVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshMBindPoseVariant {
    Default(Vec<Matrix4x4f>),
    V2017_1(Vec<Matrix4x4f>),
}

impl crate::Parse for MeshMBindPoseVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Matrix4x4f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Matrix4x4f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MeshMBindPoseVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：MeshMSkinVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshMSkinVariant {
    Default(Vec<BoneWeights4>),
    V2017_1(Vec<BoneWeights4>),
}

impl crate::Parse for MeshMSkinVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BoneWeights4::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BoneWeights4::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MeshMSkinVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：MeshMBoneNameHashesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshMBoneNameHashesVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for MeshMBoneNameHashesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MeshMBoneNameHashesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：MeshMMeshMetrics1Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshMMeshMetrics1Variant {
    Default(f32),
    V2018_3(f32),
}

impl crate::Parse for MeshMMeshMetrics1Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MeshMMeshMetrics1Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Mesh {
    pub m_name: String,
    pub m_use16_bit_indices: Option<i32>,
    pub m_sub_meshes: MeshMSubMeshesVariant,
    pub m_mesh_compression: MeshMMeshCompressionVariant,
    pub m_shapes_list: Option<Vec<MeshBlendShape>>,
    pub m_shapes: Option<BlendShapeData>,
    pub m_index_buffer: Vec<u8>,
    pub m_bind_pose: MeshMBindPoseVariant,
    pub m_stream_compression: Option<u8>,
    pub m_shape_vertices: Option<Vec<MeshBlendShapeVertex>>,
    pub m_skin: Option<MeshMSkinVariant>,
    pub m_is_readable: Option<bool>,
    pub m_bone_name_hashes: Option<MeshMBoneNameHashesVariant>,
    pub m_vertices: Option<Vec<Vector3f>>,
    pub m_keep_vertices: Option<bool>,
    pub m_root_bone_name_hash: Option<u32>,
    pub m_vertex_data: Option<VertexData>,
    pub m_keep_indices: Option<bool>,
    pub m_bones_aabb: Option<Vec<MinMaxAabb>>,
    pub m_compressed_mesh: CompressedMesh,
    pub m_variable_bone_count_weights: Option<VariableBoneCountWeights>,
    pub m_uv: Option<Vec<Vector2f>>,
    pub m_local_aabb: Aabb,
    pub m_uv1: Option<Vec<Vector2f>>,
    pub m_mesh_usage_flags: i32,
    pub m_tangents: Option<Vec<Vector4f>>,
    pub m_index_format: Option<i32>,
    pub m_normals: Option<Vec<Vector3f>>,
    pub m_colors: Option<Vec<ColorRgba32>>,
    pub m_collision_triangles: Option<Vec<u32>>,
    pub m_collision_vertex_count: Option<i32>,
    pub m_baked_convex_collision_mesh: Option<Vec<u8>>,
    pub m_baked_triangle_collision_mesh: Option<Vec<u8>>,
    pub m_mesh_metrics_0: Option<f32>,
    pub m_cooking_options: Option<i32>,
    pub m_mesh_metrics_1: Option<MeshMMeshMetrics1Variant>,
    pub m_stream_data: Option<StreamingInfo>,
    pub m_mesh_lod_info: Option<MeshLodInfo>,
}

impl crate::Parse for Mesh {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_use16_bit_indices = None;
        let mut m_sub_meshes = None;
        let mut m_mesh_compression = None;
        let mut m_shapes_list = None;
        let mut m_shapes = None;
        let mut m_index_buffer = None;
        let mut m_bind_pose = None;
        let mut m_stream_compression = None;
        let mut m_shape_vertices = None;
        let mut m_skin = None;
        let mut m_is_readable = None;
        let mut m_bone_name_hashes = None;
        let mut m_vertices = None;
        let mut m_keep_vertices = None;
        let mut m_root_bone_name_hash = None;
        let mut m_vertex_data = None;
        let mut m_keep_indices = None;
        let mut m_bones_aabb = None;
        let mut m_compressed_mesh = None;
        let mut m_variable_bone_count_weights = None;
        let mut m_uv = None;
        let mut m_local_aabb = None;
        let mut m_uv1 = None;
        let mut m_mesh_usage_flags = None;
        let mut m_tangents = None;
        let mut m_index_format = None;
        let mut m_normals = None;
        let mut m_colors = None;
        let mut m_collision_triangles = None;
        let mut m_collision_vertex_count = None;
        let mut m_baked_convex_collision_mesh = None;
        let mut m_baked_triangle_collision_mesh = None;
        let mut m_mesh_metrics_0 = None;
        let mut m_cooking_options = None;
        let mut m_mesh_metrics_1 = None;
        let mut m_stream_data = None;
        let mut m_mesh_lod_info = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_use16_bit_indices = Some(rdr.read_i32()?);
        }
        m_sub_meshes = Some(MeshMSubMeshesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_shapes = Some(BlendShapeData::parse(rdr, ver)?);
            m_bind_pose = Some(MeshMBindPoseVariant::parse(rdr, ver)?);
            m_bone_name_hashes = Some(MeshMBoneNameHashesVariant::parse(rdr, ver)?);
            m_root_bone_name_hash = Some(rdr.read_u32()?);
            m_bones_aabb = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| MinMaxAabb::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_variable_bone_count_weights = Some(VariableBoneCountWeights::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_shapes = Some(BlendShapeData::parse(rdr, ver)?);
            m_bind_pose = Some(MeshMBindPoseVariant::parse(rdr, ver)?);
            m_bone_name_hashes = Some(MeshMBoneNameHashesVariant::parse(rdr, ver)?);
            m_root_bone_name_hash = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_shapes_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| MeshBlendShape::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_shape_vertices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| MeshBlendShapeVertex::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }
        m_mesh_compression = Some(MeshMMeshCompressionVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            m_keep_vertices = Some(rdr.read_bool()?);
            m_keep_indices = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_index_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            m_keep_vertices = Some(rdr.read_bool()?);
            m_keep_indices = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_stream_compression = Some(rdr.read_u8()?);
            m_is_readable = Some(rdr.read_bool()?);
            m_keep_vertices = Some(rdr.read_bool()?);
            m_keep_indices = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_index_buffer = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_vertex_data = Some(VertexData::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_skin = Some(MeshMSkinVariant::parse(rdr, ver)?);
            m_vertex_data = Some(VertexData::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_skin = Some(MeshMSkinVariant::parse(rdr, ver)?);
            m_bind_pose = Some(MeshMBindPoseVariant::parse(rdr, ver)?);
            m_vertex_data = Some(VertexData::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
            m_vertices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector3f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_skin = Some(MeshMSkinVariant::parse(rdr, ver)?);
            m_bind_pose = Some(MeshMBindPoseVariant::parse(rdr, ver)?);
            m_uv = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector2f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_uv1 = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector2f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_tangents = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector4f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_normals = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector3f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }
        m_compressed_mesh = Some(CompressedMesh::parse(rdr, ver)?);
        m_local_aabb = Some(Aabb::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_colors = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| ColorRgba32::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_collision_triangles = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_collision_vertex_count = Some(rdr.read_i32()?);
        }
        m_mesh_usage_flags = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_cooking_options = Some(rdr.read_i32()?);
            m_baked_convex_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_triangle_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_mesh_metrics_0 = Some(rdr.read_f32()?);
            m_mesh_metrics_1 = Some(MeshMMeshMetrics1Variant::parse(rdr, ver)?);
            m_stream_data = Some(StreamingInfo::parse(rdr, ver)?);
            m_mesh_lod_info = Some(MeshLodInfo::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_cooking_options = Some(rdr.read_i32()?);
            m_baked_convex_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_triangle_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_mesh_metrics_0 = Some(rdr.read_f32()?);
            m_mesh_metrics_1 = Some(MeshMMeshMetrics1Variant::parse(rdr, ver)?);
            m_stream_data = Some(StreamingInfo::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_baked_convex_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_triangle_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_mesh_metrics_0 = Some(rdr.read_f32()?);
            m_mesh_metrics_1 = Some(MeshMMeshMetrics1Variant::parse(rdr, ver)?);
            m_stream_data = Some(StreamingInfo::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_baked_convex_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_triangle_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_mesh_metrics_0 = Some(rdr.read_f32()?);
            m_mesh_metrics_1 = Some(MeshMMeshMetrics1Variant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_baked_convex_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_triangle_collision_mesh = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }

        Ok(Mesh {
            m_name: m_name.unwrap(),
            m_use16_bit_indices,
            m_sub_meshes: m_sub_meshes.unwrap(),
            m_mesh_compression: m_mesh_compression.unwrap(),
            m_shapes_list,
            m_shapes,
            m_index_buffer: m_index_buffer.unwrap(),
            m_bind_pose: m_bind_pose.unwrap(),
            m_stream_compression,
            m_shape_vertices,
            m_skin,
            m_is_readable,
            m_bone_name_hashes,
            m_vertices,
            m_keep_vertices,
            m_root_bone_name_hash,
            m_vertex_data,
            m_keep_indices,
            m_bones_aabb,
            m_compressed_mesh: m_compressed_mesh.unwrap(),
            m_variable_bone_count_weights,
            m_uv,
            m_local_aabb: m_local_aabb.unwrap(),
            m_uv1,
            m_mesh_usage_flags: m_mesh_usage_flags.unwrap(),
            m_tangents,
            m_index_format,
            m_normals,
            m_colors,
            m_collision_triangles,
            m_collision_vertex_count,
            m_baked_convex_collision_mesh,
            m_baked_triangle_collision_mesh,
            m_mesh_metrics_0,
            m_cooking_options,
            m_mesh_metrics_1,
            m_stream_data,
            m_mesh_lod_info,
        })
    }
}

impl crate::ToValue for Mesh {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_Use16BitIndices", &self.m_use16_bit_indices),
            Some(("m_SubMeshes".to_string(), self.m_sub_meshes.to_value())),
            Some((
                "m_MeshCompression".to_string(),
                self.m_mesh_compression.to_value(),
            )),
            crate::opt_field("m_ShapesList", &self.m_shapes_list),
            crate::opt_field("m_Shapes", &self.m_shapes),
            Some(("m_IndexBuffer".to_string(), self.m_index_buffer.to_value())),
            Some(("m_BindPose".to_string(), self.m_bind_pose.to_value())),
            crate::opt_field("m_StreamCompression", &self.m_stream_compression),
            crate::opt_field("m_ShapeVertices", &self.m_shape_vertices),
            crate::opt_field("m_Skin", &self.m_skin),
            crate::opt_field("m_IsReadable", &self.m_is_readable),
            crate::opt_field("m_BoneNameHashes", &self.m_bone_name_hashes),
            crate::opt_field("m_Vertices", &self.m_vertices),
            crate::opt_field("m_KeepVertices", &self.m_keep_vertices),
            crate::opt_field("m_RootBoneNameHash", &self.m_root_bone_name_hash),
            crate::opt_field("m_VertexData", &self.m_vertex_data),
            crate::opt_field("m_KeepIndices", &self.m_keep_indices),
            crate::opt_field("m_BonesAABB", &self.m_bones_aabb),
            Some((
                "m_CompressedMesh".to_string(),
                self.m_compressed_mesh.to_value(),
            )),
            crate::opt_field(
                "m_VariableBoneCountWeights",
                &self.m_variable_bone_count_weights,
            ),
            crate::opt_field("m_UV", &self.m_uv),
            Some(("m_LocalAABB".to_string(), self.m_local_aabb.to_value())),
            crate::opt_field("m_UV1", &self.m_uv1),
            Some((
                "m_MeshUsageFlags".to_string(),
                self.m_mesh_usage_flags.to_value(),
            )),
            crate::opt_field("m_Tangents", &self.m_tangents),
            crate::opt_field("m_IndexFormat", &self.m_index_format),
            crate::opt_field("m_Normals", &self.m_normals),
            crate::opt_field("m_Colors", &self.m_colors),
            crate::opt_field("m_CollisionTriangles", &self.m_collision_triangles),
            crate::opt_field("m_CollisionVertexCount", &self.m_collision_vertex_count),
            crate::opt_field(
                "m_BakedConvexCollisionMesh",
                &self.m_baked_convex_collision_mesh,
            ),
            crate::opt_field(
                "m_BakedTriangleCollisionMesh",
                &self.m_baked_triangle_collision_mesh,
            ),
            crate::opt_field("m_MeshMetrics[0]", &self.m_mesh_metrics_0),
            crate::opt_field("m_CookingOptions", &self.m_cooking_options),
            crate::opt_field("m_MeshMetrics[1]", &self.m_mesh_metrics_1),
            crate::opt_field("m_StreamData", &self.m_stream_data),
            crate::opt_field("m_MeshLodInfo", &self.m_mesh_lod_info),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Use16BitIndices" => {
                    (self.m_use16_bit_indices).inspect_value(path, offset, limit)
                }
                "m_SubMeshes" => (self.m_sub_meshes).inspect_value(path, offset, limit),
                "m_MeshCompression" => (self.m_mesh_compression).inspect_value(path, offset, limit),
                "m_ShapesList" => (self.m_shapes_list).inspect_value(path, offset, limit),
                "m_Shapes" => (self.m_shapes).inspect_value(path, offset, limit),
                "m_IndexBuffer" => (self.m_index_buffer).inspect_value(path, offset, limit),
                "m_BindPose" => (self.m_bind_pose).inspect_value(path, offset, limit),
                "m_StreamCompression" => {
                    (self.m_stream_compression).inspect_value(path, offset, limit)
                }
                "m_ShapeVertices" => (self.m_shape_vertices).inspect_value(path, offset, limit),
                "m_Skin" => (self.m_skin).inspect_value(path, offset, limit),
                "m_IsReadable" => (self.m_is_readable).inspect_value(path, offset, limit),
                "m_BoneNameHashes" => (self.m_bone_name_hashes).inspect_value(path, offset, limit),
                "m_Vertices" => (self.m_vertices).inspect_value(path, offset, limit),
                "m_KeepVertices" => (self.m_keep_vertices).inspect_value(path, offset, limit),
                "m_RootBoneNameHash" => {
                    (self.m_root_bone_name_hash).inspect_value(path, offset, limit)
                }
                "m_VertexData" => (self.m_vertex_data).inspect_value(path, offset, limit),
                "m_KeepIndices" => (self.m_keep_indices).inspect_value(path, offset, limit),
                "m_BonesAABB" => (self.m_bones_aabb).inspect_value(path, offset, limit),
                "m_CompressedMesh" => (self.m_compressed_mesh).inspect_value(path, offset, limit),
                "m_VariableBoneCountWeights" => {
                    (self.m_variable_bone_count_weights).inspect_value(path, offset, limit)
                }
                "m_UV" => (self.m_uv).inspect_value(path, offset, limit),
                "m_LocalAABB" => (self.m_local_aabb).inspect_value(path, offset, limit),
                "m_UV1" => (self.m_uv1).inspect_value(path, offset, limit),
                "m_MeshUsageFlags" => (self.m_mesh_usage_flags).inspect_value(path, offset, limit),
                "m_Tangents" => (self.m_tangents).inspect_value(path, offset, limit),
                "m_IndexFormat" => (self.m_index_format).inspect_value(path, offset, limit),
                "m_Normals" => (self.m_normals).inspect_value(path, offset, limit),
                "m_Colors" => (self.m_colors).inspect_value(path, offset, limit),
                "m_CollisionTriangles" => {
                    (self.m_collision_triangles).inspect_value(path, offset, limit)
                }
                "m_CollisionVertexCount" => {
                    (self.m_collision_vertex_count).inspect_value(path, offset, limit)
                }
                "m_BakedConvexCollisionMesh" => {
                    (self.m_baked_convex_collision_mesh).inspect_value(path, offset, limit)
                }
                "m_BakedTriangleCollisionMesh" => {
                    (self.m_baked_triangle_collision_mesh).inspect_value(path, offset, limit)
                }
                "m_MeshMetrics[0]" => (self.m_mesh_metrics_0).inspect_value(path, offset, limit),
                "m_CookingOptions" => (self.m_cooking_options).inspect_value(path, offset, limit),
                "m_MeshMetrics[1]" => (self.m_mesh_metrics_1).inspect_value(path, offset, limit),
                "m_StreamData" => (self.m_stream_data).inspect_value(path, offset, limit),
                "m_MeshLodInfo" => (self.m_mesh_lod_info).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_use16_bit_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_Use16BitIndices".to_string(), v)),
            ((self.m_sub_meshes).inspect_value(path, offset, limit))
                .map(|v| ("m_SubMeshes".to_string(), v)),
            ((self.m_mesh_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshCompression".to_string(), v)),
            ((self.m_shapes_list).inspect_value(path, offset, limit))
                .map(|v| ("m_ShapesList".to_string(), v)),
            ((self.m_shapes).inspect_value(path, offset, limit))
                .map(|v| ("m_Shapes".to_string(), v)),
            ((self.m_index_buffer).inspect_value(path, offset, limit))
                .map(|v| ("m_IndexBuffer".to_string(), v)),
            ((self.m_bind_pose).inspect_value(path, offset, limit))
                .map(|v| ("m_BindPose".to_string(), v)),
            ((self.m_stream_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamCompression".to_string(), v)),
            ((self.m_shape_vertices).inspect_value(path, offset, limit))
                .map(|v| ("m_ShapeVertices".to_string(), v)),
            ((self.m_skin).inspect_value(path, offset, limit)).map(|v| ("m_Skin".to_string(), v)),
            ((self.m_is_readable).inspect_value(path, offset, limit))
                .map(|v| ("m_IsReadable".to_string(), v)),
            ((self.m_bone_name_hashes).inspect_value(path, offset, limit))
                .map(|v| ("m_BoneNameHashes".to_string(), v)),
            ((self.m_vertices).inspect_value(path, offset, limit))
                .map(|v| ("m_Vertices".to_string(), v)),
            ((self.m_keep_vertices).inspect_value(path, offset, limit))
                .map(|v| ("m_KeepVertices".to_string(), v)),
            ((self.m_root_bone_name_hash).inspect_value(path, offset, limit))
                .map(|v| ("m_RootBoneNameHash".to_string(), v)),
            ((self.m_vertex_data).inspect_value(path, offset, limit))
                .map(|v| ("m_VertexData".to_string(), v)),
            ((self.m_keep_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_KeepIndices".to_string(), v)),
            ((self.m_bones_aabb).inspect_value(path, offset, limit))
                .map(|v| ("m_BonesAABB".to_string(), v)),
            ((self.m_compressed_mesh).inspect_value(path, offset, limit))
                .map(|v| ("m_CompressedMesh".to_string(), v)),
            ((self.m_variable_bone_count_weights).inspect_value(path, offset, limit))
                .map(|v| ("m_VariableBoneCountWeights".to_string(), v)),
            ((self.m_uv).inspect_value(path, offset, limit)).map(|v| ("m_UV".to_string(), v)),
            ((self.m_local_aabb).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalAABB".to_string(), v)),
            ((self.m_uv1).inspect_value(path, offset, limit)).map(|v| ("m_UV1".to_string(), v)),
            ((self.m_mesh_usage_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshUsageFlags".to_string(), v)),
            ((self.m_tangents).inspect_value(path, offset, limit))
                .map(|v| ("m_Tangents".to_string(), v)),
            ((self.m_index_format).inspect_value(path, offset, limit))
                .map(|v| ("m_IndexFormat".to_string(), v)),
            ((self.m_normals).inspect_value(path, offset, limit))
                .map(|v| ("m_Normals".to_string(), v)),
            ((self.m_colors).inspect_value(path, offset, limit))
                .map(|v| ("m_Colors".to_string(), v)),
            ((self.m_collision_triangles).inspect_value(path, offset, limit))
                .map(|v| ("m_CollisionTriangles".to_string(), v)),
            ((self.m_collision_vertex_count).inspect_value(path, offset, limit))
                .map(|v| ("m_CollisionVertexCount".to_string(), v)),
            ((self.m_baked_convex_collision_mesh).inspect_value(path, offset, limit))
                .map(|v| ("m_BakedConvexCollisionMesh".to_string(), v)),
            ((self.m_baked_triangle_collision_mesh).inspect_value(path, offset, limit))
                .map(|v| ("m_BakedTriangleCollisionMesh".to_string(), v)),
            ((self.m_mesh_metrics_0).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshMetrics[0]".to_string(), v)),
            ((self.m_cooking_options).inspect_value(path, offset, limit))
                .map(|v| ("m_CookingOptions".to_string(), v)),
            ((self.m_mesh_metrics_1).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshMetrics[1]".to_string(), v)),
            ((self.m_stream_data).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamData".to_string(), v)),
            ((self.m_mesh_lod_info).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshLodInfo".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
