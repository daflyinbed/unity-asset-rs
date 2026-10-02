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

impl crate::ToValue for AnimatorMControllerVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for AnimatorMApplyRootMotionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for AnimatorMAnimatePhysicsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for AnimatorMLinearVelocityBlendingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_3_43(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_3_43(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for AnimatorMAllowConstantClipSamplingOptimizationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V2018_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for AnimatorMStabilizeFeetVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for AnimatorMKeepAnimatorStateOnDisableVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_3_46(v) => v.to_value(),
            Self::V2021_3_16(v) => v.to_value(),
            Self::V2021_3_20(v) => v.to_value(),
            Self::V2022_1_24(v) => v.to_value(),
            Self::V2022_2_9(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_3_46(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_3_16(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_3_20(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1_24(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2_9(v) => (v).inspect_value(path, offset, limit),
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
    pub m_allow_constant_clip_sampling_optimization:
        Option<AnimatorMAllowConstantClipSamplingOptimizationVariant>,
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
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_animate_physics = Some(AnimatorMAnimatePhysicsVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable =
                Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
            m_write_default_values_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 9) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable =
                Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
            m_write_default_values_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 24) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable =
                Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_controller_state_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 20) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable =
                Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
            m_write_default_values_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 16) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable =
                Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_controller_state_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_controller_state_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 46) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable =
                Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
            m_write_default_values_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 43) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_stabilize_feet = Some(AnimatorMStabilizeFeetVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_state_on_disable =
                Some(AnimatorMKeepAnimatorStateOnDisableVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
            m_keep_animator_controller_state_on_disable = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_linear_velocity_blending =
                Some(AnimatorMLinearVelocityBlendingVariant::parse(rdr, ver)?);
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 3) {
            m_has_transform_hierarchy = Some(rdr.read_bool()?);
            m_allow_constant_clip_sampling_optimization =
                Some(AnimatorMAllowConstantClipSamplingOptimizationVariant::parse(rdr, ver)?);
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

impl crate::ToValue for Animator {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Avatar".to_string(), self.m_avatar.to_value())),
            Some(("m_Controller".to_string(), self.m_controller.to_value())),
            Some(("m_CullingMode".to_string(), self.m_culling_mode.to_value())),
            Some((
                "m_ApplyRootMotion".to_string(),
                self.m_apply_root_motion.to_value(),
            )),
            crate::opt_field("m_UpdateMode", &self.m_update_mode),
            crate::opt_field("m_AnimatePhysics", &self.m_animate_physics),
            crate::opt_field("m_HasTransformHierarchy", &self.m_has_transform_hierarchy),
            crate::opt_field("m_LinearVelocityBlending", &self.m_linear_velocity_blending),
            crate::opt_field(
                "m_AllowConstantClipSamplingOptimization",
                &self.m_allow_constant_clip_sampling_optimization,
            ),
            crate::opt_field("m_StabilizeFeet", &self.m_stabilize_feet),
            crate::opt_field(
                "m_KeepAnimatorControllerStateOnDisable",
                &self.m_keep_animator_controller_state_on_disable,
            ),
            crate::opt_field(
                "m_KeepAnimatorStateOnDisable",
                &self.m_keep_animator_state_on_disable,
            ),
            crate::opt_field(
                "m_WriteDefaultValuesOnDisable",
                &self.m_write_default_values_on_disable,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Avatar" => (self.m_avatar).inspect_value(path, offset, limit),
                "m_Controller" => (self.m_controller).inspect_value(path, offset, limit),
                "m_CullingMode" => (self.m_culling_mode).inspect_value(path, offset, limit),
                "m_ApplyRootMotion" => {
                    (self.m_apply_root_motion).inspect_value(path, offset, limit)
                }
                "m_UpdateMode" => (self.m_update_mode).inspect_value(path, offset, limit),
                "m_AnimatePhysics" => (self.m_animate_physics).inspect_value(path, offset, limit),
                "m_HasTransformHierarchy" => {
                    (self.m_has_transform_hierarchy).inspect_value(path, offset, limit)
                }
                "m_LinearVelocityBlending" => {
                    (self.m_linear_velocity_blending).inspect_value(path, offset, limit)
                }
                "m_AllowConstantClipSamplingOptimization" => (self
                    .m_allow_constant_clip_sampling_optimization)
                    .inspect_value(path, offset, limit),
                "m_StabilizeFeet" => (self.m_stabilize_feet).inspect_value(path, offset, limit),
                "m_KeepAnimatorControllerStateOnDisable" => (self
                    .m_keep_animator_controller_state_on_disable)
                    .inspect_value(path, offset, limit),
                "m_KeepAnimatorStateOnDisable" => {
                    (self.m_keep_animator_state_on_disable).inspect_value(path, offset, limit)
                }
                "m_WriteDefaultValuesOnDisable" => {
                    (self.m_write_default_values_on_disable).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_avatar).inspect_value(path, offset, limit))
                .map(|v| ("m_Avatar".to_string(), v)),
            ((self.m_controller).inspect_value(path, offset, limit))
                .map(|v| ("m_Controller".to_string(), v)),
            ((self.m_culling_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_CullingMode".to_string(), v)),
            ((self.m_apply_root_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_ApplyRootMotion".to_string(), v)),
            ((self.m_update_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateMode".to_string(), v)),
            ((self.m_animate_physics).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimatePhysics".to_string(), v)),
            ((self.m_has_transform_hierarchy).inspect_value(path, offset, limit))
                .map(|v| ("m_HasTransformHierarchy".to_string(), v)),
            ((self.m_linear_velocity_blending).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearVelocityBlending".to_string(), v)),
            ((self.m_allow_constant_clip_sampling_optimization).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowConstantClipSamplingOptimization".to_string(), v)),
            ((self.m_stabilize_feet).inspect_value(path, offset, limit))
                .map(|v| ("m_StabilizeFeet".to_string(), v)),
            ((self.m_keep_animator_controller_state_on_disable).inspect_value(path, offset, limit))
                .map(|v| ("m_KeepAnimatorControllerStateOnDisable".to_string(), v)),
            ((self.m_keep_animator_state_on_disable).inspect_value(path, offset, limit))
                .map(|v| ("m_KeepAnimatorStateOnDisable".to_string(), v)),
            ((self.m_write_default_values_on_disable).inspect_value(path, offset, limit))
                .map(|v| ("m_WriteDefaultValuesOnDisable".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
