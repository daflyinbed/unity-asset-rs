//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct ClipAnimationInfoCurve {
    pub name: String,
    pub curve: AnimationCurve,
}

impl crate::Parse for ClipAnimationInfoCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let curve = AnimationCurve::parse(rdr, ver)?;

        Ok(ClipAnimationInfoCurve { name, curve })
    }
}
