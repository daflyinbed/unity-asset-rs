//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct FloatCurve {
    pub curve: AnimationCurve,
    pub attribute: String,
    pub path: String,
    pub class_id: i32,
    pub script: PPtr,
}

impl crate::Parse for FloatCurve {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = AnimationCurve::parse(r, ver)?;
        let attribute = r.read_aligned_string()?;
        r.align(4)?;
        let path = r.read_aligned_string()?;
        r.align(4)?;
        let class_id = r.read_i32()?;
        let script = crate::PPtr::parse(r, ver)?;

        Ok(FloatCurve {
            curve,
            attribute,
            path,
            class_id,
            script,
        })
    }
}
