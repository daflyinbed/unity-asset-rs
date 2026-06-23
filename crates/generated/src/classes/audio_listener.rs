//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::extension_property_value::ExtensionPropertyValue;
use super::audio_behaviour::AudioBehaviour;

#[derive(Debug, Clone)]
pub struct AudioListener {
    pub parent: AudioBehaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_extension_property_values: Option<Vec<ExtensionPropertyValue>>,
}

impl crate::Parse for AudioListener {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AudioBehaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_extension_property_values = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ExtensionPropertyValue::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AudioListener {
            parent,
            m_game_object,
            m_enabled,
            m_extension_property_values,
        })
    }
}
