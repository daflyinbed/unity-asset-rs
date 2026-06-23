//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct ClampVelocityModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub magnitude: MinMaxCurve,
    pub separate_axis: bool,
    pub dampen: f32,
}

impl crate::Parse for ClampVelocityModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let x = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let y = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let z = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let magnitude = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let separate_axis = r.read_bool()?;
        r.align(4)?;
        let dampen = r.read_f32()?;

        Ok(ClampVelocityModule {
            enabled,
            x,
            y,
            z,
            magnitude,
            separate_axis,
            dampen,
        })
    }
}
