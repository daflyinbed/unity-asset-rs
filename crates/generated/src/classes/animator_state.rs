//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：AnimatorStateMTransitionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMTransitionsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimatorStateMTransitionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMStateMachineBehavioursVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMStateMachineBehavioursVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimatorStateMStateMachineBehavioursVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMIkOnFeetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMIkOnFeetVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for AnimatorStateMIkOnFeetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMMirrorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMMirrorVariant {
    Default(bool),
    V5_1(bool),
}

impl crate::Parse for AnimatorStateMMirrorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorStateMCycleOffsetParameterActiveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorStateMCycleOffsetParameterActiveVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for AnimatorStateMCycleOffsetParameterActiveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimatorState {
    pub m_name: String,
    pub m_speed: f32,
    pub m_motions: Option<Vec<PPtr>>,
    pub m_cycle_offset: Option<f32>,
    pub m_parent_state_machine: Option<PPtr>,
    pub m_transitions: Option<AnimatorStateMTransitionsVariant>,
    pub m_position: Vector3f,
    pub m_state_machine_behaviours: Option<AnimatorStateMStateMachineBehavioursVariant>,
    pub m_ik_on_feet: AnimatorStateMIkOnFeetVariant,
    pub m_tag: String,
    pub m_mirror: Option<AnimatorStateMMirrorVariant>,
    pub m_write_default_values: Option<bool>,
    pub m_motion: Option<PPtr>,
    pub m_speed_parameter_active: Option<bool>,
    pub m_mirror_parameter_active: Option<bool>,
    pub m_cycle_offset_parameter_active: Option<AnimatorStateMCycleOffsetParameterActiveVariant>,
    pub m_time_parameter_active: Option<bool>,
    pub m_speed_parameter: Option<String>,
    pub m_mirror_parameter: Option<String>,
    pub m_cycle_offset_parameter: Option<String>,
    pub m_time_parameter: Option<String>,
}

impl crate::Parse for AnimatorState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_speed = None;
        let mut m_motions = None;
        let mut m_cycle_offset = None;
        let mut m_parent_state_machine = None;
        let mut m_transitions = None;
        let mut m_position = None;
        let mut m_state_machine_behaviours = None;
        let mut m_ik_on_feet = None;
        let mut m_tag = None;
        let mut m_mirror = None;
        let mut m_write_default_values = None;
        let mut m_motion = None;
        let mut m_speed_parameter_active = None;
        let mut m_mirror_parameter_active = None;
        let mut m_cycle_offset_parameter_active = None;
        let mut m_time_parameter_active = None;
        let mut m_speed_parameter = None;
        let mut m_mirror_parameter = None;
        let mut m_cycle_offset_parameter = None;
        let mut m_time_parameter = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_speed = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_cycle_offset = Some(rdr.read_f32()?);
            m_transitions = Some(AnimatorStateMTransitionsVariant::parse(rdr, ver)?);
            m_state_machine_behaviours = Some(AnimatorStateMStateMachineBehavioursVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_cycle_offset = Some(rdr.read_f32()?);
            m_motions = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_parent_state_machine = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
            m_motions = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_parent_state_machine = Some(crate::PPtr::parse(rdr, ver)?);
        }
        m_position = Some(Vector3f::parse(rdr, ver)?);
        m_ik_on_feet = Some(AnimatorStateMIkOnFeetVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_write_default_values = Some(rdr.read_bool()?);
            m_mirror = Some(AnimatorStateMMirrorVariant::parse(rdr, ver)?);
            m_speed_parameter_active = Some(rdr.read_bool()?);
            m_mirror_parameter_active = Some(rdr.read_bool()?);
            m_cycle_offset_parameter_active = Some(
                AnimatorStateMCycleOffsetParameterActiveVariant::parse(rdr, ver)?,
            );
            m_time_parameter_active = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_motion = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_write_default_values = Some(rdr.read_bool()?);
            m_mirror = Some(AnimatorStateMMirrorVariant::parse(rdr, ver)?);
            m_speed_parameter_active = Some(rdr.read_bool()?);
            m_mirror_parameter_active = Some(rdr.read_bool()?);
            m_cycle_offset_parameter_active = Some(
                AnimatorStateMCycleOffsetParameterActiveVariant::parse(rdr, ver)?,
            );
            m_motion = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_write_default_values = Some(rdr.read_bool()?);
            m_mirror = Some(AnimatorStateMMirrorVariant::parse(rdr, ver)?);
            m_motion = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_mirror = Some(AnimatorStateMMirrorVariant::parse(rdr, ver)?);
        } else {
        }
        m_tag = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_speed_parameter = Some(rdr.read_aligned_string()?);
            m_mirror_parameter = Some(rdr.read_aligned_string()?);
            m_cycle_offset_parameter = Some(rdr.read_aligned_string()?);
            m_time_parameter = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_speed_parameter = Some(rdr.read_aligned_string()?);
            m_mirror_parameter = Some(rdr.read_aligned_string()?);
            m_cycle_offset_parameter = Some(rdr.read_aligned_string()?);
        } else {
        }

        Ok(AnimatorState {
            m_name: m_name.unwrap(),
            m_speed: m_speed.unwrap(),
            m_motions,
            m_cycle_offset,
            m_parent_state_machine,
            m_transitions,
            m_position: m_position.unwrap(),
            m_state_machine_behaviours,
            m_ik_on_feet: m_ik_on_feet.unwrap(),
            m_tag: m_tag.unwrap(),
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
