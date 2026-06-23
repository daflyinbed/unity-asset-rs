//! 自动生成。请勿手动修改。

use crate::subclasses::quaternionf::Quaternionf;

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub value: Quaternionf,
    pub in_slope: Quaternionf,
    pub out_slope: Quaternionf,
    pub tangent_mode: i32,
}

impl crate::Parse for Keyframe {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = r.read_f32()?;
        let value = Quaternionf::parse(r, ver)?;
        let in_slope = Quaternionf::parse(r, ver)?;
        let out_slope = Quaternionf::parse(r, ver)?;
        let tangent_mode = r.read_i32()?;

        Ok(Keyframe {
            time,
            value,
            in_slope,
            out_slope,
            tangent_mode,
        })
    }
}
