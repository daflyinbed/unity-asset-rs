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
