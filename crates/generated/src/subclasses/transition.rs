//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::condition::Condition;

#[derive(Debug, Clone)]
pub struct Transition {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_src_state: PPtr,
    pub m_dst_state: PPtr,
    pub m_transition_duration: f32,
    pub m_transition_offset: f32,
    pub m_conditions: Vec<Condition>,
    pub m_atomic: bool,
    pub m_solo: bool,
    pub m_mute: bool,
}

impl crate::Parse for Transition {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_src_state = crate::PPtr::parse(r, ver)?;
        let m_dst_state = crate::PPtr::parse(r, ver)?;
        let m_transition_duration = r.read_f32()?;
        let m_transition_offset = r.read_f32()?;
        let m_conditions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Condition::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_atomic = r.read_bool()?;
        let m_solo = r.read_bool()?;
        let m_mute = r.read_bool()?;

        Ok(Transition {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_src_state,
            m_dst_state,
            m_transition_duration,
            m_transition_offset,
            m_conditions,
            m_atomic,
            m_solo,
            m_mute,
        })
    }
}
