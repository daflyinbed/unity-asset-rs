//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animator_condition::AnimatorCondition;

#[derive(Debug, Clone)]
pub struct AnimatorTransition {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_conditions: Vec<AnimatorCondition>,
    pub m_dst_state_machine: PPtr,
    pub m_dst_state: PPtr,
    pub m_solo: bool,
    pub m_mute: bool,
    pub m_is_exit: bool,
}

impl crate::Parse for AnimatorTransition {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_conditions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AnimatorCondition::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_dst_state_machine = crate::PPtr::parse(r, ver)?;
        let m_dst_state = crate::PPtr::parse(r, ver)?;
        let m_solo = r.read_bool()?;
        let m_mute = r.read_bool()?;
        let m_is_exit = r.read_bool()?;
        r.align(4)?;

        Ok(AnimatorTransition {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_conditions,
            m_dst_state_machine,
            m_dst_state,
            m_solo,
            m_mute,
            m_is_exit,
        })
    }
}
