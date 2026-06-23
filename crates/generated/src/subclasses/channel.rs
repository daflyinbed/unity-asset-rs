//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;

#[derive(Debug, Clone)]
pub struct Channel {
    pub byte_offset: i32,
    pub curve: AnimationCurve,
    pub attribute_name: String,
}

impl crate::Parse for Channel {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let byte_offset = r.read_i32()?;
        let curve = AnimationCurve::parse(r, ver)?;
        let attribute_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(Channel {
            byte_offset,
            curve,
            attribute_name,
        })
    }
}
