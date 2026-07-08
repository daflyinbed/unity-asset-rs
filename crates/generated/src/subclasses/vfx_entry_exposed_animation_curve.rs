//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct VfxEntryExposed_AnimationCurve {
    pub m_value: AnimationCurve,
    pub m_name: String,
    pub m_overridden: bool,
}

impl crate::Parse for VfxEntryExposed_AnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value = AnimationCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_name = rdr.read_aligned_string()?;
        let m_overridden = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxEntryExposed_AnimationCurve {
            m_value,
            m_name,
            m_overridden,
        })
    }
}
