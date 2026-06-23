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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = PresetType::parse(r, ver)?;
        r.align(4)?;
        let default_presets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { DefaultPreset::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(DefaultPresetList {
            r#type,
            default_presets,
        })
    }
}
