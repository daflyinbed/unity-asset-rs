//! 自动生成。请勿手动修改。

use crate::subclasses::packed_bit_vector::PackedBitVector;

#[derive(Debug, Clone)]
pub struct CompressedMesh {
    pub m_vertices: PackedBitVector,
    pub m_uv: PackedBitVector,
    pub m_bind_poses: PackedBitVector,
    pub m_normals: PackedBitVector,
    pub m_tangents: PackedBitVector,
    pub m_weights: PackedBitVector,
    pub m_normal_signs: PackedBitVector,
    pub m_tangent_signs: PackedBitVector,
    pub m_bone_indices: PackedBitVector,
    pub m_triangles: PackedBitVector,
}

impl crate::Parse for CompressedMesh {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_vertices = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_uv = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_bind_poses = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_normals = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_tangents = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_weights = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_normal_signs = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_tangent_signs = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_bone_indices = PackedBitVector::parse(r, ver)?;
        r.align(4)?;
        let m_triangles = PackedBitVector::parse(r, ver)?;
        r.align(4)?;

        Ok(CompressedMesh {
            m_vertices,
            m_uv,
            m_bind_poses,
            m_normals,
            m_tangents,
            m_weights,
            m_normal_signs,
            m_tangent_signs,
            m_bone_indices,
            m_triangles,
        })
    }
}
