//! 自动生成。请勿手动修改。

use crate::subclasses::packed_bit_vector::PackedBitVector;

#[derive(Debug, Clone)]
pub struct CompressedAnimationCurve {
    pub m_path: String,
    pub m_times: PackedBitVector,
    pub m_values: PackedBitVector,
    pub m_slopes: PackedBitVector,
    pub m_pre_infinity: i32,
    pub m_post_infinity: i32,
}

impl crate::Parse for CompressedAnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_path = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_times = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_values = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_slopes = PackedBitVector::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_pre_infinity = rdr.read_i32()?;
        let m_post_infinity = rdr.read_i32()?;

        Ok(CompressedAnimationCurve {
            m_path,
            m_times,
            m_values,
            m_slopes,
            m_pre_infinity,
            m_post_infinity,
        })
    }
}
