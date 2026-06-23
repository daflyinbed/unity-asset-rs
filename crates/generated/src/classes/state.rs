//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct State {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_speed: f32,
    pub m_motions: Option<Vec<PPtr>>,
    pub m_cycle_offset: Option<f32>,
    pub m_parent_state_machine: Option<PPtr>,
    pub m_transitions: Option<Vec<PPtr>>,
    pub m_position: Vector3f,
    pub m_state_machine_behaviours: Option<Vec<PPtr>>,
    pub m_ik_on_feet: bool,
    pub m_tag: String,
    pub m_mirror: Option<bool>,
    pub m_write_default_values: Option<bool>,
    pub m_motion: Option<PPtr>,
    pub m_speed_parameter_active: Option<bool>,
    pub m_mirror_parameter_active: Option<bool>,
    pub m_cycle_offset_parameter_active: Option<bool>,
    pub m_time_parameter_active: Option<bool>,
    pub m_speed_parameter: Option<String>,
    pub m_mirror_parameter: Option<String>,
    pub m_cycle_offset_parameter: Option<String>,
    pub m_time_parameter: Option<String>,
}

impl crate::Parse for State {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_speed = r.read_f32()?;
        let m_motions = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_cycle_offset = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_parent_state_machine = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_transitions = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_position = Vector3f::parse(r, ver)?;
        let m_state_machine_behaviours = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_ik_on_feet = r.read_bool()?;
        r.align(4)?;
        let m_tag = r.read_aligned_string()?;
        r.align(4)?;
        let m_mirror = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_write_default_values = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_motion = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_speed_parameter_active = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_mirror_parameter_active = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_cycle_offset_parameter_active = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_time_parameter_active = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_speed_parameter = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mirror_parameter = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cycle_offset_parameter = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_time_parameter = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(State {
            parent,
            m_name,
            m_speed,
            m_motions,
            m_cycle_offset,
            m_parent_state_machine,
            m_transitions,
            m_position,
            m_state_machine_behaviours,
            m_ik_on_feet,
            m_tag,
            m_mirror,
            m_write_default_values,
            m_motion,
            m_speed_parameter_active,
            m_mirror_parameter_active,
            m_cycle_offset_parameter_active,
            m_time_parameter_active,
            m_speed_parameter,
            m_mirror_parameter,
            m_cycle_offset_parameter,
            m_time_parameter,
        })
    }
}
