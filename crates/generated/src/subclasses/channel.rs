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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let byte_offset = rdr.read_i32()?;
        let curve = AnimationCurve::parse(rdr, ver)?;
        let attribute_name = rdr.read_aligned_string()?;

        Ok(Channel {
            byte_offset,
            curve,
            attribute_name,
        })
    }
}
