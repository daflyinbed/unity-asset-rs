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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let ratio = r.read_f32()?;
        let light = crate::PPtr::parse(r, ver)?;
        let random_distribution = r.read_bool()?;
        let color = r.read_bool()?;
        let range = r.read_bool()?;
        let intensity = r.read_bool()?;
        let range_curve = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let intensity_curve = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let max_lights = r.read_i32()?;

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
