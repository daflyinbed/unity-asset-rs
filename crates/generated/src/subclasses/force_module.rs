//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct ForceModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub in_world_space: bool,
    pub randomize_per_frame: bool,
}

impl crate::Parse for ForceModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let x = MinMaxCurve::parse(rdr, ver)?;
        let y = MinMaxCurve::parse(rdr, ver)?;
        let z = MinMaxCurve::parse(rdr, ver)?;
        let in_world_space = rdr.read_bool()?;
        let randomize_per_frame = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(ForceModule {
            enabled,
            x,
            y,
            z,
            in_world_space,
            randomize_per_frame,
        })
    }
}
