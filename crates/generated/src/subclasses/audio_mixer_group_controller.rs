//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AudioMixerGroupController {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_audio_mixer: PPtr,
    pub m_group_id: Guid,
    pub m_children: Vec<PPtr>,
    pub m_volume: Guid,
    pub m_pitch: Guid,
    pub m_effects: Vec<PPtr>,
    pub m_user_color_index: i32,
    pub m_mute: bool,
    pub m_solo: bool,
    pub m_bypass_effects: bool,
}

impl crate::Parse for AudioMixerGroupController {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_audio_mixer = crate::PPtr::parse(r, ver)?;
        let m_group_id = Guid::parse(r, ver)?;
        let m_children = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_volume = Guid::parse(r, ver)?;
        let m_pitch = Guid::parse(r, ver)?;
        let m_effects = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_user_color_index = r.read_i32()?;
        let m_mute = r.read_bool()?;
        let m_solo = r.read_bool()?;
        let m_bypass_effects = r.read_bool()?;
        r.align(4)?;

        Ok(AudioMixerGroupController {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_audio_mixer,
            m_group_id,
            m_children,
            m_volume,
            m_pitch,
            m_effects,
            m_user_color_index,
            m_mute,
            m_solo,
            m_bypass_effects,
        })
    }
}
