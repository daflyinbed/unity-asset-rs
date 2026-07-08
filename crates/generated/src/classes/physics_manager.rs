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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::V2023_3(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2017_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = r.read_f32()?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 2, 6) {
            let v = r.read_i32()?;
            Ok(Self::V6000_2_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Ok(Self::V6000_1(v))
        } else if *ver >= crate::UnityVersion::new(6000, 0, 59) {
            let v = r.read_i32()?;
            Ok(Self::V6000_0_59(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = r.read_i32()?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 0, 59) {
            let v = r.read_i32()?;
            r.align(4)?;
            Ok(Self::V6000_0_59(v))
        } else {
            let v = r.read_i32()?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2021_2(v))
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 1, 4) {
            let v = r.read_bool()?;
            Ok(Self::V6000_1_4(v))
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V6000_1(v))
        } else if *ver >= crate::UnityVersion::new(6000, 0, 50) {
            let v = r.read_bool()?;
            Ok(Self::V6000_0_50(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
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
    pub m_cloth_inter_collision_stiffness: Option<PhysicsManagerMClothInterCollisionStiffnessVariant>,
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
        r: &mut R,
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
            m_gravity = Some(Vector3f::parse(r, ver)?);
            m_default_material = Some(PhysicsManagerMDefaultMaterialVariant::parse(r, ver)?);
            m_bounce_threshold = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_default_max_depenetration_velocity = Some(r.read_f32()?);
            m_sleep_threshold = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_default_solver_iterations = Some(r.read_i32()?);
            m_default_solver_velocity_iterations = Some(r.read_i32()?);
            m_queries_hit_backfaces = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_enable_adaptive_force = Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(r, ver)?);
            m_cloth_inter_collision_distance = Some(r.read_f32()?);
            m_cloth_inter_collision_stiffness = Some(PhysicsManagerMClothInterCollisionStiffnessVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_default_max_depenetration_velocity = Some(r.read_f32()?);
            m_sleep_threshold = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_default_solver_iterations = Some(r.read_i32()?);
            m_default_solver_velocity_iterations = Some(r.read_i32()?);
            m_queries_hit_backfaces = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_enable_adaptive_force = Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(r, ver)?);
            m_cloth_inter_collision_distance = Some(r.read_f32()?);
            m_cloth_inter_collision_stiffness = Some(PhysicsManagerMClothInterCollisionStiffnessVariant::parse(r, ver)?);
            m_contacts_generation = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_sleep_threshold = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_default_solver_iterations = Some(r.read_i32()?);
            m_default_solver_velocity_iterations = Some(r.read_i32()?);
            m_queries_hit_backfaces = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_enable_adaptive_force = Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(r, ver)?);
            m_cloth_inter_collision_distance = Some(r.read_f32()?);
            m_cloth_inter_collision_stiffness = Some(PhysicsManagerMClothInterCollisionStiffnessVariant::parse(r, ver)?);
            m_contacts_generation = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_sleep_threshold = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_default_solver_iterations = Some(r.read_i32()?);
            m_default_solver_velocity_iterations = Some(r.read_i32()?);
            m_queries_hit_backfaces = Some(r.read_bool()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_enable_adaptive_force = Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(r, ver)?);
            m_enable_pcm = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_sleep_threshold = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_solver_iteration_count = Some(r.read_i32()?);
            m_solver_velocity_iterations = Some(r.read_i32()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_enable_adaptive_force = Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_sleep_threshold = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_solver_iteration_count = Some(r.read_i32()?);
            m_queries_hit_triggers = Some(r.read_bool()?);
            m_enable_adaptive_force = Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_sleep_threshold = Some(r.read_f32()?);
            m_default_contact_offset = Some(r.read_f32()?);
            m_solver_iteration_count = Some(r.read_i32()?);
            m_raycasts_hit_triggers = Some(PhysicsManagerMRaycastsHitTriggersVariant::parse(r, ver)?);
            m_enable_adaptive_force = Some(PhysicsManagerMEnableAdaptiveForceVariant::parse(r, ver)?);
        } else {
            m_sleep_velocity = Some(r.read_f32()?);
            m_sleep_angular_velocity = Some(r.read_f32()?);
            m_max_angular_velocity = Some(r.read_f32()?);
            m_min_penetration_for_penalty = Some(r.read_f32()?);
            m_solver_iteration_count = Some(r.read_i32()?);
            m_raycasts_hit_triggers = Some(PhysicsManagerMRaycastsHitTriggersVariant::parse(r, ver)?);
        }
            m_layer_collision_matrix = Some(PhysicsManagerMLayerCollisionMatrixVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 7) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
            m_scene_buffers_release_interval = Some(r.read_u32()?);
            m_release_scene_buffers = Some(r.read_bool()?);
            r.align(4)?;
            m_log_verbosity = Some(r.read_i32()?);
            m_incremental_static_broadphase = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 3, 3) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
            m_scene_buffers_release_interval = Some(r.read_u32()?);
            m_release_scene_buffers = Some(r.read_bool()?);
            r.align(4)?;
            m_log_verbosity = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 1) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
            m_scene_buffers_release_interval = Some(r.read_u32()?);
            m_release_scene_buffers = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 2, 6) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 4) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 67) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
            m_scene_buffers_release_interval = Some(r.read_u32()?);
            m_release_scene_buffers = Some(r.read_bool()?);
            r.align(4)?;
            m_log_verbosity = Some(r.read_i32()?);
            m_incremental_static_broadphase = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 0, 65) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
            m_scene_buffers_release_interval = Some(r.read_u32()?);
            m_release_scene_buffers = Some(r.read_bool()?);
            r.align(4)?;
            m_log_verbosity = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 59) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 50) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_generate_on_trigger_stay_events = Some(r.read_bool()?);
            r.align(4)?;
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 26) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_current_backend_id = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 12) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_scratch_buffer_chunk_count = Some(r.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 21) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
            m_fast_motion_threshold = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_simulation_mode = Some(r.read_i32()?);
            r.align(4)?;
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_invoke_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_auto_simulation = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
            m_improved_patch_friction = Some(PhysicsManagerMImprovedPatchFrictionVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_auto_simulation = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
            m_solver_type = Some(r.read_i32()?);
            r.align(4)?;
            m_default_max_angular_speed = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_auto_simulation = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_cloth_gravity = Some(Vector3f::parse(r, ver)?);
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
            m_default_max_angular_speed = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_auto_simulation = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_reuse_collision_callbacks = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
            m_friction_type = Some(r.read_i32()?);
            m_enable_enhanced_determinism = Some(r.read_bool()?);
            m_enable_unified_heightmaps = Some(PhysicsManagerMEnableUnifiedHeightmapsVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_auto_simulation = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
            m_cloth_inter_collision_settings_toggle = Some(r.read_bool()?);
            r.align(4)?;
            m_contact_pairs_mode = Some(r.read_i32()?);
            m_broadphase_type = Some(PhysicsManagerMBroadphaseTypeVariant::parse(r, ver)?);
            m_world_bounds = Some(Aabb::parse(r, ver)?);
            m_world_subdivisions = Some(PhysicsManagerMWorldSubdivisionsVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_auto_simulation = Some(r.read_bool()?);
            m_auto_sync_transforms = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_auto_simulation = Some(r.read_bool()?);
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
