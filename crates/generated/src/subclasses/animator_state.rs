//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct AnimatorState {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_speed: f32,
    pub m_cycle_offset: f32,
    pub m_transitions: Vec<PPtr>,
    pub m_state_machine_behaviours: Vec<PPtr>,
    pub m_position: Vector3f,
    pub m_ik_on_feet: bool,
    pub m_write_default_values: bool,
    pub m_mirror: bool,
    pub m_motion: PPtr,
    pub m_tag: String,
}

impl crate::Parse for AnimatorState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_speed = r.read_f32()?;
        let m_cycle_offset = r.read_f32()?;
        let m_transitions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_state_machine_behaviours = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_position = Vector3f::parse(r, ver)?;
        let m_ik_on_feet = r.read_bool()?;
        let m_write_default_values = r.read_bool()?;
        let m_mirror = r.read_bool()?;
        r.align(4)?;
        let m_motion = crate::PPtr::parse(r, ver)?;
        let m_tag = r.read_aligned_string()?;
        r.align(4)?;

        Ok(AnimatorState {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_speed,
            m_cycle_offset,
            m_transitions,
            m_state_machine_behaviours,
            m_position,
            m_ik_on_feet,
            m_write_default_values,
            m_mirror,
            m_motion,
            m_tag,
        })
    }
}
