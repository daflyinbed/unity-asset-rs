//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgb_af::ColorRgbAf;

#[derive(Debug, Clone)]
pub struct AlbedoSwitchInfo {
    pub name: String,
    pub color: ColorRgbAf,
    pub min_luminance: f32,
    pub max_luminance: f32,
}

impl crate::Parse for AlbedoSwitchInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let color = ColorRgbAf::parse(rdr, ver)?;
        let min_luminance = rdr.read_f32()?;
        let max_luminance = rdr.read_f32()?;
        rdr.align(4)?;

        Ok(AlbedoSwitchInfo {
            name,
            color,
            min_luminance,
            max_luminance,
        })
    }
}

impl crate::ToValue for AlbedoSwitchInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("color".to_string(), self.color.to_value())),
            Some(("minLuminance".to_string(), self.min_luminance.to_value())),
            Some(("maxLuminance".to_string(), self.max_luminance.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "color" => (self.color).inspect_value(path, offset, limit),
                "minLuminance" => (self.min_luminance).inspect_value(path, offset, limit),
                "maxLuminance" => (self.max_luminance).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.color).inspect_value(path, offset, limit)).map(|v| ("color".to_string(), v)),
            ((self.min_luminance).inspect_value(path, offset, limit))
                .map(|v| ("minLuminance".to_string(), v)),
            ((self.max_luminance).inspect_value(path, offset, limit))
                .map(|v| ("maxLuminance".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
