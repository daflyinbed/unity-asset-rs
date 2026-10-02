//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：PhysicsManagerMDefaultMaterialVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMDefaultMaterialVariant {
    Default(PPtr),
    V2023_3(PPtr),
}

impl crate::Parse for PhysicsManagerMDefaultMaterialVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V2023_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMDefaultMaterialVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PhysicsManagerMRaycastsHitTriggersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMRaycastsHitTriggersVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for PhysicsManagerMRaycastsHitTriggersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMRaycastsHitTriggersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PhysicsManagerMEnableAdaptiveForceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMEnableAdaptiveForceVariant {
    Default(bool),
    V5_5(bool),
    V2017_3(bool),
}

impl crate::Parse for PhysicsManagerMEnableAdaptiveForceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMEnableAdaptiveForceVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PhysicsManagerMLayerCollisionMatrixVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMLayerCollisionMatrixVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for PhysicsManagerMLayerCollisionMatrixVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMLayerCollisionMatrixVariant {
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

/// 版本枚举：PhysicsManagerMClothInterCollisionStiffnessVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMClothInterCollisionStiffnessVariant {
    Default(f32),
    V2023_1(f32),
}

impl crate::Parse for PhysicsManagerMClothInterCollisionStiffnessVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMClothInterCollisionStiffnessVariant {
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

/// 版本枚举：PhysicsManagerMBroadphaseTypeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMBroadphaseTypeVariant {
    Default(i32),
    V2023_1(i32),
    V6000_0_59(i32),
    V6000_1(i32),
    V6000_2_6(i32),
}

impl crate::Parse for PhysicsManagerMBroadphaseTypeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 2, 6) {
            let v = rdr.read_i32()?;
            Ok(Self::V6000_2_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V6000_1(v))
        } else if *ver >= crate::UnityVersion::new(6000, 0, 59) {
            let v = rdr.read_i32()?;
            Ok(Self::V6000_0_59(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMBroadphaseTypeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
            Self::V6000_0_59(v) => v.to_value(),
            Self::V6000_1(v) => v.to_value(),
            Self::V6000_2_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_0_59(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_1(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_2_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PhysicsManagerMWorldSubdivisionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMWorldSubdivisionsVariant {
    Default(i32),
    V6000_0_59(i32),
}

impl crate::Parse for PhysicsManagerMWorldSubdivisionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 0, 59) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V6000_0_59(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMWorldSubdivisionsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_0_59(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_0_59(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PhysicsManagerMEnableUnifiedHeightmapsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMEnableUnifiedHeightmapsVariant {
    Default(bool),
    V2019_1(bool),
    V2021_2(bool),
}

impl crate::Parse for PhysicsManagerMEnableUnifiedHeightmapsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMEnableUnifiedHeightmapsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PhysicsManagerMImprovedPatchFrictionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PhysicsManagerMImprovedPatchFrictionVariant {
    Default(bool),
    V6000_0_50(bool),
    V6000_1(bool),
    V6000_1_4(bool),
}

impl crate::Parse for PhysicsManagerMImprovedPatchFrictionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 1, 4) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_1_4(v))
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_1(v))
        } else if *ver >= crate::UnityVersion::new(6000, 0, 50) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_0_50(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PhysicsManagerMImprovedPatchFrictionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_0_50(v) => v.to_value(),
            Self::V6000_1(v) => v.to_value(),
            Self::V6000_1_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_0_50(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_1(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_1_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PhysicsManager {
    pub m_gravity: Vector3f,
    pub m_default_material: PhysicsManagerMDefaultMaterialVariant,
    pub m_bounce_threshold: f32,
    pub m_sleep_velocity: Option<f32>,
    pub m_sleep_threshold: Option<f32>,
    pub m_default_max_depenetration_velocity: Option<f32>,
    pub m_sleep_angular_velocity: Option<f32>,
    pub m_default_contact_offset: Option<f32>,
    pub m_max_angular_velocity: Option<f32>,
    pub m_solver_iteration_count: Option<i32>,
    pub m_default_solver_iterations: Option<i32>,
    pub m_min_penetration_for_penalty: Option<f32>,
    pub m_raycasts_hit_triggers: Option<PhysicsManagerMRaycastsHitTriggersVariant>,
    pub m_queries_hit_triggers: Option<bool>,
    pub m_solver_velocity_iterations: Option<i32>,
    pub m_default_solver_velocity_iterations: Option<i32>,
    pub m_enable_adaptive_force: Option<PhysicsManagerMEnableAdaptiveForceVariant>,
    pub m_queries_hit_backfaces: Option<bool>,
    pub m_layer_collision_matrix: PhysicsManagerMLayerCollisionMatrixVariant,
    pub m_enable_pcm: Option<bool>,
    pub m_cloth_inter_collision_distance: Option<f32>,
    pub m_cloth_inter_collision_stiffness:
        Option<PhysicsManagerMClothInterCollisionStiffnessVariant>,
    pub m_auto_simulation: Option<bool>,
    pub m_contacts_generation: Option<i32>,
    pub m_auto_sync_transforms: Option<bool>,
    pub m_simulation_mode: Option<i32>,
    pub m_cloth_inter_collision_settings_toggle: Option<bool>,
    pub m_reuse_collision_callbacks: Option<bool>,
    pub m_contact_pairs_mode: Option<i32>,
    pub m_invoke_collision_callbacks: Option<bool>,
    pub m_broadphase_type: Option<PhysicsManagerMBroadphaseTypeVariant>,
    pub m_cloth_gravity: Option<Vector3f>,
    pub m_world_bounds: Option<Aabb>,
    pub m_world_subdivisions: Option<PhysicsManagerMWorldSubdivisionsVariant>,
    pub m_friction_type: Option<i32>,
    pub m_enable_enhanced_determinism: Option<bool>,
    pub m_enable_unified_heightmaps: Option<PhysicsManagerMEnableUnifiedHeightmapsVariant>,
    pub m_improved_patch_friction: Option<PhysicsManagerMImprovedPatchFrictionVariant>,
    pub m_solver_type: Option<i32>,
    pub m_generate_on_trigger_stay_events: Option<bool>,
    pub m_default_max_angular_speed: Option<f32>,
    pub m_scratch_buffer_chunk_count: Option<u32>,
    pub m_current_backend_id: Option<u32>,
    pub m_fast_motion_threshold: Option<f32>,
    pub m_scene_buffers_release_interval: Option<u32>,
    pub m_release_scene_buffers: Option<bool>,
    pub m_log_verbosity: Option<i32>,
    pub m_incremental_static_broadphase: Option<bool>,
}

impl crate::Parse for PhysicsManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_gravity = None;
        let mut m_default_material = None;
        let mut m_bounce_threshold = None;
        let mut m_sleep_velocity = None;
        let mut m_sleep_threshold = None;
        let mut m_default_max_depenetration_velocity = None;
        let mut m_sleep_angular_velocity = None;
        let mut m_default_contact_offset = None;
        let mut m_max_angular_velocity = None;
        let mut m_solver_iteration_count = None;
        let mut m_default_solver_iterations = None;
        let mut m_min_penetration_for_penalty = None;
        let mut m_raycasts_hit_triggers = None;
        let mut m_queries_hit_triggers = None;
        let mut m_solver_velocity_iterations = None;
        let mut m_default_solver_velocity_iterations = None;
        let mut m_enable_adaptive_force = None;
        let mut m_queries_hit_backfaces = None;
        let mut m_layer_collision_matrix = None;
        let mut m_enable_pcm = None;
        let mut m_cloth_inter_collision_distance = None;
        let mut m_cloth_inter_collision_stiffness = None;
        let mut m_auto_simulation = None;
        let mut m_contacts_generation = None;
        let mut m_auto_sync_transforms = None;
        let mut m_simulation_mode = None;
        let mut m_cloth_inter_collision_settings_toggle = None;
        let mut m_reuse_collision_callbacks = None;
        let mut m_contact_pairs_mode = None;
        let mut m_invoke_collision_callbacks = None;
        let mut m_broadphase_type = None;
        let mut m_cloth_gravity = None;
        let mut m_world_bounds = None;
        let mut m_world_subdivisions = None;
        let mut m_friction_type = None;
        let mut m_enable_enhanced_determinism = None;
        let mut m_enable_unified_heightmaps = None;
        let mut m_improved_patch_friction = None;
        let mut m_solver_type = None;
        let mut m_generate_on_trigger_stay_events = None;
        let mut m_default_max_angular_speed = None;
        let mut m_scratch_buffer_chunk_count = None;
        let mut m_current_backend_id = None;
        let mut m_fast_motion_threshold = None;
        let mut m_scene_buffers_release_interval = None;
        let mut m_release_scene_buffers = None;
        let mut m_log_verbosity = None;
        let mut m_incremental_static_broadphase = None;
        m_gravity = Some(Vector3f::parse(rdr, ver)?);
        m_default_material = Some(PhysicsManagerMDefaultMaterialVariant::parse(rdr, ver)?);
        m_bounce_threshold = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_default_max_depenetration_velocity = Some(rdr.read_f32()?);
            m_sleep_threshold = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_default_solver_iterations = Some(rdr.read_i32()?);
            m_default_solver_velocity_iterations = Some(rdr.read_i32()?);
            m_queries_hit_backfaces = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_enable_adaptive_force =
                Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(rdr, ver)?);
            m_cloth_inter_collision_distance = Some(rdr.read_f32()?);
            m_cloth_inter_collision_stiffness = Some(
                PhysicsManagerMClothInterCollisionStiffnessVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_default_max_depenetration_velocity = Some(rdr.read_f32()?);
            m_sleep_threshold = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_default_solver_iterations = Some(rdr.read_i32()?);
            m_default_solver_velocity_iterations = Some(rdr.read_i32()?);
            m_queries_hit_backfaces = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_enable_adaptive_force =
                Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(rdr, ver)?);
            m_cloth_inter_collision_distance = Some(rdr.read_f32()?);
            m_cloth_inter_collision_stiffness = Some(
                PhysicsManagerMClothInterCollisionStiffnessVariant::parse(rdr, ver)?,
            );
            m_contacts_generation = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_sleep_threshold = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_default_solver_iterations = Some(rdr.read_i32()?);
            m_default_solver_velocity_iterations = Some(rdr.read_i32()?);
            m_queries_hit_backfaces = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_enable_adaptive_force =
                Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(rdr, ver)?);
            m_cloth_inter_collision_distance = Some(rdr.read_f32()?);
            m_cloth_inter_collision_stiffness = Some(
                PhysicsManagerMClothInterCollisionStiffnessVariant::parse(rdr, ver)?,
            );
            m_contacts_generation = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_sleep_threshold = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_default_solver_iterations = Some(rdr.read_i32()?);
            m_default_solver_velocity_iterations = Some(rdr.read_i32()?);
            m_queries_hit_backfaces = Some(rdr.read_bool()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_enable_adaptive_force =
                Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(rdr, ver)?);
            m_enable_pcm = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_sleep_threshold = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_solver_iteration_count = Some(rdr.read_i32()?);
            m_solver_velocity_iterations = Some(rdr.read_i32()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_enable_adaptive_force =
                Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_sleep_threshold = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_solver_iteration_count = Some(rdr.read_i32()?);
            m_queries_hit_triggers = Some(rdr.read_bool()?);
            m_enable_adaptive_force =
                Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_sleep_threshold = Some(rdr.read_f32()?);
            m_default_contact_offset = Some(rdr.read_f32()?);
            m_solver_iteration_count = Some(rdr.read_i32()?);
            m_raycasts_hit_triggers =
                Some(PhysicsManagerMRaycastsHitTriggersVariant::parse(rdr, ver)?);
            m_enable_adaptive_force =
                Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(rdr, ver)?);
        } else {
            m_sleep_velocity = Some(rdr.read_f32()?);
            m_sleep_angular_velocity = Some(rdr.read_f32()?);
            m_max_angular_velocity = Some(rdr.read_f32()?);
            m_min_penetration_for_penalty = Some(rdr.read_f32()?);
            m_solver_iteration_count = Some(rdr.read_i32()?);
            m_raycasts_hit_triggers =
                Some(PhysicsManagerMRaycastsHitTriggersVariant::parse(rdr, ver)?);
        }
        m_layer_collision_matrix =
            Some(PhysicsManagerMLayerCollisionMatrixVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 7) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
            m_scene_buffers_release_interval = Some(rdr.read_u32()?);
            m_release_scene_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_verbosity = Some(rdr.read_i32()?);
            m_incremental_static_broadphase = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 3, 3) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
            m_scene_buffers_release_interval = Some(rdr.read_u32()?);
            m_release_scene_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_verbosity = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 1) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
            m_scene_buffers_release_interval = Some(rdr.read_u32()?);
            m_release_scene_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 2, 6) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 4) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 67) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
            m_scene_buffers_release_interval = Some(rdr.read_u32()?);
            m_release_scene_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_verbosity = Some(rdr.read_i32()?);
            m_incremental_static_broadphase = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 0, 65) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
            m_scene_buffers_release_interval = Some(rdr.read_u32()?);
            m_release_scene_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_log_verbosity = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 59) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 50) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_generate_on_trigger_stay_events = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 26) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_current_backend_id = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 12) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_scratch_buffer_chunk_count = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 21) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
            m_fast_motion_threshold = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_simulation_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_invoke_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_auto_simulation = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(
                rdr, ver,
            )?);
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_auto_simulation = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
            m_solver_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_default_max_angular_speed = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_auto_simulation = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(rdr, ver)?);
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
            m_default_max_angular_speed = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_auto_simulation = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_reuse_collision_callbacks = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
            m_friction_type = Some(rdr.read_i32()?);
            m_enable_enhanced_determinism = Some(rdr.read_bool()?);
            m_enable_unified_heightmaps = Some(
                PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_auto_simulation = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_contact_pairs_mode = Some(rdr.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(rdr, ver)?);
            m_world_bounds = Some(Aabb::parse(rdr, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_auto_simulation = Some(rdr.read_bool()?);
            m_auto_sync_transforms = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_auto_simulation = Some(rdr.read_bool()?);
        } else {
        }

        Ok(PhysicsManager {
            m_gravity: m_gravity.unwrap(),
            m_default_material: m_default_material.unwrap(),
            m_bounce_threshold: m_bounce_threshold.unwrap(),
            m_sleep_velocity,
            m_sleep_threshold,
            m_default_max_depenetration_velocity,
            m_sleep_angular_velocity,
            m_default_contact_offset,
            m_max_angular_velocity,
            m_solver_iteration_count,
            m_default_solver_iterations,
            m_min_penetration_for_penalty,
            m_raycasts_hit_triggers,
            m_queries_hit_triggers,
            m_solver_velocity_iterations,
            m_default_solver_velocity_iterations,
            m_enable_adaptive_force,
            m_queries_hit_backfaces,
            m_layer_collision_matrix: m_layer_collision_matrix.unwrap(),
            m_enable_pcm,
            m_cloth_inter_collision_distance,
            m_cloth_inter_collision_stiffness,
            m_auto_simulation,
            m_contacts_generation,
            m_auto_sync_transforms,
            m_simulation_mode,
            m_cloth_inter_collision_settings_toggle,
            m_reuse_collision_callbacks,
            m_contact_pairs_mode,
            m_invoke_collision_callbacks,
            m_broadphase_type,
            m_cloth_gravity,
            m_world_bounds,
            m_world_subdivisions,
            m_friction_type,
            m_enable_enhanced_determinism,
            m_enable_unified_heightmaps,
            m_improved_patch_friction,
            m_solver_type,
            m_generate_on_trigger_stay_events,
            m_default_max_angular_speed,
            m_scratch_buffer_chunk_count,
            m_current_backend_id,
            m_fast_motion_threshold,
            m_scene_buffers_release_interval,
            m_release_scene_buffers,
            m_log_verbosity,
            m_incremental_static_broadphase,
        })
    }
}

impl crate::ToValue for PhysicsManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Gravity".to_string(), self.m_gravity.to_value())),
            Some((
                "m_DefaultMaterial".to_string(),
                self.m_default_material.to_value(),
            )),
            Some((
                "m_BounceThreshold".to_string(),
                self.m_bounce_threshold.to_value(),
            )),
            crate::opt_field("m_SleepVelocity", &self.m_sleep_velocity),
            crate::opt_field("m_SleepThreshold", &self.m_sleep_threshold),
            crate::opt_field(
                "m_DefaultMaxDepenetrationVelocity",
                &self.m_default_max_depenetration_velocity,
            ),
            crate::opt_field("m_SleepAngularVelocity", &self.m_sleep_angular_velocity),
            crate::opt_field("m_DefaultContactOffset", &self.m_default_contact_offset),
            crate::opt_field("m_MaxAngularVelocity", &self.m_max_angular_velocity),
            crate::opt_field("m_SolverIterationCount", &self.m_solver_iteration_count),
            crate::opt_field(
                "m_DefaultSolverIterations",
                &self.m_default_solver_iterations,
            ),
            crate::opt_field(
                "m_MinPenetrationForPenalty",
                &self.m_min_penetration_for_penalty,
            ),
            crate::opt_field("m_RaycastsHitTriggers", &self.m_raycasts_hit_triggers),
            crate::opt_field("m_QueriesHitTriggers", &self.m_queries_hit_triggers),
            crate::opt_field(
                "m_SolverVelocityIterations",
                &self.m_solver_velocity_iterations,
            ),
            crate::opt_field(
                "m_DefaultSolverVelocityIterations",
                &self.m_default_solver_velocity_iterations,
            ),
            crate::opt_field("m_EnableAdaptiveForce", &self.m_enable_adaptive_force),
            crate::opt_field("m_QueriesHitBackfaces", &self.m_queries_hit_backfaces),
            Some((
                "m_LayerCollisionMatrix".to_string(),
                self.m_layer_collision_matrix.to_value(),
            )),
            crate::opt_field("m_EnablePCM", &self.m_enable_pcm),
            crate::opt_field(
                "m_ClothInterCollisionDistance",
                &self.m_cloth_inter_collision_distance,
            ),
            crate::opt_field(
                "m_ClothInterCollisionStiffness",
                &self.m_cloth_inter_collision_stiffness,
            ),
            crate::opt_field("m_AutoSimulation", &self.m_auto_simulation),
            crate::opt_field("m_ContactsGeneration", &self.m_contacts_generation),
            crate::opt_field("m_AutoSyncTransforms", &self.m_auto_sync_transforms),
            crate::opt_field("m_SimulationMode", &self.m_simulation_mode),
            crate::opt_field(
                "m_ClothInterCollisionSettingsToggle",
                &self.m_cloth_inter_collision_settings_toggle,
            ),
            crate::opt_field(
                "m_ReuseCollisionCallbacks",
                &self.m_reuse_collision_callbacks,
            ),
            crate::opt_field("m_ContactPairsMode", &self.m_contact_pairs_mode),
            crate::opt_field(
                "m_InvokeCollisionCallbacks",
                &self.m_invoke_collision_callbacks,
            ),
            crate::opt_field("m_BroadphaseType", &self.m_broadphase_type),
            crate::opt_field("m_ClothGravity", &self.m_cloth_gravity),
            crate::opt_field("m_WorldBounds", &self.m_world_bounds),
            crate::opt_field("m_WorldSubdivisions", &self.m_world_subdivisions),
            crate::opt_field("m_FrictionType", &self.m_friction_type),
            crate::opt_field(
                "m_EnableEnhancedDeterminism",
                &self.m_enable_enhanced_determinism,
            ),
            crate::opt_field(
                "m_EnableUnifiedHeightmaps",
                &self.m_enable_unified_heightmaps,
            ),
            crate::opt_field("m_ImprovedPatchFriction", &self.m_improved_patch_friction),
            crate::opt_field("m_SolverType", &self.m_solver_type),
            crate::opt_field(
                "m_GenerateOnTriggerStayEvents",
                &self.m_generate_on_trigger_stay_events,
            ),
            crate::opt_field(
                "m_DefaultMaxAngularSpeed",
                &self.m_default_max_angular_speed,
            ),
            crate::opt_field(
                "m_ScratchBufferChunkCount",
                &self.m_scratch_buffer_chunk_count,
            ),
            crate::opt_field("m_CurrentBackendId", &self.m_current_backend_id),
            crate::opt_field("m_FastMotionThreshold", &self.m_fast_motion_threshold),
            crate::opt_field(
                "m_SceneBuffersReleaseInterval",
                &self.m_scene_buffers_release_interval,
            ),
            crate::opt_field("m_ReleaseSceneBuffers", &self.m_release_scene_buffers),
            crate::opt_field("m_LogVerbosity", &self.m_log_verbosity),
            crate::opt_field(
                "m_IncrementalStaticBroadphase",
                &self.m_incremental_static_broadphase,
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
                "m_Gravity" => (self.m_gravity).inspect_value(path, offset, limit),
                "m_DefaultMaterial" => (self.m_default_material).inspect_value(path, offset, limit),
                "m_BounceThreshold" => (self.m_bounce_threshold).inspect_value(path, offset, limit),
                "m_SleepVelocity" => (self.m_sleep_velocity).inspect_value(path, offset, limit),
                "m_SleepThreshold" => (self.m_sleep_threshold).inspect_value(path, offset, limit),
                "m_DefaultMaxDepenetrationVelocity" => {
                    (self.m_default_max_depenetration_velocity).inspect_value(path, offset, limit)
                }
                "m_SleepAngularVelocity" => {
                    (self.m_sleep_angular_velocity).inspect_value(path, offset, limit)
                }
                "m_DefaultContactOffset" => {
                    (self.m_default_contact_offset).inspect_value(path, offset, limit)
                }
                "m_MaxAngularVelocity" => {
                    (self.m_max_angular_velocity).inspect_value(path, offset, limit)
                }
                "m_SolverIterationCount" => {
                    (self.m_solver_iteration_count).inspect_value(path, offset, limit)
                }
                "m_DefaultSolverIterations" => {
                    (self.m_default_solver_iterations).inspect_value(path, offset, limit)
                }
                "m_MinPenetrationForPenalty" => {
                    (self.m_min_penetration_for_penalty).inspect_value(path, offset, limit)
                }
                "m_RaycastsHitTriggers" => {
                    (self.m_raycasts_hit_triggers).inspect_value(path, offset, limit)
                }
                "m_QueriesHitTriggers" => {
                    (self.m_queries_hit_triggers).inspect_value(path, offset, limit)
                }
                "m_SolverVelocityIterations" => {
                    (self.m_solver_velocity_iterations).inspect_value(path, offset, limit)
                }
                "m_DefaultSolverVelocityIterations" => {
                    (self.m_default_solver_velocity_iterations).inspect_value(path, offset, limit)
                }
                "m_EnableAdaptiveForce" => {
                    (self.m_enable_adaptive_force).inspect_value(path, offset, limit)
                }
                "m_QueriesHitBackfaces" => {
                    (self.m_queries_hit_backfaces).inspect_value(path, offset, limit)
                }
                "m_LayerCollisionMatrix" => {
                    (self.m_layer_collision_matrix).inspect_value(path, offset, limit)
                }
                "m_EnablePCM" => (self.m_enable_pcm).inspect_value(path, offset, limit),
                "m_ClothInterCollisionDistance" => {
                    (self.m_cloth_inter_collision_distance).inspect_value(path, offset, limit)
                }
                "m_ClothInterCollisionStiffness" => {
                    (self.m_cloth_inter_collision_stiffness).inspect_value(path, offset, limit)
                }
                "m_AutoSimulation" => (self.m_auto_simulation).inspect_value(path, offset, limit),
                "m_ContactsGeneration" => {
                    (self.m_contacts_generation).inspect_value(path, offset, limit)
                }
                "m_AutoSyncTransforms" => {
                    (self.m_auto_sync_transforms).inspect_value(path, offset, limit)
                }
                "m_SimulationMode" => (self.m_simulation_mode).inspect_value(path, offset, limit),
                "m_ClothInterCollisionSettingsToggle" => (self
                    .m_cloth_inter_collision_settings_toggle)
                    .inspect_value(path, offset, limit),
                "m_ReuseCollisionCallbacks" => {
                    (self.m_reuse_collision_callbacks).inspect_value(path, offset, limit)
                }
                "m_ContactPairsMode" => {
                    (self.m_contact_pairs_mode).inspect_value(path, offset, limit)
                }
                "m_InvokeCollisionCallbacks" => {
                    (self.m_invoke_collision_callbacks).inspect_value(path, offset, limit)
                }
                "m_BroadphaseType" => (self.m_broadphase_type).inspect_value(path, offset, limit),
                "m_ClothGravity" => (self.m_cloth_gravity).inspect_value(path, offset, limit),
                "m_WorldBounds" => (self.m_world_bounds).inspect_value(path, offset, limit),
                "m_WorldSubdivisions" => {
                    (self.m_world_subdivisions).inspect_value(path, offset, limit)
                }
                "m_FrictionType" => (self.m_friction_type).inspect_value(path, offset, limit),
                "m_EnableEnhancedDeterminism" => {
                    (self.m_enable_enhanced_determinism).inspect_value(path, offset, limit)
                }
                "m_EnableUnifiedHeightmaps" => {
                    (self.m_enable_unified_heightmaps).inspect_value(path, offset, limit)
                }
                "m_ImprovedPatchFriction" => {
                    (self.m_improved_patch_friction).inspect_value(path, offset, limit)
                }
                "m_SolverType" => (self.m_solver_type).inspect_value(path, offset, limit),
                "m_GenerateOnTriggerStayEvents" => {
                    (self.m_generate_on_trigger_stay_events).inspect_value(path, offset, limit)
                }
                "m_DefaultMaxAngularSpeed" => {
                    (self.m_default_max_angular_speed).inspect_value(path, offset, limit)
                }
                "m_ScratchBufferChunkCount" => {
                    (self.m_scratch_buffer_chunk_count).inspect_value(path, offset, limit)
                }
                "m_CurrentBackendId" => {
                    (self.m_current_backend_id).inspect_value(path, offset, limit)
                }
                "m_FastMotionThreshold" => {
                    (self.m_fast_motion_threshold).inspect_value(path, offset, limit)
                }
                "m_SceneBuffersReleaseInterval" => {
                    (self.m_scene_buffers_release_interval).inspect_value(path, offset, limit)
                }
                "m_ReleaseSceneBuffers" => {
                    (self.m_release_scene_buffers).inspect_value(path, offset, limit)
                }
                "m_LogVerbosity" => (self.m_log_verbosity).inspect_value(path, offset, limit),
                "m_IncrementalStaticBroadphase" => {
                    (self.m_incremental_static_broadphase).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_gravity).inspect_value(path, offset, limit))
                .map(|v| ("m_Gravity".to_string(), v)),
            ((self.m_default_material).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultMaterial".to_string(), v)),
            ((self.m_bounce_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceThreshold".to_string(), v)),
            ((self.m_sleep_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_SleepVelocity".to_string(), v)),
            ((self.m_sleep_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_SleepThreshold".to_string(), v)),
            ((self.m_default_max_depenetration_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultMaxDepenetrationVelocity".to_string(), v)),
            ((self.m_sleep_angular_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_SleepAngularVelocity".to_string(), v)),
            ((self.m_default_contact_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultContactOffset".to_string(), v)),
            ((self.m_max_angular_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxAngularVelocity".to_string(), v)),
            ((self.m_solver_iteration_count).inspect_value(path, offset, limit))
                .map(|v| ("m_SolverIterationCount".to_string(), v)),
            ((self.m_default_solver_iterations).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultSolverIterations".to_string(), v)),
            ((self.m_min_penetration_for_penalty).inspect_value(path, offset, limit))
                .map(|v| ("m_MinPenetrationForPenalty".to_string(), v)),
            ((self.m_raycasts_hit_triggers).inspect_value(path, offset, limit))
                .map(|v| ("m_RaycastsHitTriggers".to_string(), v)),
            ((self.m_queries_hit_triggers).inspect_value(path, offset, limit))
                .map(|v| ("m_QueriesHitTriggers".to_string(), v)),
            ((self.m_solver_velocity_iterations).inspect_value(path, offset, limit))
                .map(|v| ("m_SolverVelocityIterations".to_string(), v)),
            ((self.m_default_solver_velocity_iterations).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultSolverVelocityIterations".to_string(), v)),
            ((self.m_enable_adaptive_force).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableAdaptiveForce".to_string(), v)),
            ((self.m_queries_hit_backfaces).inspect_value(path, offset, limit))
                .map(|v| ("m_QueriesHitBackfaces".to_string(), v)),
            ((self.m_layer_collision_matrix).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerCollisionMatrix".to_string(), v)),
            ((self.m_enable_pcm).inspect_value(path, offset, limit))
                .map(|v| ("m_EnablePCM".to_string(), v)),
            ((self.m_cloth_inter_collision_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_ClothInterCollisionDistance".to_string(), v)),
            ((self.m_cloth_inter_collision_stiffness).inspect_value(path, offset, limit))
                .map(|v| ("m_ClothInterCollisionStiffness".to_string(), v)),
            ((self.m_auto_simulation).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoSimulation".to_string(), v)),
            ((self.m_contacts_generation).inspect_value(path, offset, limit))
                .map(|v| ("m_ContactsGeneration".to_string(), v)),
            ((self.m_auto_sync_transforms).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoSyncTransforms".to_string(), v)),
            ((self.m_simulation_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_SimulationMode".to_string(), v)),
            ((self.m_cloth_inter_collision_settings_toggle).inspect_value(path, offset, limit))
                .map(|v| ("m_ClothInterCollisionSettingsToggle".to_string(), v)),
            ((self.m_reuse_collision_callbacks).inspect_value(path, offset, limit))
                .map(|v| ("m_ReuseCollisionCallbacks".to_string(), v)),
            ((self.m_contact_pairs_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ContactPairsMode".to_string(), v)),
            ((self.m_invoke_collision_callbacks).inspect_value(path, offset, limit))
                .map(|v| ("m_InvokeCollisionCallbacks".to_string(), v)),
            ((self.m_broadphase_type).inspect_value(path, offset, limit))
                .map(|v| ("m_BroadphaseType".to_string(), v)),
            ((self.m_cloth_gravity).inspect_value(path, offset, limit))
                .map(|v| ("m_ClothGravity".to_string(), v)),
            ((self.m_world_bounds).inspect_value(path, offset, limit))
                .map(|v| ("m_WorldBounds".to_string(), v)),
            ((self.m_world_subdivisions).inspect_value(path, offset, limit))
                .map(|v| ("m_WorldSubdivisions".to_string(), v)),
            ((self.m_friction_type).inspect_value(path, offset, limit))
                .map(|v| ("m_FrictionType".to_string(), v)),
            ((self.m_enable_enhanced_determinism).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableEnhancedDeterminism".to_string(), v)),
            ((self.m_enable_unified_heightmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableUnifiedHeightmaps".to_string(), v)),
            ((self.m_improved_patch_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_ImprovedPatchFriction".to_string(), v)),
            ((self.m_solver_type).inspect_value(path, offset, limit))
                .map(|v| ("m_SolverType".to_string(), v)),
            ((self.m_generate_on_trigger_stay_events).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateOnTriggerStayEvents".to_string(), v)),
            ((self.m_default_max_angular_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultMaxAngularSpeed".to_string(), v)),
            ((self.m_scratch_buffer_chunk_count).inspect_value(path, offset, limit))
                .map(|v| ("m_ScratchBufferChunkCount".to_string(), v)),
            ((self.m_current_backend_id).inspect_value(path, offset, limit))
                .map(|v| ("m_CurrentBackendId".to_string(), v)),
            ((self.m_fast_motion_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_FastMotionThreshold".to_string(), v)),
            ((self.m_scene_buffers_release_interval).inspect_value(path, offset, limit))
                .map(|v| ("m_SceneBuffersReleaseInterval".to_string(), v)),
            ((self.m_release_scene_buffers).inspect_value(path, offset, limit))
                .map(|v| ("m_ReleaseSceneBuffers".to_string(), v)),
            ((self.m_log_verbosity).inspect_value(path, offset, limit))
                .map(|v| ("m_LogVerbosity".to_string(), v)),
            ((self.m_incremental_static_broadphase).inspect_value(path, offset, limit))
                .map(|v| ("m_IncrementalStaticBroadphase".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
