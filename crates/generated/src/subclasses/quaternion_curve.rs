//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct QuaternionCurve {
    pub curve: AnimationCurve,
    pub path: String,
}

impl crate::Parse for QuaternionCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = AnimationCurve::parse(rdr, ver)?;
        let path = rdr.read_aligned_string()?;

        Ok(QuaternionCurve { curve, path })
    }
}
