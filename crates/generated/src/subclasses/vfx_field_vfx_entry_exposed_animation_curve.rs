//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_exposed_animation_curve::VfxEntryExposed_AnimationCurve;

#[derive(Debug, Clone)]
pub struct VfxField_VfxEntryExposed_AnimationCurve {
    pub m_array: Vec<VfxEntryExposed_AnimationCurve>,
}

impl crate::Parse for VfxField_VfxEntryExposed_AnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VfxEntryExposed_AnimationCurve::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(VfxField_VfxEntryExposed_AnimationCurve { m_array })
    }
}
