//! 自动生成。请勿手动修改。

use crate::subclasses::human_bone::HumanBone;
use crate::subclasses::human_handle::HumanHandle;
use crate::subclasses::skeleton_bone::SkeletonBone;

#[derive(Debug, Clone)]
pub struct HumanDescription {
    pub m_human: Vec<HumanBone>,
    pub m_skeleton: Vec<SkeletonBone>,
    pub m_handles: Vec<HumanHandle>,
    pub m_arm_twist: f32,
    pub m_fore_arm_twist: f32,
    pub m_upper_leg_twist: f32,
    pub m_leg_twist: f32,
    pub m_arm_stretch: f32,
    pub m_leg_stretch: f32,
    pub m_feet_spacing: f32,
    pub m_root_motion_bone_name: String,
}

impl crate::Parse for HumanDescription {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_human = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HumanBone::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_skeleton = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SkeletonBone::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_handles = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HumanHandle::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_arm_twist = r.read_f32()?;
        let m_fore_arm_twist = r.read_f32()?;
        let m_upper_leg_twist = r.read_f32()?;
        let m_leg_twist = r.read_f32()?;
        let m_arm_stretch = r.read_f32()?;
        let m_leg_stretch = r.read_f32()?;
        let m_feet_spacing = r.read_f32()?;
        let m_root_motion_bone_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(HumanDescription {
            m_human,
            m_skeleton,
            m_handles,
            m_arm_twist,
            m_fore_arm_twist,
            m_upper_leg_twist,
            m_leg_twist,
            m_arm_stretch,
            m_leg_stretch,
            m_feet_spacing,
            m_root_motion_bone_name,
        })
    }
}
