//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct MultiModeParameter {
    pub value: f32,
    pub mode: i32,
    pub spread: f32,
    pub speed: MinMaxCurve,
}

impl crate::Parse for MultiModeParameter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = r.read_f32()?;
        let mode = r.read_i32()?;
        let spread = r.read_f32()?;
        let speed = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;

        Ok(MultiModeParameter {
            value,
            mode,
            spread,
            speed,
        })
    }
}
