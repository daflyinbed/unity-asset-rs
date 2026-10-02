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

impl crate::ToValue for LightsModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("ratio".to_string(), self.ratio.to_value())),
            Some(("light".to_string(), self.light.to_value())),
            Some((
                "randomDistribution".to_string(),
                self.random_distribution.to_value(),
            )),
            Some(("color".to_string(), self.color.to_value())),
            Some(("range".to_string(), self.range.to_value())),
            Some(("intensity".to_string(), self.intensity.to_value())),
            Some(("rangeCurve".to_string(), self.range_curve.to_value())),
            Some((
                "intensityCurve".to_string(),
                self.intensity_curve.to_value(),
            )),
            Some(("maxLights".to_string(), self.max_lights.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "ratio" => (self.ratio).inspect_value(path, offset, limit),
                "light" => (self.light).inspect_value(path, offset, limit),
                "randomDistribution" => {
                    (self.random_distribution).inspect_value(path, offset, limit)
                }
                "color" => (self.color).inspect_value(path, offset, limit),
                "range" => (self.range).inspect_value(path, offset, limit),
                "intensity" => (self.intensity).inspect_value(path, offset, limit),
                "rangeCurve" => (self.range_curve).inspect_value(path, offset, limit),
                "intensityCurve" => (self.intensity_curve).inspect_value(path, offset, limit),
                "maxLights" => (self.max_lights).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.ratio).inspect_value(path, offset, limit)).map(|v| ("ratio".to_string(), v)),
            ((self.light).inspect_value(path, offset, limit)).map(|v| ("light".to_string(), v)),
            ((self.random_distribution).inspect_value(path, offset, limit))
                .map(|v| ("randomDistribution".to_string(), v)),
            ((self.color).inspect_value(path, offset, limit)).map(|v| ("color".to_string(), v)),
            ((self.range).inspect_value(path, offset, limit)).map(|v| ("range".to_string(), v)),
            ((self.intensity).inspect_value(path, offset, limit))
                .map(|v| ("intensity".to_string(), v)),
            ((self.range_curve).inspect_value(path, offset, limit))
                .map(|v| ("rangeCurve".to_string(), v)),
            ((self.intensity_curve).inspect_value(path, offset, limit))
                .map(|v| ("intensityCurve".to_string(), v)),
            ((self.max_lights).inspect_value(path, offset, limit))
                .map(|v| ("maxLights".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
