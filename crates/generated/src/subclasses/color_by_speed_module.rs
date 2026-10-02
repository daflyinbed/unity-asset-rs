//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_gradient::MinMaxGradient;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ColorBySpeedModule {
    pub enabled: bool,
    pub gradient: MinMaxGradient,
    pub range: Vector2f,
}

impl crate::Parse for ColorBySpeedModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let gradient = MinMaxGradient::parse(rdr, ver)?;
        let range = Vector2f::parse(rdr, ver)?;

        Ok(ColorBySpeedModule {
            enabled,
            gradient,
            range,
        })
    }
}

impl crate::ToValue for ColorBySpeedModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("gradient".to_string(), self.gradient.to_value())),
            Some(("range".to_string(), self.range.to_value())),
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
                "gradient" => (self.gradient).inspect_value(path, offset, limit),
                "range" => (self.range).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.gradient).inspect_value(path, offset, limit))
                .map(|v| ("gradient".to_string(), v)),
            ((self.range).inspect_value(path, offset, limit)).map(|v| ("range".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
