//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct VelocityModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub in_world_space: bool,
}

impl crate::Parse for VelocityModule {
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
        let in_world_space = r.read_bool()?;
        r.align(4)?;

        Ok(VelocityModule {
            enabled,
            x,
            y,
            z,
            in_world_space,
        })
    }
}
