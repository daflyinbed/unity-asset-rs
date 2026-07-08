//! 自动生成。请勿手动修改。

use crate::subclasses::animation_curve::AnimationCurve;


/// 版本枚举：ChannelCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ChannelCurveVariant {
    Default(Box<AnimationCurve>),
    V2017_1(Box<AnimationCurve>),
}

impl crate::Parse for ChannelCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { Ok(Box::new(AnimationCurve::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { Ok(Box::new(AnimationCurve::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Channel {
    pub byte_offset: i32,
    pub curve: ChannelCurveVariant,
    pub attribute_name: String,
}

impl crate::Parse for Channel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let byte_offset = rdr.read_i32()?;
        let curve = ChannelCurveVariant::parse(rdr, ver)?;
        let attribute_name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(Channel {
            byte_offset,
            curve,
            attribute_name,
        })
    }
}
