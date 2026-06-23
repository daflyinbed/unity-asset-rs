//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;
use crate::subclasses::bone_influence::BoneInfluence;
use crate::subclasses::color_rgba::ColorRgba;
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
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_use16_bit_indices: Option<i32>,
    pub m_sub_meshes: Vec<SubMesh>,
    pub m_mesh_compression: u8,
    pub m_shapes: Option<Vec<MeshBlendShape>>,
    pub m_index_buffer: Vec<u8>,
    pub m_bind_pose: Vec<Matrix4x4f>,
    pub m_stream_compression: Option<u8>,
    pub m_shape_vertices: Option<Vec<MeshBlendShapeVertex>>,
    pub m_skin: Option<Vec<BoneInfluence>>,
    pub m_is_readable: Option<bool>,
    pub m_bone_name_hashes: Option<Vec<u32>>,
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
    pub m_colors: Option<Vec<ColorRgba>>,
    pub m_collision_triangles: Option<Vec<u32>>,
    pub m_collision_vertex_count: Option<i32>,
    pub m_baked_convex_collision_mesh: Option<Vec<u8>>,
    pub m_baked_triangle_collision_mesh: Option<Vec<u8>>,
    pub m_mesh_metrics_0: Option<f32>,
    pub m_cooking_options: Option<i32>,
    pub m_mesh_metrics_1: Option<f32>,
    pub m_stream_data: Option<StreamingInfo>,
    pub m_mesh_lod_info: Option<MeshLodInfo>,
}

impl crate::Parse for Mesh {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_use16_bit_indices = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sub_meshes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubMesh::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_mesh_compression = r.read_u8()?;
        r.align(4)?;
        let m_shapes = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MeshBlendShape::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_index_buffer = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_bind_pose = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Matrix4x4f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_stream_compression = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_u8()?)
        } else {
            None
        };
        let m_shape_vertices = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { MeshBlendShapeVertex::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_skin = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { BoneInfluence::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_is_readable = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_bone_name_hashes = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_vertices = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_keep_vertices = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_root_bone_name_hash = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_vertex_data = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = VertexData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_keep_indices = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_bones_aabb = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MinMaxAabb::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_compressed_mesh = CompressedMesh::parse(r, ver)?;
        r.align(4)?;
        let m_variable_bone_count_weights = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = VariableBoneCountWeights::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_uv = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_local_aabb = Aabb::parse(r, ver)?;
        let m_uv1 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_mesh_usage_flags = r.read_i32()?;
        let m_tangents = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector4f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_index_format = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_normals = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_colors = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ColorRgba::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_collision_triangles = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_collision_vertex_count = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_baked_convex_collision_mesh = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_baked_triangle_collision_mesh = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mesh_metrics_0 = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_cooking_options = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_mesh_metrics_1 = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_stream_data = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = StreamingInfo::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mesh_lod_info = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = MeshLodInfo::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Mesh {
            parent,
            m_name,
            m_use16_bit_indices,
            m_sub_meshes,
            m_mesh_compression,
            m_shapes,
            m_index_buffer,
            m_bind_pose,
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
            m_compressed_mesh,
            m_variable_bone_count_weights,
            m_uv,
            m_local_aabb,
            m_uv1,
            m_mesh_usage_flags,
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
