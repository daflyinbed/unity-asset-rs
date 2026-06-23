//! 自动生成。请勿手动修改。

use crate::subclasses::default_preset_list::DefaultPresetList;

#[derive(Debug, Clone)]
pub struct PresetManager {
    pub m_object_hide_flags: u32,
    pub m_default_list: Vec<DefaultPresetList>,
}

impl crate::Parse for PresetManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_default_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { DefaultPresetList::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(PresetManager {
            m_object_hide_flags,
            m_default_list,
        })
    }
}
