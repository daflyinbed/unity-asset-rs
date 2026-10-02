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

impl crate::ToValue for HumanMHasRightHandVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_2(v) => (v).inspect_value(path, offset, limit),
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
            m_handles = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Handle::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_collider_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Collider::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_human_bone_index = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_human_bone_mass = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
        } else {
            m_collider_index = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for Human {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_RootX".to_string(), self.m_root_x.to_value())),
            Some(("m_Skeleton".to_string(), self.m_skeleton.to_value())),
            Some((
                "m_SkeletonPose".to_string(),
                self.m_skeleton_pose.to_value(),
            )),
            Some(("m_LeftHand".to_string(), self.m_left_hand.to_value())),
            Some(("m_RightHand".to_string(), self.m_right_hand.to_value())),
            crate::opt_field("m_Handles", &self.m_handles),
            Some((
                "m_HumanBoneIndex".to_string(),
                self.m_human_bone_index.to_value(),
            )),
            crate::opt_field("m_ColliderArray", &self.m_collider_array),
            Some((
                "m_HumanBoneMass".to_string(),
                self.m_human_bone_mass.to_value(),
            )),
            Some(("m_Scale".to_string(), self.m_scale.to_value())),
            Some(("m_ArmTwist".to_string(), self.m_arm_twist.to_value())),
            crate::opt_field("m_ColliderIndex", &self.m_collider_index),
            Some((
                "m_ForeArmTwist".to_string(),
                self.m_fore_arm_twist.to_value(),
            )),
            Some((
                "m_UpperLegTwist".to_string(),
                self.m_upper_leg_twist.to_value(),
            )),
            Some(("m_LegTwist".to_string(), self.m_leg_twist.to_value())),
            Some(("m_ArmStretch".to_string(), self.m_arm_stretch.to_value())),
            Some(("m_LegStretch".to_string(), self.m_leg_stretch.to_value())),
            Some(("m_FeetSpacing".to_string(), self.m_feet_spacing.to_value())),
            Some(("m_HasLeftHand".to_string(), self.m_has_left_hand.to_value())),
            Some((
                "m_HasRightHand".to_string(),
                self.m_has_right_hand.to_value(),
            )),
            crate::opt_field("m_HasTDoF", &self.m_has_t_do_f),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_RootX" => (self.m_root_x).inspect_value(path, offset, limit),
                "m_Skeleton" => (self.m_skeleton).inspect_value(path, offset, limit),
                "m_SkeletonPose" => (self.m_skeleton_pose).inspect_value(path, offset, limit),
                "m_LeftHand" => (self.m_left_hand).inspect_value(path, offset, limit),
                "m_RightHand" => (self.m_right_hand).inspect_value(path, offset, limit),
                "m_Handles" => (self.m_handles).inspect_value(path, offset, limit),
                "m_HumanBoneIndex" => (self.m_human_bone_index).inspect_value(path, offset, limit),
                "m_ColliderArray" => (self.m_collider_array).inspect_value(path, offset, limit),
                "m_HumanBoneMass" => (self.m_human_bone_mass).inspect_value(path, offset, limit),
                "m_Scale" => (self.m_scale).inspect_value(path, offset, limit),
                "m_ArmTwist" => (self.m_arm_twist).inspect_value(path, offset, limit),
                "m_ColliderIndex" => (self.m_collider_index).inspect_value(path, offset, limit),
                "m_ForeArmTwist" => (self.m_fore_arm_twist).inspect_value(path, offset, limit),
                "m_UpperLegTwist" => (self.m_upper_leg_twist).inspect_value(path, offset, limit),
                "m_LegTwist" => (self.m_leg_twist).inspect_value(path, offset, limit),
                "m_ArmStretch" => (self.m_arm_stretch).inspect_value(path, offset, limit),
                "m_LegStretch" => (self.m_leg_stretch).inspect_value(path, offset, limit),
                "m_FeetSpacing" => (self.m_feet_spacing).inspect_value(path, offset, limit),
                "m_HasLeftHand" => (self.m_has_left_hand).inspect_value(path, offset, limit),
                "m_HasRightHand" => (self.m_has_right_hand).inspect_value(path, offset, limit),
                "m_HasTDoF" => (self.m_has_t_do_f).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_root_x).inspect_value(path, offset, limit))
                .map(|v| ("m_RootX".to_string(), v)),
            ((self.m_skeleton).inspect_value(path, offset, limit))
                .map(|v| ("m_Skeleton".to_string(), v)),
            ((self.m_skeleton_pose).inspect_value(path, offset, limit))
                .map(|v| ("m_SkeletonPose".to_string(), v)),
            ((self.m_left_hand).inspect_value(path, offset, limit))
                .map(|v| ("m_LeftHand".to_string(), v)),
            ((self.m_right_hand).inspect_value(path, offset, limit))
                .map(|v| ("m_RightHand".to_string(), v)),
            ((self.m_handles).inspect_value(path, offset, limit))
                .map(|v| ("m_Handles".to_string(), v)),
            ((self.m_human_bone_index).inspect_value(path, offset, limit))
                .map(|v| ("m_HumanBoneIndex".to_string(), v)),
            ((self.m_collider_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderArray".to_string(), v)),
            ((self.m_human_bone_mass).inspect_value(path, offset, limit))
                .map(|v| ("m_HumanBoneMass".to_string(), v)),
            ((self.m_scale).inspect_value(path, offset, limit)).map(|v| ("m_Scale".to_string(), v)),
            ((self.m_arm_twist).inspect_value(path, offset, limit))
                .map(|v| ("m_ArmTwist".to_string(), v)),
            ((self.m_collider_index).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderIndex".to_string(), v)),
            ((self.m_fore_arm_twist).inspect_value(path, offset, limit))
                .map(|v| ("m_ForeArmTwist".to_string(), v)),
            ((self.m_upper_leg_twist).inspect_value(path, offset, limit))
                .map(|v| ("m_UpperLegTwist".to_string(), v)),
            ((self.m_leg_twist).inspect_value(path, offset, limit))
                .map(|v| ("m_LegTwist".to_string(), v)),
            ((self.m_arm_stretch).inspect_value(path, offset, limit))
                .map(|v| ("m_ArmStretch".to_string(), v)),
            ((self.m_leg_stretch).inspect_value(path, offset, limit))
                .map(|v| ("m_LegStretch".to_string(), v)),
            ((self.m_feet_spacing).inspect_value(path, offset, limit))
                .map(|v| ("m_FeetSpacing".to_string(), v)),
            ((self.m_has_left_hand).inspect_value(path, offset, limit))
                .map(|v| ("m_HasLeftHand".to_string(), v)),
            ((self.m_has_right_hand).inspect_value(path, offset, limit))
                .map(|v| ("m_HasRightHand".to_string(), v)),
            ((self.m_has_t_do_f).inspect_value(path, offset, limit))
                .map(|v| ("m_HasTDoF".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
