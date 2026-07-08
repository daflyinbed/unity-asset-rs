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
    pub flags: Option<i32>,
}

impl crate::Parse for FloatCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = AnimationCurve::parse(rdr, ver)?;
        let attribute = rdr.read_aligned_string()?;
        let path = rdr.read_aligned_string()?;
        let class_id = rdr.read_i32()?;
        let script = crate::PPtr::parse(rdr, ver)?;
        let flags = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(FloatCurve {
            curve,
            attribute,
            path,
            class_id,
            script,
            flags,
        })
    }
}
