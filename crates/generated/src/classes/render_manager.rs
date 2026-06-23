//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::builtin_shader_settings::BuiltinShaderSettings;
use crate::subclasses::platform_shader_defines::PlatformShaderDefines;
use crate::subclasses::platform_shader_settings::PlatformShaderSettings;
use crate::subclasses::tier_graphics_settings::TierGraphicsSettings;
use crate::subclasses::vector3f::Vector3f;
use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct RenderManager {
    pub parent: GlobalGameManager,
    pub m_always_included_shaders: Option<Vec<PPtr>>,
    pub m_deferred: Option<BuiltinShaderSettings>,
    pub m_legacy_deferred: Option<BuiltinShaderSettings>,
    pub m_deferred_reflections: Option<BuiltinShaderSettings>,
    pub m_screen_space_shadows: Option<BuiltinShaderSettings>,
    pub m_preloaded_shaders: Option<Vec<PPtr>>,
    pub m_depth_normals: Option<BuiltinShaderSettings>,
    pub m_motion_vectors: Option<BuiltinShaderSettings>,
    pub m_shader_settings: Option<PlatformShaderSettings>,
    pub m_light_halo: Option<BuiltinShaderSettings>,
    pub m_lens_flare: Option<BuiltinShaderSettings>,
    pub m_video_shaders_include_mode: Option<i32>,
    pub m_shader_settings_tier1: Option<PlatformShaderSettings>,
    pub m_sprites_default_material: Option<PPtr>,
    pub m_preload_shaders_batch_time_limit: Option<i32>,
    pub m_shader_settings_tier2: Option<PlatformShaderSettings>,
    pub m_tier_settings_tier1: Option<TierGraphicsSettings>,
    pub m_transparency_sort_mode: Option<i32>,
    pub m_custom_render_pipeline: Option<PPtr>,
    pub m_graphics_state_collection: Option<PPtr>,
    pub m_shader_settings_tier3: Option<PlatformShaderSettings>,
    pub m_tier_settings_tier2: Option<TierGraphicsSettings>,
    pub m_transparency_sort_axis: Option<Vector3f>,
    pub m_collection_startup_action: Option<i32>,
    pub m_tier_settings_tier3: Option<TierGraphicsSettings>,
    pub m_trace_save_path: Option<String>,
    pub m_trace_send_to_editor: Option<bool>,
    pub m_additional_warmup_collections: Option<Vec<PPtr>>,
    pub m_lights_use_linear_intensity: Option<bool>,
    pub m_warmup_async: Option<bool>,
    pub m_lights_use_cct: Option<bool>,
    pub m_shader_defines_per_shader_compiler: Option<Vec<PlatformShaderDefines>>,
    pub m_enable_cache_miss_tracing: Option<bool>,
    pub m_lights_use_color_temperature: Option<bool>,
    pub m_warmup_progressively_limit: Option<i32>,
    pub m_cache_miss_collection_path: Option<String>,
    pub m_log_when_shader_is_compiled: Option<bool>,
    pub m_allow_enlighten_support_for_upgraded_project: Option<bool>,
    pub m_default_rendering_layer_mask: Option<u32>,
    pub m_current_render_pipeline_global_settings: Option<PPtr>,
    pub m_camera_relative_light_culling: Option<bool>,
    pub m_srp_default_settings: Option<Vec<(String, PPtr)>>,
    pub m_light_probe_outside_hull_strategy: Option<i32>,
    pub m_camera_relative_shadow_culling: Option<bool>,
}

impl crate::Parse for RenderManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_always_included_shaders = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_deferred = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(BuiltinShaderSettings::parse(r, ver)?)
        } else {
            None
        };
        let m_legacy_deferred = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(BuiltinShaderSettings::parse(r, ver)?)
        } else {
            None
        };
        let m_deferred_reflections = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(BuiltinShaderSettings::parse(r, ver)?)
        } else {
            None
        };
        let m_screen_space_shadows = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(BuiltinShaderSettings::parse(r, ver)?)
        } else {
            None
        };
        let m_preloaded_shaders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_depth_normals = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(BuiltinShaderSettings::parse(r, ver)?)
        } else {
            None
        };
        let m_motion_vectors = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(BuiltinShaderSettings::parse(r, ver)?)
        } else {
            None
        };
        let m_shader_settings = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = PlatformShaderSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_light_halo = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(BuiltinShaderSettings::parse(r, ver)?)
        } else {
            None
        };
        let m_lens_flare = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(BuiltinShaderSettings::parse(r, ver)?)
        } else {
            None
        };
        let m_video_shaders_include_mode = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_shader_settings_tier1 = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = PlatformShaderSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sprites_default_material = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_preload_shaders_batch_time_limit = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_shader_settings_tier2 = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = PlatformShaderSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_tier_settings_tier1 = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = TierGraphicsSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_transparency_sort_mode = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_custom_render_pipeline = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_graphics_state_collection = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_shader_settings_tier3 = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = PlatformShaderSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_tier_settings_tier2 = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = TierGraphicsSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_transparency_sort_axis = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_collection_startup_action = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_tier_settings_tier3 = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = TierGraphicsSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_trace_save_path = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_trace_send_to_editor = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_additional_warmup_collections = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_lights_use_linear_intensity = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_warmup_async = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_lights_use_cct = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_shader_defines_per_shader_compiler = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PlatformShaderDefines::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_cache_miss_tracing = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_lights_use_color_temperature = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_warmup_progressively_limit = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_cache_miss_collection_path = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_log_when_shader_is_compiled = if *ver >= crate::UnityVersion::new(2018, 4, 6) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_allow_enlighten_support_for_upgraded_project = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_default_rendering_layer_mask = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_current_render_pipeline_global_settings = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_camera_relative_light_culling = if *ver >= crate::UnityVersion::new(2020, 3, 45) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_srp_default_settings = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_light_probe_outside_hull_strategy = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_camera_relative_shadow_culling = if *ver >= crate::UnityVersion::new(2020, 3, 45) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(RenderManager {
            parent,
            m_always_included_shaders,
            m_deferred,
            m_legacy_deferred,
            m_deferred_reflections,
            m_screen_space_shadows,
            m_preloaded_shaders,
            m_depth_normals,
            m_motion_vectors,
            m_shader_settings,
            m_light_halo,
            m_lens_flare,
            m_video_shaders_include_mode,
            m_shader_settings_tier1,
            m_sprites_default_material,
            m_preload_shaders_batch_time_limit,
            m_shader_settings_tier2,
            m_tier_settings_tier1,
            m_transparency_sort_mode,
            m_custom_render_pipeline,
            m_graphics_state_collection,
            m_shader_settings_tier3,
            m_tier_settings_tier2,
            m_transparency_sort_axis,
            m_collection_startup_action,
            m_tier_settings_tier3,
            m_trace_save_path,
            m_trace_send_to_editor,
            m_additional_warmup_collections,
            m_lights_use_linear_intensity,
            m_warmup_async,
            m_lights_use_cct,
            m_shader_defines_per_shader_compiler,
            m_enable_cache_miss_tracing,
            m_lights_use_color_temperature,
            m_warmup_progressively_limit,
            m_cache_miss_collection_path,
            m_log_when_shader_is_compiled,
            m_allow_enlighten_support_for_upgraded_project,
            m_default_rendering_layer_mask,
            m_current_render_pipeline_global_settings,
            m_camera_relative_light_culling,
            m_srp_default_settings,
            m_light_probe_outside_hull_strategy,
            m_camera_relative_shadow_culling,
        })
    }
}
