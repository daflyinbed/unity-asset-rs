//! 自动生成。请勿手动修改。

use crate::subclasses::human_bone::HumanBone;
use crate::subclasses::human_handle::HumanHandle;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::skeleton_bone::SkeletonBone;

/// 版本枚举：HumanDescriptionMHumanVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HumanDescriptionMHumanVariant {
    Default(Vec<HumanBone>),
    V2017_1(Vec<HumanBone>),
}

impl crate::Parse for HumanDescriptionMHumanVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| HumanBone::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| HumanBone::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HumanDescriptionMHumanVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：HumanDescriptionMSkeletonVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HumanDescriptionMSkeletonVariant {
    Default(Vec<SkeletonBone>),
    V2017_1(Vec<SkeletonBone>),
}

impl crate::Parse for HumanDescriptionMSkeletonVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SkeletonBone::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SkeletonBone::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HumanDescriptionMSkeletonVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：HumanDescriptionMHasTranslationDoFVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HumanDescriptionMHasTranslationDoFVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for HumanDescriptionMHasTranslationDoFVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HumanDescriptionMHasTranslationDoFVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HumanDescription {
    pub m_human: HumanDescriptionMHumanVariant,
    pub m_skeleton: HumanDescriptionMSkeletonVariant,
    pub m_handles: Option<Vec<HumanHandle>>,
    pub m_arm_twist: f32,
    pub m_fore_arm_twist: f32,
    pub m_upper_leg_twist: f32,
    pub m_leg_twist: f32,
    pub m_arm_stretch: f32,
    pub m_leg_stretch: f32,
    pub m_feet_spacing: f32,
    pub m_root_motion_bone_name: String,
    pub m_global_scale: Option<f32>,
    pub m_has_translation_do_f: Option<HumanDescriptionMHasTranslationDoFVariant>,
    pub m_root_motion_bone_rotation: Option<Quaternionf>,
    pub m_has_extra_root: Option<bool>,
    pub m_skeleton_has_parents: Option<bool>,
}

impl crate::Parse for HumanDescription {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_human = None;
        let mut m_skeleton = None;
        let mut m_handles = None;
        let mut m_arm_twist = None;
        let mut m_fore_arm_twist = None;
        let mut m_upper_leg_twist = None;
        let mut m_leg_twist = None;
        let mut m_arm_stretch = None;
        let mut m_leg_stretch = None;
        let mut m_feet_spacing = None;
        let mut m_root_motion_bone_name = None;
        let mut m_global_scale = None;
        let mut m_has_translation_do_f = None;
        let mut m_root_motion_bone_rotation = None;
        let mut m_has_extra_root = None;
        let mut m_skeleton_has_parents = None;
        m_human = Some(HumanDescriptionMHumanVariant::parse(rdr, ver)?);
        m_skeleton = Some(HumanDescriptionMSkeletonVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            m_handles = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| HumanHandle::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_arm_twist = Some(rdr.read_f32()?);
        m_fore_arm_twist = Some(rdr.read_f32()?);
        m_upper_leg_twist = Some(rdr.read_f32()?);
        m_leg_twist = Some(rdr.read_f32()?);
        m_arm_stretch = Some(rdr.read_f32()?);
        m_leg_stretch = Some(rdr.read_f32()?);
        m_feet_spacing = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_global_scale = Some(rdr.read_f32()?);
        } else {
        }
        m_root_motion_bone_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2018, 2, 13) {
            m_has_translation_do_f =
                Some(HumanDescriptionMHasTranslationDoFVariant::parse(rdr, ver)?);
            m_has_extra_root = Some(rdr.read_bool()?);
            m_skeleton_has_parents = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_root_motion_bone_rotation = Some(Quaternionf::parse(rdr, ver)?);
            m_has_translation_do_f =
                Some(HumanDescriptionMHasTranslationDoFVariant::parse(rdr, ver)?);
            m_has_extra_root = Some(rdr.read_bool()?);
            m_skeleton_has_parents = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_has_translation_do_f =
                Some(HumanDescriptionMHasTranslationDoFVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(HumanDescription {
            m_human: m_human.unwrap(),
            m_skeleton: m_skeleton.unwrap(),
            m_handles,
            m_arm_twist: m_arm_twist.unwrap(),
            m_fore_arm_twist: m_fore_arm_twist.unwrap(),
            m_upper_leg_twist: m_upper_leg_twist.unwrap(),
            m_leg_twist: m_leg_twist.unwrap(),
            m_arm_stretch: m_arm_stretch.unwrap(),
            m_leg_stretch: m_leg_stretch.unwrap(),
            m_feet_spacing: m_feet_spacing.unwrap(),
            m_root_motion_bone_name: m_root_motion_bone_name.unwrap(),
            m_global_scale,
            m_has_translation_do_f,
            m_root_motion_bone_rotation,
            m_has_extra_root,
            m_skeleton_has_parents,
        })
    }
}

impl crate::ToValue for HumanDescription {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Human".to_string(), self.m_human.to_value())),
            Some(("m_Skeleton".to_string(), self.m_skeleton.to_value())),
            crate::opt_field("m_Handles", &self.m_handles),
            Some(("m_ArmTwist".to_string(), self.m_arm_twist.to_value())),
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
            Some((
                "m_RootMotionBoneName".to_string(),
                self.m_root_motion_bone_name.to_value(),
            )),
            crate::opt_field("m_GlobalScale", &self.m_global_scale),
            crate::opt_field("m_HasTranslationDoF", &self.m_has_translation_do_f),
            crate::opt_field(
                "m_RootMotionBoneRotation",
                &self.m_root_motion_bone_rotation,
            ),
            crate::opt_field("m_HasExtraRoot", &self.m_has_extra_root),
            crate::opt_field("m_SkeletonHasParents", &self.m_skeleton_has_parents),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Human" => (self.m_human).inspect_value(path, offset, limit),
                "m_Skeleton" => (self.m_skeleton).inspect_value(path, offset, limit),
                "m_Handles" => (self.m_handles).inspect_value(path, offset, limit),
                "m_ArmTwist" => (self.m_arm_twist).inspect_value(path, offset, limit),
                "m_ForeArmTwist" => (self.m_fore_arm_twist).inspect_value(path, offset, limit),
                "m_UpperLegTwist" => (self.m_upper_leg_twist).inspect_value(path, offset, limit),
                "m_LegTwist" => (self.m_leg_twist).inspect_value(path, offset, limit),
                "m_ArmStretch" => (self.m_arm_stretch).inspect_value(path, offset, limit),
                "m_LegStretch" => (self.m_leg_stretch).inspect_value(path, offset, limit),
                "m_FeetSpacing" => (self.m_feet_spacing).inspect_value(path, offset, limit),
                "m_RootMotionBoneName" => {
                    (self.m_root_motion_bone_name).inspect_value(path, offset, limit)
                }
                "m_GlobalScale" => (self.m_global_scale).inspect_value(path, offset, limit),
                "m_HasTranslationDoF" => {
                    (self.m_has_translation_do_f).inspect_value(path, offset, limit)
                }
                "m_RootMotionBoneRotation" => {
                    (self.m_root_motion_bone_rotation).inspect_value(path, offset, limit)
                }
                "m_HasExtraRoot" => (self.m_has_extra_root).inspect_value(path, offset, limit),
                "m_SkeletonHasParents" => {
                    (self.m_skeleton_has_parents).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_human).inspect_value(path, offset, limit)).map(|v| ("m_Human".to_string(), v)),
            ((self.m_skeleton).inspect_value(path, offset, limit))
                .map(|v| ("m_Skeleton".to_string(), v)),
            ((self.m_handles).inspect_value(path, offset, limit))
                .map(|v| ("m_Handles".to_string(), v)),
            ((self.m_arm_twist).inspect_value(path, offset, limit))
                .map(|v| ("m_ArmTwist".to_string(), v)),
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
            ((self.m_root_motion_bone_name).inspect_value(path, offset, limit))
                .map(|v| ("m_RootMotionBoneName".to_string(), v)),
            ((self.m_global_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_GlobalScale".to_string(), v)),
            ((self.m_has_translation_do_f).inspect_value(path, offset, limit))
                .map(|v| ("m_HasTranslationDoF".to_string(), v)),
            ((self.m_root_motion_bone_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_RootMotionBoneRotation".to_string(), v)),
            ((self.m_has_extra_root).inspect_value(path, offset, limit))
                .map(|v| ("m_HasExtraRoot".to_string(), v)),
            ((self.m_skeleton_has_parents).inspect_value(path, offset, limit))
                .map(|v| ("m_SkeletonHasParents".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
