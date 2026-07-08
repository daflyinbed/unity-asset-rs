//! 自动生成。请勿手动修改。

use crate::PPtr;


/// 版本枚举：AnimatorMControllerVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorMControllerVariant {
    Default(PPtr),
    V4_3(PPtr),
}

impl crate::Parse for AnimatorMControllerVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V4_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorMApplyRootMotionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorMApplyRootMotionVariant {
    Default(bool),
    V4_5(bool),
    V5_0(bool),
}

impl crate::Parse for AnimatorMApplyRootMotionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_0(v))
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorMAnimatePhysicsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorMAnimatePhysicsVariant {
    Default(bool),
    V2023_1(bool),
}

impl crate::Parse for AnimatorMAnimatePhysicsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorMLinearVelocityBlendingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorMLinearVelocityBlendingVariant {
    Default(bool),
    V2020_3_43(bool),
    V2021_1(bool),
    V2021_2(bool),
}

impl crate::Parse for AnimatorMLinearVelocityBlendingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2020, 3, 43) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_3_43(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorMAllowConstantClipSamplingOptimizationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorMAllowConstantClipSamplingOptimizationVariant {
    Default(bool),
    V5_0(bool),
    V2018_1(bool),
}

impl crate::Parse for AnimatorMAllowConstantClipSamplingOptimizationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorMStabilizeFeetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorMStabilizeFeetVariant {
    Default(bool),
    V2023_1(bool),
}

impl crate::Parse for AnimatorMStabilizeFeetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimatorMKeepAnimatorStateOnDisableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorMKeepAnimatorStateOnDisableVariant {
    Default(bool),
    V2020_3_46(bool),
    V2021_3_16(bool),
    V2021_3_20(bool),
    V2022_1_24(bool),
    V2022_2_9(bool),
}

impl crate::Parse for AnimatorMKeepAnimatorStateOnDisableVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 9) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_2_9(v))
        } else if *ver >= crate::UnityVersion::new(2022, 1, 24) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_1_24(v))
        } else if *ver >= crate::UnityVersion::new(2021, 3, 20) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_3_20(v))
        } else if *ver >= crate::UnityVersion::new(2021, 3, 16) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_3_16(v))
        } else if *ver >= crate::UnityVersion::new(2020, 3, 46) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_3_46(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Animator {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_avatar: PPtr,
    pub m_controller: AnimatorMControllerVariant,
    pub m_culling_mode: i32,
    pub m_apply_root_motion: AnimatorMApplyRootMotionVariant,
    pub m_update_mode: Option<i32>,
    pub m_animate_physics: Option<AnimatorMAnimatePhysicsVariant>,
    pub m_has_transform_hierarchy: Option<bool>,
    pub m_linear_velocity_blending: Option<AnimatorMLinearVelocityBlendingVariant>,
    pub m_allow_constant_clip_sampling_optimization: Option<AnimatorMAllowConstantClipSamplingOptimizationVariant>,
    pub m_stabilize_feet: Option<AnimatorMStabilizeFeetVariant>,
    pub m_keep_animator_controller_state_on_disable: Option<bool>,
    pub m_keep_animator_state_on_disable: Option<AnimatorMKeepAnimatorStateOnDisableVariant>,
    pub m_write_default_values_on_disable: Option<bool>,
}

impl crate::Parse for Animator {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_avatar = None;
        let mut m_controller = None;
        let mut m_culling_mode = None;
        let mut m_apply_root_motion = None;
        let mut m_update_mode = None;
        let mut m_animate_physics = None;
        let mut m_has_transform_hierarchy = None;
        let mut m_linear_velocity_blending = None;
        let mut m_allow_constant_clip_sampling_optimization = None;
        let mut m_stabilize_feet = None;
        let mut m_keep_animator_controller_state_on_disable = None;
        let mut m_keep_animator_state_on_disable = None;
        let mut m_write_default_values_on_disable = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_avatar = Some(crate::PPtr::parse(rdr, ver)?);
            m_controller = Some(AnimatorMControllerVariant::parse(rdr, ver)?);
            m_culling_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_update_mode = Some(rdr.read_i32()?);
        } else {
        }
            m_apply_root_motion = Some(AnimatorMApplyRootMotionVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_animate_physics = Some(AnimatorMAnimatePhysicsVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable = Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
            m_write_default_values_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 9) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable = Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
            m_write_default_values_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 24) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable = Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_controller_state_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 20) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable = Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
            m_write_default_values_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 16) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable = Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_controller_state_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_controller_state_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 46) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable = Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
            m_write_default_values_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 43) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable = Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_controller_state_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_linear_velocity_blending = Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 3) {
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization = Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_animate_physics = Some(AnimatorMAnimatePhysicsVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
        } else {
            m_animate_physics = Some(AnimatorMAnimatePhysicsVariant::parse(rdr, ver)?);
        }

        Ok(Animator {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_avatar: m_avatar.unwrap(),
            m_controller: m_controller.unwrap(),
            m_culling_mode: m_culling_mode.unwrap(),
            m_apply_root_motion: m_apply_root_motion.unwrap(),
            m_update_mode,
            m_animate_physics,
            m_has_transform_hierarchy,
            m_linear_velocity_blending,
            m_allow_constant_clip_sampling_optimization,
            m_stabilize_feet,
            m_keep_animator_controller_state_on_disable,
            m_keep_animator_state_on_disable,
            m_write_default_values_on_disable,
        })
    }
}
