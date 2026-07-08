//! 自动生成。请勿手动修改。

use crate::subclasses::packed_bit_vector_int32::PackedBitVector_Int32;
use crate::subclasses::packed_bit_vector_quaternionf::PackedBitVector_Quaternionf;
use crate::subclasses::packed_bit_vector_single::PackedBitVector_Single;

#[derive(Debug, Clone)]
pub struct CompressedAnimationCurve {
    pub m_path: String,
    pub m_times: PackedBitVector_Int32,
    pub m_values: PackedBitVector_Quaternionf,
    pub m_slopes: PackedBitVector_Single,
    pub m_pre_infinity: i32,
    pub m_post_infinity: i32,
}

impl crate::Parse for CompressedAnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_path = rdr.read_aligned_string()?;
        let m_times = PackedBitVector_Int32::parse(rdr, ver)?;
        let m_values = PackedBitVector_Quaternionf::parse(rdr, ver)?;
        let m_slopes = PackedBitVector_Single::parse(rdr, ver)?;
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
