//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct LightsModule {
    pub enabled: bool,
    pub ratio: f32,
    pub light: PPtr,
    pub random_distribution: bool,
    pub color: bool,
    pub range: bool,
    pub intensity: bool,
    pub range_curve: MinMaxCurve,
    pub intensity_curve: MinMaxCurve,
    pub max_lights: i32,
}

impl crate::Parse for LightsModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let ratio = rdr.read_f32()?;
        let light = crate::PPtr::parse(rdr, ver)?;
        let random_distribution = rdr.read_bool()?;
        let color = rdr.read_bool()?;
        let range = rdr.read_bool()?;
        let intensity = rdr.read_bool()?;
        let range_curve = MinMaxCurve::parse(rdr, ver)?;
        let intensity_curve = MinMaxCurve::parse(rdr, ver)?;
        let max_lights = rdr.read_i32()?;

        Ok(LightsModule {
            enabled,
            ratio,
            light,
            random_distribution,
            color,
            range,
            intensity,
            range_curve,
            intensity_curve,
            max_lights,
        })
    }
}
