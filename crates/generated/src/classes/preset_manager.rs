//! 自动生成。请勿手动修改。

use crate::subclasses::default_preset::DefaultPreset;
use crate::subclasses::default_preset_list::DefaultPresetList;
use crate::subclasses::preset_type::PresetType;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct PresetManager {
    pub parent: Object,
    pub m_default_list: Option<Vec<DefaultPresetList>>,
    pub m_default_presets: Option<Vec<(PresetType, Vec<DefaultPreset>)>>,
}

impl crate::Parse for PresetManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_default_list = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { DefaultPresetList::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_default_presets = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = PresetType::parse(r, ver)?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { DefaultPreset::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(PresetManager {
            parent,
            m_default_list,
            m_default_presets,
        })
    }
}
