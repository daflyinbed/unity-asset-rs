//! 自动生成。请勿手动修改。

use crate::subclasses::default_preset::DefaultPreset;
use crate::subclasses::default_preset_list::DefaultPresetList;
use crate::subclasses::preset_type::PresetType;

#[derive(Debug, Clone)]
pub struct PresetManager {
    pub m_default_list: Option<Vec<DefaultPresetList>>,
    pub m_default_presets: Option<Vec<(PresetType, Vec<DefaultPreset>)>>,
}

impl crate::Parse for PresetManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_default_list = None;
        let mut m_default_presets = None;
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_default_presets = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = PresetType::parse(rdr, ver)?;
                                let b = ({
                                    let n = rdr.read_i32()? as usize;
                                    (0..n)
                                        .map(|_| DefaultPreset::parse(rdr, ver))
                                        .collect::<Result<Vec<_>, _>>()
                                })?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
            m_default_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| DefaultPresetList::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }

        Ok(PresetManager {
            m_default_list,
            m_default_presets,
        })
    }
}

impl crate::ToValue for PresetManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_DefaultList", &self.m_default_list),
            crate::opt_field("m_DefaultPresets", &self.m_default_presets),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_DefaultList" => (self.m_default_list).inspect_value(path, offset, limit),
                "m_DefaultPresets" => (self.m_default_presets).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_default_list).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultList".to_string(), v)),
            ((self.m_default_presets).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultPresets".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
