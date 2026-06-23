//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct ClipAnimationInfoCurve {
    pub name: String,
    pub curve: AnimationCurve,
}

impl crate::Parse for ClipAnimationInfoCurve {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let curve = AnimationCurve::parse(r, ver)?;

        Ok(ClipAnimationInfoCurve {
            name,
            curve,
        })
    }
}
