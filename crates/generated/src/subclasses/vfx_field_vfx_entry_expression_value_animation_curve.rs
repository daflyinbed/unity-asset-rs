//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_expression_value_animation_curve::VfxEntryExpressionValue_AnimationCurve;

#[derive(Debug, Clone)]
pub struct VfxField_VfxEntryExpressionValue_AnimationCurve {
    pub m_array: Vec<VfxEntryExpressionValue_AnimationCurve>,
}

impl crate::Parse for VfxField_VfxEntryExpressionValue_AnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExpressionValue_AnimationCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(VfxField_VfxEntryExpressionValue_AnimationCurve {
            m_array,
        })
    }
}
