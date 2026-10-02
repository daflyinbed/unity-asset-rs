//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct MultiModeParameter_MeshSpawn {
    pub value: f32,
    pub mode: i32,
    pub spread: f32,
    pub speed: MinMaxCurve,
}

impl crate::Parse for MultiModeParameter_MeshSpawn {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = rdr.read_f32()?;
        let mode = rdr.read_i32()?;
        let spread = rdr.read_f32()?;
        let speed = MinMaxCurve::parse(rdr, ver)?;

        Ok(MultiModeParameter_MeshSpawn {
            value,
            mode,
            spread,
            speed,
        })
    }
}

impl crate::ToValue for MultiModeParameter_MeshSpawn {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("value".to_string(), self.value.to_value())),
            Some(("mode".to_string(), self.mode.to_value())),
            Some(("spread".to_string(), self.spread.to_value())),
            Some(("speed".to_string(), self.speed.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "value" => (self.value).inspect_value(path, offset, limit),
                "mode" => (self.mode).inspect_value(path, offset, limit),
                "spread" => (self.spread).inspect_value(path, offset, limit),
                "speed" => (self.speed).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
            ((self.mode).inspect_value(path, offset, limit)).map(|v| ("mode".to_string(), v)),
            ((self.spread).inspect_value(path, offset, limit)).map(|v| ("spread".to_string(), v)),
            ((self.speed).inspect_value(path, offset, limit)).map(|v| ("speed".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
