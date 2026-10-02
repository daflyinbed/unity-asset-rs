//! 自动生成。请勿手动修改。

use crate::subclasses::default_preset::DefaultPreset;
use crate::subclasses::preset_type::PresetType;

#[derive(Debug, Clone)]
pub struct DefaultPresetList {
    pub r#type: PresetType,
    pub default_presets: Vec<DefaultPreset>,
}

impl crate::Parse for DefaultPresetList {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = PresetType::parse(rdr, ver)?;
        let default_presets = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| DefaultPreset::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(DefaultPresetList {
            r#type,
            default_presets,
        })
    }
}

impl crate::ToValue for DefaultPresetList {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("type".to_string(), self.r#type.to_value())),
            Some((
                "defaultPresets".to_string(),
                self.default_presets.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "defaultPresets" => (self.default_presets).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.default_presets).inspect_value(path, offset, limit))
                .map(|v| ("defaultPresets".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
