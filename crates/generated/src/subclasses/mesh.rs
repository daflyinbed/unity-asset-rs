//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use crate::subclasses::bone_influence::BoneInfluence;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::compressed_mesh::CompressedMesh;
use crate::subclasses::matrix4x4f::Matrix4x4f;
use crate::subclasses::sub_mesh::SubMesh;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector3f::Vector3f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_use16_bit_indices: i32,
    pub m_sub_meshes: Vec<SubMesh>,
    pub m_mesh_compression: u8,
    pub m_index_buffer: Vec<u8>,
    pub m_vertices: Vec<Vector3f>,
    pub m_skin: Vec<BoneInfluence>,
    pub m_bind_pose: Vec<Matrix4x4f>,
    pub m_uv: Vec<Vector2f>,
    pub m_uv1: Vec<Vector2f>,
    pub m_tangents: Vec<Vector4f>,
    pub m_normals: Vec<Vector3f>,
    pub m_compressed_mesh: CompressedMesh,
    pub m_local_aabb: Aabb,
    pub m_colors: Vec<ColorRgba>,
    pub m_collision_triangles: Vec<u32>,
    pub m_collision_vertex_count: i32,
    pub m_mesh_usage_flags: i32,
}

impl crate::Parse for Mesh {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_use16_bit_indices = r.read_i32()?;
        let m_sub_meshes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubMesh::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_mesh_compression = r.read_u8()?;
        r.align(4)?;
        let m_index_buffer = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_vertices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_skin = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BoneInfluence::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_bind_pose = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Matrix4x4f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_uv = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_uv1 = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_tangents = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector4f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_normals = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_compressed_mesh = CompressedMesh::parse(r, ver)?;
        r.align(4)?;
        let m_local_aabb = Aabb::parse(r, ver)?;
        let m_colors = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ColorRgba::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_collision_triangles = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_collision_vertex_count = r.read_i32()?;
        let m_mesh_usage_flags = r.read_i32()?;

        Ok(Mesh {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_use16_bit_indices,
            m_sub_meshes,
            m_mesh_compression,
            m_index_buffer,
            m_vertices,
            m_skin,
            m_bind_pose,
            m_uv,
            m_uv1,
            m_tangents,
            m_normals,
            m_compressed_mesh,
            m_local_aabb,
            m_colors,
            m_collision_triangles,
            m_collision_vertex_count,
            m_mesh_usage_flags,
        })
    }
}
