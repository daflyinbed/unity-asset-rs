//! 自动生成。请勿手动修改。

use crate::subclasses::packed_bit_vector_int32::PackedBitVector_Int32;
use crate::subclasses::packed_bit_vector_single::PackedBitVector_Single;

#[derive(Debug, Clone)]
pub struct CompressedMesh {
    pub m_vertices: PackedBitVector_Single,
    pub m_uv: PackedBitVector_Single,
    pub m_bind_poses: Option<PackedBitVector_Single>,
    pub m_normals: PackedBitVector_Single,
    pub m_tangents: PackedBitVector_Single,
    pub m_weights: PackedBitVector_Int32,
    pub m_normal_signs: PackedBitVector_Int32,
    pub m_tangent_signs: PackedBitVector_Int32,
    pub m_float_colors: Option<PackedBitVector_Single>,
    pub m_bone_indices: PackedBitVector_Int32,
    pub m_triangles: PackedBitVector_Int32,
    pub m_colors: Option<PackedBitVector_Int32>,
    pub m_uv_info: Option<u32>,
}

impl crate::Parse for CompressedMesh {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_vertices = None;
        let mut m_uv = None;
        let mut m_bind_poses = None;
        let mut m_normals = None;
        let mut m_tangents = None;
        let mut m_weights = None;
        let mut m_normal_signs = None;
        let mut m_tangent_signs = None;
        let mut m_float_colors = None;
        let mut m_bone_indices = None;
        let mut m_triangles = None;
        let mut m_colors = None;
        let mut m_uv_info = None;
            m_vertices = Some(PackedBitVector_Single::parse(rdr, ver)?);
            m_uv = Some(PackedBitVector_Single::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_bind_poses = Some(PackedBitVector_Single::parse(rdr, ver)?);
        }
            m_normals = Some(PackedBitVector_Single::parse(rdr, ver)?);
            m_tangents = Some(PackedBitVector_Single::parse(rdr, ver)?);
            m_weights = Some(PackedBitVector_Int32::parse(rdr, ver)?);
            m_normal_signs = Some(PackedBitVector_Int32::parse(rdr, ver)?);
            m_tangent_signs = Some(PackedBitVector_Int32::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_float_colors = Some(PackedBitVector_Single::parse(rdr, ver)?);
        } else {
        }
            m_bone_indices = Some(PackedBitVector_Int32::parse(rdr, ver)?);
            m_triangles = Some(PackedBitVector_Int32::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_uv_info = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_colors = Some(PackedBitVector_Int32::parse(rdr, ver)?);
        } else {
        }

        Ok(CompressedMesh {
            m_vertices: m_vertices.unwrap(),
            m_uv: m_uv.unwrap(),
            m_bind_poses,
            m_normals: m_normals.unwrap(),
            m_tangents: m_tangents.unwrap(),
            m_weights: m_weights.unwrap(),
            m_normal_signs: m_normal_signs.unwrap(),
            m_tangent_signs: m_tangent_signs.unwrap(),
            m_float_colors,
            m_bone_indices: m_bone_indices.unwrap(),
            m_triangles: m_triangles.unwrap(),
            m_colors,
            m_uv_info,
        })
    }
}
