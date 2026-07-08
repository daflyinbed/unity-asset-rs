//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_AnimationCurve {
    pub m_expression_index: u32,
    pub m_value: AnimationCurve,
}

impl crate::Parse for VfxEntryExpressionValue_AnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = AnimationCurve::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_AnimationCurve {
            m_expression_index,
            m_value,
        })
    }
}
