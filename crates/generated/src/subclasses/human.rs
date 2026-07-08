//! 自动生成。请勿手动修改。

use crate::subclasses::collider::Collider;
use crate::subclasses::handle::Handle;
use crate::subclasses::offset_ptr::OffsetPtr;
use crate::subclasses::xform::xform;


/// 版本枚举：HumanMHasRightHandVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HumanMHasRightHandVariant {
    Default(bool),
    V5_2(bool),
}

impl crate::Parse for HumanMHasRightHandVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Human {
    pub m_root_x: xform,
    pub m_skeleton: OffsetPtr,
    pub m_skeleton_pose: OffsetPtr,
    pub m_left_hand: OffsetPtr,
    pub m_right_hand: OffsetPtr,
    pub m_handles: Option<Vec<Handle>>,
    pub m_human_bone_index: Vec<i32>,
    pub m_collider_array: Option<Vec<Collider>>,
    pub m_human_bone_mass: Vec<f32>,
    pub m_scale: f32,
    pub m_arm_twist: f32,
    pub m_collider_index: Option<Vec<i32>>,
    pub m_fore_arm_twist: f32,
    pub m_upper_leg_twist: f32,
    pub m_leg_twist: f32,
    pub m_arm_stretch: f32,
    pub m_leg_stretch: f32,
    pub m_feet_spacing: f32,
    pub m_has_left_hand: bool,
    pub m_has_right_hand: HumanMHasRightHandVariant,
    pub m_has_t_do_f: Option<bool>,
}

impl crate::Parse for Human {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_root_x = None;
        let mut m_skeleton = None;
        let mut m_skeleton_pose = None;
        let mut m_left_hand = None;
        let mut m_right_hand = None;
        let mut m_handles = None;
        let mut m_human_bone_index = None;
        let mut m_collider_array = None;
        let mut m_human_bone_mass = None;
        let mut m_scale = None;
        let mut m_arm_twist = None;
        let mut m_collider_index = None;
        let mut m_fore_arm_twist = None;
        let mut m_upper_leg_twist = None;
        let mut m_leg_twist = None;
        let mut m_arm_stretch = None;
        let mut m_leg_stretch = None;
        let mut m_feet_spacing = None;
        let mut m_has_left_hand = None;
        let mut m_has_right_hand = None;
        let mut m_has_t_do_f = None;
            m_root_x = Some(xform::parse(rdr, ver)?);
            m_skeleton = Some(OffsetPtr::parse(rdr, ver)?);
            m_skeleton_pose = Some(OffsetPtr::parse(rdr, ver)?);
            m_left_hand = Some(OffsetPtr::parse(rdr, ver)?);
            m_right_hand = Some(OffsetPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
        } else {
            m_handles = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Handle::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_collider_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Collider::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_human_bone_index = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            m_human_bone_mass = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
        } else {
            m_collider_index = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_scale = Some(rdr.read_f32()?);
            m_arm_twist = Some(rdr.read_f32()?);
            m_fore_arm_twist = Some(rdr.read_f32()?);
            m_upper_leg_twist = Some(rdr.read_f32()?);
            m_leg_twist = Some(rdr.read_f32()?);
            m_arm_stretch = Some(rdr.read_f32()?);
            m_leg_stretch = Some(rdr.read_f32()?);
            m_feet_spacing = Some(rdr.read_f32()?);
            m_has_left_hand = Some(rdr.read_bool()?);
            m_has_right_hand = Some(HumanMHasRightHandVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_has_t_do_f = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(Human {
            m_root_x: m_root_x.unwrap(),
            m_skeleton: m_skeleton.unwrap(),
            m_skeleton_pose: m_skeleton_pose.unwrap(),
            m_left_hand: m_left_hand.unwrap(),
            m_right_hand: m_right_hand.unwrap(),
            m_handles,
            m_human_bone_index: m_human_bone_index.unwrap(),
            m_collider_array,
            m_human_bone_mass: m_human_bone_mass.unwrap(),
            m_scale: m_scale.unwrap(),
            m_arm_twist: m_arm_twist.unwrap(),
            m_collider_index,
            m_fore_arm_twist: m_fore_arm_twist.unwrap(),
            m_upper_leg_twist: m_upper_leg_twist.unwrap(),
            m_leg_twist: m_leg_twist.unwrap(),
            m_arm_stretch: m_arm_stretch.unwrap(),
            m_leg_stretch: m_leg_stretch.unwrap(),
            m_feet_spacing: m_feet_spacing.unwrap(),
            m_has_left_hand: m_has_left_hand.unwrap(),
            m_has_right_hand: m_has_right_hand.unwrap(),
            m_has_t_do_f,
        })
    }
}
