//! 自动生成。请勿手动修改。

use crate::subclasses::packed_bit_vector::PackedBitVector;

#[derive(Debug, Clone)]
pub struct CompressedMesh {
    pub m_vertices: PackedBitVector,
    pub m_uv: PackedBitVector,
    pub m_bind_poses: Option<PackedBitVector>,
    pub m_normals: PackedBitVector,
    pub m_tangents: PackedBitVector,
    pub m_weights: PackedBitVector,
    pub m_normal_signs: PackedBitVector,
    pub m_tangent_signs: PackedBitVector,
    pub m_float_colors: Option<PackedBitVector>,
    pub m_bone_indices: PackedBitVector,
    pub m_triangles: PackedBitVector,
    pub m_colors: Option<PackedBitVector>,
    pub m_uv_info: Option<u32>,
}

impl crate::Parse for CompressedMesh {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_vertices = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_uv = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_bind_poses = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = PackedBitVector::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_normals = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_tangents = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_weights = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_normal_signs = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_tangent_signs = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_float_colors = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = PackedBitVector::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_bone_indices = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_triangles = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_colors = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = PackedBitVector::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_uv_info = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };

        Ok(CompressedMesh {
            m_vertices,
            m_uv,
            m_bind_poses,
            m_normals,
            m_tangents,
            m_weights,
            m_normal_signs,
            m_tangent_signs,
            m_float_colors,
            m_bone_indices,
            m_triangles,
            m_colors,
            m_uv_info,
        })
    }
}
