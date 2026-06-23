//! 自动生成。请勿手动修改。

use crate::subclasses::collider::Collider;
use crate::subclasses::handle::Handle;
use crate::subclasses::offset_ptr::OffsetPtr;
use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct Human {
    pub m_root_x: xform,
    pub m_skeleton: OffsetPtr,
    pub m_skeleton_pose: OffsetPtr,
    pub m_left_hand: OffsetPtr,
    pub m_right_hand: OffsetPtr,
    pub m_handles: Vec<Handle>,
    pub m_collider_array: Vec<Collider>,
    pub m_human_bone_index: Vec<i32>,
    pub m_human_bone_mass: Vec<f32>,
    pub m_collider_index: Vec<i32>,
    pub m_scale: f32,
    pub m_arm_twist: f32,
    pub m_fore_arm_twist: f32,
    pub m_upper_leg_twist: f32,
    pub m_leg_twist: f32,
    pub m_arm_stretch: f32,
    pub m_leg_stretch: f32,
    pub m_feet_spacing: f32,
    pub m_has_left_hand: bool,
    pub m_has_right_hand: bool,
}

impl crate::Parse for Human {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_root_x = xform::parse(r, ver)?;
        let m_skeleton = OffsetPtr::parse(r, ver)?;
        let m_skeleton_pose = OffsetPtr::parse(r, ver)?;
        let m_left_hand = OffsetPtr::parse(r, ver)?;
        let m_right_hand = OffsetPtr::parse(r, ver)?;
        let m_handles = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Handle::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_collider_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Collider::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_human_bone_index = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_human_bone_mass = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_collider_index = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_scale = r.read_f32()?;
        let m_arm_twist = r.read_f32()?;
        let m_fore_arm_twist = r.read_f32()?;
        let m_upper_leg_twist = r.read_f32()?;
        let m_leg_twist = r.read_f32()?;
        let m_arm_stretch = r.read_f32()?;
        let m_leg_stretch = r.read_f32()?;
        let m_feet_spacing = r.read_f32()?;
        let m_has_left_hand = r.read_bool()?;
        let m_has_right_hand = r.read_bool()?;
        r.align(4)?;

        Ok(Human {
            m_root_x,
            m_skeleton,
            m_skeleton_pose,
            m_left_hand,
            m_right_hand,
            m_handles,
            m_collider_array,
            m_human_bone_index,
            m_human_bone_mass,
            m_collider_index,
            m_scale,
            m_arm_twist,
            m_fore_arm_twist,
            m_upper_leg_twist,
            m_leg_twist,
            m_arm_stretch,
            m_leg_stretch,
            m_feet_spacing,
            m_has_left_hand,
            m_has_right_hand,
        })
    }
}
