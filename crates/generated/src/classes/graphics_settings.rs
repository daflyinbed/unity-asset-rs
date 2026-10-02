//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::builtin_shader_settings::BuiltinShaderSettings;
use crate::subclasses::platform_shader_defines::PlatformShaderDefines;
use crate::subclasses::tier_graphics_settings::TierGraphicsSettings;
use crate::subclasses::tier_graphics_settings_editor::TierGraphicsSettingsEditor;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：GraphicsSettingsMAlwaysIncludedShadersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GraphicsSettingsMAlwaysIncludedShadersVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for GraphicsSettingsMAlwaysIncludedShadersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for GraphicsSettingsMAlwaysIncludedShadersVariant {
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

/// 版本枚举：GraphicsSettingsMPreloadedShadersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GraphicsSettingsMPreloadedShadersVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for GraphicsSettingsMPreloadedShadersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for GraphicsSettingsMPreloadedShadersVariant {
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

/// 版本枚举：GraphicsSettingsMLightsUseColorTemperatureVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GraphicsSettingsMLightsUseColorTemperatureVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for GraphicsSettingsMLightsUseColorTemperatureVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2020_2(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for GraphicsSettingsMLightsUseColorTemperatureVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：GraphicsSettingsMLogWhenShaderIsCompiledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GraphicsSettingsMLogWhenShaderIsCompiledVariant {
    Default(bool),
    V2021_2(bool),
}

impl crate::Parse for GraphicsSettingsMLogWhenShaderIsCompiledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_2(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for GraphicsSettingsMLogWhenShaderIsCompiledVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：GraphicsSettingsMCameraRelativeLightCullingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GraphicsSettingsMCameraRelativeLightCullingVariant {
    Default(bool),
    V6000_1(bool),
}

impl crate::Parse for GraphicsSettingsMCameraRelativeLightCullingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for GraphicsSettingsMCameraRelativeLightCullingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：GraphicsSettingsMCameraRelativeShadowCullingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GraphicsSettingsMCameraRelativeShadowCullingVariant {
    Default(bool),
    V6000_1(bool),
}

impl crate::Parse for GraphicsSettingsMCameraRelativeShadowCullingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for GraphicsSettingsMCameraRelativeShadowCullingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GraphicsSettings {
    pub m_always_included_shaders: Option<GraphicsSettingsMAlwaysIncludedShadersVariant>,
    pub m_deferred: Option<BuiltinShaderSettings>,
    pub m_legacy_deferred: Option<BuiltinShaderSettings>,
    pub m_deferred_reflections: Option<BuiltinShaderSettings>,
    pub m_screen_space_shadows: Option<BuiltinShaderSettings>,
    pub m_preloaded_shaders: Option<GraphicsSettingsMPreloadedShadersVariant>,
    pub m_depth_normals: Option<BuiltinShaderSettings>,
    pub m_motion_vectors: Option<BuiltinShaderSettings>,
    pub m_settings: Option<TierGraphicsSettingsEditor>,
    pub m_light_halo: Option<BuiltinShaderSettings>,
    pub m_lens_flare: Option<BuiltinShaderSettings>,
    pub m_video_shaders_include_mode: Option<i32>,
    pub m_sprites_default_material: Option<PPtr>,
    pub m_preload_shaders_batch_time_limit: Option<i32>,
    pub m_shader_settings_tier1: Option<TierGraphicsSettingsEditor>,
    pub m_tier_settings_tier1: Option<TierGraphicsSettings>,
    pub m_custom_render_pipeline: Option<PPtr>,
    pub m_graphics_state_collection: Option<PPtr>,
    pub m_shader_settings_tier2: Option<TierGraphicsSettingsEditor>,
    pub m_tier_settings_tier2: Option<TierGraphicsSettings>,
    pub m_transparency_sort_mode: Option<i32>,
    pub m_collection_startup_action: Option<i32>,
    pub m_shader_settings_tier3: Option<TierGraphicsSettingsEditor>,
    pub m_tier_settings_tier3: Option<TierGraphicsSettings>,
    pub m_transparency_sort_axis: Option<Vector3f>,
    pub m_trace_save_path: Option<String>,
    pub m_trace_send_to_editor: Option<bool>,
    pub m_additional_warmup_collections: Option<Vec<PPtr>>,
    pub m_warmup_async: Option<bool>,
    pub m_lights_use_linear_intensity: Option<bool>,
    pub m_shader_defines_per_shader_compiler: Option<Vec<PlatformShaderDefines>>,
    pub m_enable_cache_miss_tracing: Option<bool>,
    pub m_lights_use_color_temperature: Option<GraphicsSettingsMLightsUseColorTemperatureVariant>,
    pub m_warmup_progressively_limit: Option<i32>,
    pub m_cache_miss_collection_path: Option<String>,
    pub m_log_when_shader_is_compiled: Option<GraphicsSettingsMLogWhenShaderIsCompiledVariant>,
    pub m_allow_enlighten_support_for_upgraded_project: Option<bool>,
    pub m_default_rendering_layer_mask: Option<u32>,
    pub m_current_render_pipeline_global_settings: Option<PPtr>,
    pub m_camera_relative_light_culling: Option<GraphicsSettingsMCameraRelativeLightCullingVariant>,
    pub m_srp_default_settings: Option<Vec<(String, PPtr)>>,
    pub m_light_probe_outside_hull_strategy: Option<i32>,
    pub m_camera_relative_shadow_culling:
        Option<GraphicsSettingsMCameraRelativeShadowCullingVariant>,
}

impl crate::Parse for GraphicsSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_always_included_shaders = None;
        let mut m_deferred = None;
        let mut m_legacy_deferred = None;
        let mut m_deferred_reflections = None;
        let mut m_screen_space_shadows = None;
        let mut m_preloaded_shaders = None;
        let mut m_depth_normals = None;
        let mut m_motion_vectors = None;
        let mut m_settings = None;
        let mut m_light_halo = None;
        let mut m_lens_flare = None;
        let mut m_video_shaders_include_mode = None;
        let mut m_sprites_default_material = None;
        let mut m_preload_shaders_batch_time_limit = None;
        let mut m_shader_settings_tier1 = None;
        let mut m_tier_settings_tier1 = None;
        let mut m_custom_render_pipeline = None;
        let mut m_graphics_state_collection = None;
        let mut m_shader_settings_tier2 = None;
        let mut m_tier_settings_tier2 = None;
        let mut m_transparency_sort_mode = None;
        let mut m_collection_startup_action = None;
        let mut m_shader_settings_tier3 = None;
        let mut m_tier_settings_tier3 = None;
        let mut m_transparency_sort_axis = None;
        let mut m_trace_save_path = None;
        let mut m_trace_send_to_editor = None;
        let mut m_additional_warmup_collections = None;
        let mut m_warmup_async = None;
        let mut m_lights_use_linear_intensity = None;
        let mut m_shader_defines_per_shader_compiler = None;
        let mut m_enable_cache_miss_tracing = None;
        let mut m_lights_use_color_temperature = None;
        let mut m_warmup_progressively_limit = None;
        let mut m_cache_miss_collection_path = None;
        let mut m_log_when_shader_is_compiled = None;
        let mut m_allow_enlighten_support_for_upgraded_project = None;
        let mut m_default_rendering_layer_mask = None;
        let mut m_current_render_pipeline_global_settings = None;
        let mut m_camera_relative_light_culling = None;
        let mut m_srp_default_settings = None;
        let mut m_light_probe_outside_hull_strategy = None;
        let mut m_camera_relative_shadow_culling = None;
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_preload_shaders_batch_time_limit = Some(rdr.read_i32()?);
            m_graphics_state_collection = Some(crate::PPtr::parse(rdr, ver)?);
            m_collection_startup_action = Some(rdr.read_i32()?);
            m_trace_save_path = Some(rdr.read_aligned_string()?);
            m_trace_send_to_editor = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_additional_warmup_collections = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_warmup_async = Some(rdr.read_bool()?);
            m_enable_cache_miss_tracing = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_warmup_progressively_limit = Some(rdr.read_i32()?);
            m_cache_miss_collection_path = Some(rdr.read_aligned_string()?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_current_render_pipeline_global_settings = Some(crate::PPtr::parse(rdr, ver)?);
            m_light_probe_outside_hull_strategy = Some(rdr.read_i32()?);
            m_camera_relative_light_culling = Some(
                GraphicsSettingsMCameraRelativeLightCullingVariant::parse(rdr, ver)?,
            );
            m_camera_relative_shadow_culling = Some(
                GraphicsSettingsMCameraRelativeShadowCullingVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_preload_shaders_batch_time_limit = Some(rdr.read_i32()?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_current_render_pipeline_global_settings = Some(crate::PPtr::parse(rdr, ver)?);
            m_light_probe_outside_hull_strategy = Some(rdr.read_i32()?);
            m_camera_relative_light_culling = Some(
                GraphicsSettingsMCameraRelativeLightCullingVariant::parse(rdr, ver)?,
            );
            m_camera_relative_shadow_culling = Some(
                GraphicsSettingsMCameraRelativeShadowCullingVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_preload_shaders_batch_time_limit = Some(rdr.read_i32()?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_current_render_pipeline_global_settings = Some(crate::PPtr::parse(rdr, ver)?);
            m_light_probe_outside_hull_strategy = Some(rdr.read_i32()?);
            m_camera_relative_light_culling = Some(
                GraphicsSettingsMCameraRelativeLightCullingVariant::parse(rdr, ver)?,
            );
            m_camera_relative_shadow_culling = Some(
                GraphicsSettingsMCameraRelativeShadowCullingVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 2, 2) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_preload_shaders_batch_time_limit = Some(rdr.read_i32()?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_srp_default_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_light_probe_outside_hull_strategy = Some(rdr.read_i32()?);
            m_camera_relative_light_culling = Some(
                GraphicsSettingsMCameraRelativeLightCullingVariant::parse(rdr, ver)?,
            );
            m_camera_relative_shadow_culling = Some(
                GraphicsSettingsMCameraRelativeShadowCullingVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_preload_shaders_batch_time_limit = Some(rdr.read_i32()?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_srp_default_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_light_probe_outside_hull_strategy = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_preload_shaders_batch_time_limit = Some(rdr.read_i32()?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_srp_default_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 3, 18) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_preload_shaders_batch_time_limit = Some(rdr.read_i32()?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_srp_default_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_camera_relative_light_culling = Some(
                GraphicsSettingsMCameraRelativeLightCullingVariant::parse(rdr, ver)?,
            );
            m_camera_relative_shadow_culling = Some(
                GraphicsSettingsMCameraRelativeShadowCullingVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_preload_shaders_batch_time_limit = Some(rdr.read_i32()?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_srp_default_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 3, 45) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_camera_relative_light_culling = Some(
                GraphicsSettingsMCameraRelativeLightCullingVariant::parse(rdr, ver)?,
            );
            m_camera_relative_shadow_culling = Some(
                GraphicsSettingsMCameraRelativeShadowCullingVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_default_rendering_layer_mask = Some(rdr.read_u32()?);
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_video_shaders_include_mode = Some(rdr.read_i32()?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
            m_allow_enlighten_support_for_upgraded_project = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 7) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 4, 6) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
            m_log_when_shader_is_compiled = Some(
                GraphicsSettingsMLogWhenShaderIsCompiledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_shader_defines_per_shader_compiler = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| PlatformShaderDefines::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            m_transparency_sort_mode = Some(rdr.read_i32()?);
            m_transparency_sort_axis = Some(Vector3f::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_lights_use_linear_intensity = Some(rdr.read_bool()?);
            m_lights_use_color_temperature = Some(
                GraphicsSettingsMLightsUseColorTemperatureVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_tier_settings_tier1 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier2 = Some(TierGraphicsSettings::parse(rdr, ver)?);
            m_tier_settings_tier3 = Some(TierGraphicsSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_screen_space_shadows = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_depth_normals = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_motion_vectors = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_light_halo = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_lens_flare = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_sprites_default_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_shader_settings_tier1 = Some(TierGraphicsSettingsEditor::parse(rdr, ver)?);
            m_shader_settings_tier2 = Some(TierGraphicsSettingsEditor::parse(rdr, ver)?);
            m_shader_settings_tier3 = Some(TierGraphicsSettingsEditor::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
            m_settings = Some(TierGraphicsSettingsEditor::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_deferred_reflections = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_legacy_deferred = Some(BuiltinShaderSettings::parse(rdr, ver)?);
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
            m_preloaded_shaders = Some(GraphicsSettingsMPreloadedShadersVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_always_included_shaders = Some(GraphicsSettingsMAlwaysIncludedShadersVariant::parse(
                rdr, ver,
            )?);
        } else {
        }

        Ok(GraphicsSettings {
            m_always_included_shaders,
            m_deferred,
            m_legacy_deferred,
            m_deferred_reflections,
            m_screen_space_shadows,
            m_preloaded_shaders,
            m_depth_normals,
            m_motion_vectors,
            m_settings,
            m_light_halo,
            m_lens_flare,
            m_video_shaders_include_mode,
            m_sprites_default_material,
            m_preload_shaders_batch_time_limit,
            m_shader_settings_tier1,
            m_tier_settings_tier1,
            m_custom_render_pipeline,
            m_graphics_state_collection,
            m_shader_settings_tier2,
            m_tier_settings_tier2,
            m_transparency_sort_mode,
            m_collection_startup_action,
            m_shader_settings_tier3,
            m_tier_settings_tier3,
            m_transparency_sort_axis,
            m_trace_save_path,
            m_trace_send_to_editor,
            m_additional_warmup_collections,
            m_warmup_async,
            m_lights_use_linear_intensity,
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

impl crate::ToValue for GraphicsSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_AlwaysIncludedShaders", &self.m_always_included_shaders),
            crate::opt_field("m_Deferred", &self.m_deferred),
            crate::opt_field("m_LegacyDeferred", &self.m_legacy_deferred),
            crate::opt_field("m_DeferredReflections", &self.m_deferred_reflections),
            crate::opt_field("m_ScreenSpaceShadows", &self.m_screen_space_shadows),
            crate::opt_field("m_PreloadedShaders", &self.m_preloaded_shaders),
            crate::opt_field("m_DepthNormals", &self.m_depth_normals),
            crate::opt_field("m_MotionVectors", &self.m_motion_vectors),
            crate::opt_field("m_Settings", &self.m_settings),
            crate::opt_field("m_LightHalo", &self.m_light_halo),
            crate::opt_field("m_LensFlare", &self.m_lens_flare),
            crate::opt_field(
                "m_VideoShadersIncludeMode",
                &self.m_video_shaders_include_mode,
            ),
            crate::opt_field("m_SpritesDefaultMaterial", &self.m_sprites_default_material),
            crate::opt_field(
                "m_PreloadShadersBatchTimeLimit",
                &self.m_preload_shaders_batch_time_limit,
            ),
            crate::opt_field("m_ShaderSettings_Tier1", &self.m_shader_settings_tier1),
            crate::opt_field("m_TierSettings_Tier1", &self.m_tier_settings_tier1),
            crate::opt_field("m_CustomRenderPipeline", &self.m_custom_render_pipeline),
            crate::opt_field(
                "m_GraphicsStateCollection",
                &self.m_graphics_state_collection,
            ),
            crate::opt_field("m_ShaderSettings_Tier2", &self.m_shader_settings_tier2),
            crate::opt_field("m_TierSettings_Tier2", &self.m_tier_settings_tier2),
            crate::opt_field("m_TransparencySortMode", &self.m_transparency_sort_mode),
            crate::opt_field(
                "m_CollectionStartupAction",
                &self.m_collection_startup_action,
            ),
            crate::opt_field("m_ShaderSettings_Tier3", &self.m_shader_settings_tier3),
            crate::opt_field("m_TierSettings_Tier3", &self.m_tier_settings_tier3),
            crate::opt_field("m_TransparencySortAxis", &self.m_transparency_sort_axis),
            crate::opt_field("m_TraceSavePath", &self.m_trace_save_path),
            crate::opt_field("m_TraceSendToEditor", &self.m_trace_send_to_editor),
            crate::opt_field(
                "m_AdditionalWarmupCollections",
                &self.m_additional_warmup_collections,
            ),
            crate::opt_field("m_WarmupAsync", &self.m_warmup_async),
            crate::opt_field(
                "m_LightsUseLinearIntensity",
                &self.m_lights_use_linear_intensity,
            ),
            crate::opt_field(
                "m_ShaderDefinesPerShaderCompiler",
                &self.m_shader_defines_per_shader_compiler,
            ),
            crate::opt_field(
                "m_EnableCacheMissTracing",
                &self.m_enable_cache_miss_tracing,
            ),
            crate::opt_field(
                "m_LightsUseColorTemperature",
                &self.m_lights_use_color_temperature,
            ),
            crate::opt_field(
                "m_WarmupProgressivelyLimit",
                &self.m_warmup_progressively_limit,
            ),
            crate::opt_field(
                "m_CacheMissCollectionPath",
                &self.m_cache_miss_collection_path,
            ),
            crate::opt_field(
                "m_LogWhenShaderIsCompiled",
                &self.m_log_when_shader_is_compiled,
            ),
            crate::opt_field(
                "m_AllowEnlightenSupportForUpgradedProject",
                &self.m_allow_enlighten_support_for_upgraded_project,
            ),
            crate::opt_field(
                "m_DefaultRenderingLayerMask",
                &self.m_default_rendering_layer_mask,
            ),
            crate::opt_field(
                "m_CurrentRenderPipelineGlobalSettings",
                &self.m_current_render_pipeline_global_settings,
            ),
            crate::opt_field(
                "m_CameraRelativeLightCulling",
                &self.m_camera_relative_light_culling,
            ),
            crate::opt_field("m_SRPDefaultSettings", &self.m_srp_default_settings),
            crate::opt_field(
                "m_LightProbeOutsideHullStrategy",
                &self.m_light_probe_outside_hull_strategy,
            ),
            crate::opt_field(
                "m_CameraRelativeShadowCulling",
                &self.m_camera_relative_shadow_culling,
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
                "m_AlwaysIncludedShaders" => {
                    (self.m_always_included_shaders).inspect_value(path, offset, limit)
                }
                "m_Deferred" => (self.m_deferred).inspect_value(path, offset, limit),
                "m_LegacyDeferred" => (self.m_legacy_deferred).inspect_value(path, offset, limit),
                "m_DeferredReflections" => {
                    (self.m_deferred_reflections).inspect_value(path, offset, limit)
                }
                "m_ScreenSpaceShadows" => {
                    (self.m_screen_space_shadows).inspect_value(path, offset, limit)
                }
                "m_PreloadedShaders" => {
                    (self.m_preloaded_shaders).inspect_value(path, offset, limit)
                }
                "m_DepthNormals" => (self.m_depth_normals).inspect_value(path, offset, limit),
                "m_MotionVectors" => (self.m_motion_vectors).inspect_value(path, offset, limit),
                "m_Settings" => (self.m_settings).inspect_value(path, offset, limit),
                "m_LightHalo" => (self.m_light_halo).inspect_value(path, offset, limit),
                "m_LensFlare" => (self.m_lens_flare).inspect_value(path, offset, limit),
                "m_VideoShadersIncludeMode" => {
                    (self.m_video_shaders_include_mode).inspect_value(path, offset, limit)
                }
                "m_SpritesDefaultMaterial" => {
                    (self.m_sprites_default_material).inspect_value(path, offset, limit)
                }
                "m_PreloadShadersBatchTimeLimit" => {
                    (self.m_preload_shaders_batch_time_limit).inspect_value(path, offset, limit)
                }
                "m_ShaderSettings_Tier1" => {
                    (self.m_shader_settings_tier1).inspect_value(path, offset, limit)
                }
                "m_TierSettings_Tier1" => {
                    (self.m_tier_settings_tier1).inspect_value(path, offset, limit)
                }
                "m_CustomRenderPipeline" => {
                    (self.m_custom_render_pipeline).inspect_value(path, offset, limit)
                }
                "m_GraphicsStateCollection" => {
                    (self.m_graphics_state_collection).inspect_value(path, offset, limit)
                }
                "m_ShaderSettings_Tier2" => {
                    (self.m_shader_settings_tier2).inspect_value(path, offset, limit)
                }
                "m_TierSettings_Tier2" => {
                    (self.m_tier_settings_tier2).inspect_value(path, offset, limit)
                }
                "m_TransparencySortMode" => {
                    (self.m_transparency_sort_mode).inspect_value(path, offset, limit)
                }
                "m_CollectionStartupAction" => {
                    (self.m_collection_startup_action).inspect_value(path, offset, limit)
                }
                "m_ShaderSettings_Tier3" => {
                    (self.m_shader_settings_tier3).inspect_value(path, offset, limit)
                }
                "m_TierSettings_Tier3" => {
                    (self.m_tier_settings_tier3).inspect_value(path, offset, limit)
                }
                "m_TransparencySortAxis" => {
                    (self.m_transparency_sort_axis).inspect_value(path, offset, limit)
                }
                "m_TraceSavePath" => (self.m_trace_save_path).inspect_value(path, offset, limit),
                "m_TraceSendToEditor" => {
                    (self.m_trace_send_to_editor).inspect_value(path, offset, limit)
                }
                "m_AdditionalWarmupCollections" => {
                    (self.m_additional_warmup_collections).inspect_value(path, offset, limit)
                }
                "m_WarmupAsync" => (self.m_warmup_async).inspect_value(path, offset, limit),
                "m_LightsUseLinearIntensity" => {
                    (self.m_lights_use_linear_intensity).inspect_value(path, offset, limit)
                }
                "m_ShaderDefinesPerShaderCompiler" => {
                    (self.m_shader_defines_per_shader_compiler).inspect_value(path, offset, limit)
                }
                "m_EnableCacheMissTracing" => {
                    (self.m_enable_cache_miss_tracing).inspect_value(path, offset, limit)
                }
                "m_LightsUseColorTemperature" => {
                    (self.m_lights_use_color_temperature).inspect_value(path, offset, limit)
                }
                "m_WarmupProgressivelyLimit" => {
                    (self.m_warmup_progressively_limit).inspect_value(path, offset, limit)
                }
                "m_CacheMissCollectionPath" => {
                    (self.m_cache_miss_collection_path).inspect_value(path, offset, limit)
                }
                "m_LogWhenShaderIsCompiled" => {
                    (self.m_log_when_shader_is_compiled).inspect_value(path, offset, limit)
                }
                "m_AllowEnlightenSupportForUpgradedProject" => (self
                    .m_allow_enlighten_support_for_upgraded_project)
                    .inspect_value(path, offset, limit),
                "m_DefaultRenderingLayerMask" => {
                    (self.m_default_rendering_layer_mask).inspect_value(path, offset, limit)
                }
                "m_CurrentRenderPipelineGlobalSettings" => (self
                    .m_current_render_pipeline_global_settings)
                    .inspect_value(path, offset, limit),
                "m_CameraRelativeLightCulling" => {
                    (self.m_camera_relative_light_culling).inspect_value(path, offset, limit)
                }
                "m_SRPDefaultSettings" => {
                    (self.m_srp_default_settings).inspect_value(path, offset, limit)
                }
                "m_LightProbeOutsideHullStrategy" => {
                    (self.m_light_probe_outside_hull_strategy).inspect_value(path, offset, limit)
                }
                "m_CameraRelativeShadowCulling" => {
                    (self.m_camera_relative_shadow_culling).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_always_included_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_AlwaysIncludedShaders".to_string(), v)),
            ((self.m_deferred).inspect_value(path, offset, limit))
                .map(|v| ("m_Deferred".to_string(), v)),
            ((self.m_legacy_deferred).inspect_value(path, offset, limit))
                .map(|v| ("m_LegacyDeferred".to_string(), v)),
            ((self.m_deferred_reflections).inspect_value(path, offset, limit))
                .map(|v| ("m_DeferredReflections".to_string(), v)),
            ((self.m_screen_space_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_ScreenSpaceShadows".to_string(), v)),
            ((self.m_preloaded_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_PreloadedShaders".to_string(), v)),
            ((self.m_depth_normals).inspect_value(path, offset, limit))
                .map(|v| ("m_DepthNormals".to_string(), v)),
            ((self.m_motion_vectors).inspect_value(path, offset, limit))
                .map(|v| ("m_MotionVectors".to_string(), v)),
            ((self.m_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_Settings".to_string(), v)),
            ((self.m_light_halo).inspect_value(path, offset, limit))
                .map(|v| ("m_LightHalo".to_string(), v)),
            ((self.m_lens_flare).inspect_value(path, offset, limit))
                .map(|v| ("m_LensFlare".to_string(), v)),
            ((self.m_video_shaders_include_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_VideoShadersIncludeMode".to_string(), v)),
            ((self.m_sprites_default_material).inspect_value(path, offset, limit))
                .map(|v| ("m_SpritesDefaultMaterial".to_string(), v)),
            ((self.m_preload_shaders_batch_time_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_PreloadShadersBatchTimeLimit".to_string(), v)),
            ((self.m_shader_settings_tier1).inspect_value(path, offset, limit))
                .map(|v| ("m_ShaderSettings_Tier1".to_string(), v)),
            ((self.m_tier_settings_tier1).inspect_value(path, offset, limit))
                .map(|v| ("m_TierSettings_Tier1".to_string(), v)),
            ((self.m_custom_render_pipeline).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomRenderPipeline".to_string(), v)),
            ((self.m_graphics_state_collection).inspect_value(path, offset, limit))
                .map(|v| ("m_GraphicsStateCollection".to_string(), v)),
            ((self.m_shader_settings_tier2).inspect_value(path, offset, limit))
                .map(|v| ("m_ShaderSettings_Tier2".to_string(), v)),
            ((self.m_tier_settings_tier2).inspect_value(path, offset, limit))
                .map(|v| ("m_TierSettings_Tier2".to_string(), v)),
            ((self.m_transparency_sort_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_TransparencySortMode".to_string(), v)),
            ((self.m_collection_startup_action).inspect_value(path, offset, limit))
                .map(|v| ("m_CollectionStartupAction".to_string(), v)),
            ((self.m_shader_settings_tier3).inspect_value(path, offset, limit))
                .map(|v| ("m_ShaderSettings_Tier3".to_string(), v)),
            ((self.m_tier_settings_tier3).inspect_value(path, offset, limit))
                .map(|v| ("m_TierSettings_Tier3".to_string(), v)),
            ((self.m_transparency_sort_axis).inspect_value(path, offset, limit))
                .map(|v| ("m_TransparencySortAxis".to_string(), v)),
            ((self.m_trace_save_path).inspect_value(path, offset, limit))
                .map(|v| ("m_TraceSavePath".to_string(), v)),
            ((self.m_trace_send_to_editor).inspect_value(path, offset, limit))
                .map(|v| ("m_TraceSendToEditor".to_string(), v)),
            ((self.m_additional_warmup_collections).inspect_value(path, offset, limit))
                .map(|v| ("m_AdditionalWarmupCollections".to_string(), v)),
            ((self.m_warmup_async).inspect_value(path, offset, limit))
                .map(|v| ("m_WarmupAsync".to_string(), v)),
            ((self.m_lights_use_linear_intensity).inspect_value(path, offset, limit))
                .map(|v| ("m_LightsUseLinearIntensity".to_string(), v)),
            ((self.m_shader_defines_per_shader_compiler).inspect_value(path, offset, limit))
                .map(|v| ("m_ShaderDefinesPerShaderCompiler".to_string(), v)),
            ((self.m_enable_cache_miss_tracing).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableCacheMissTracing".to_string(), v)),
            ((self.m_lights_use_color_temperature).inspect_value(path, offset, limit))
                .map(|v| ("m_LightsUseColorTemperature".to_string(), v)),
            ((self.m_warmup_progressively_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_WarmupProgressivelyLimit".to_string(), v)),
            ((self.m_cache_miss_collection_path).inspect_value(path, offset, limit))
                .map(|v| ("m_CacheMissCollectionPath".to_string(), v)),
            ((self.m_log_when_shader_is_compiled).inspect_value(path, offset, limit))
                .map(|v| ("m_LogWhenShaderIsCompiled".to_string(), v)),
            ((self.m_allow_enlighten_support_for_upgraded_project)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_AllowEnlightenSupportForUpgradedProject".to_string(), v)),
            ((self.m_default_rendering_layer_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultRenderingLayerMask".to_string(), v)),
            ((self.m_current_render_pipeline_global_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_CurrentRenderPipelineGlobalSettings".to_string(), v)),
            ((self.m_camera_relative_light_culling).inspect_value(path, offset, limit))
                .map(|v| ("m_CameraRelativeLightCulling".to_string(), v)),
            ((self.m_srp_default_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_SRPDefaultSettings".to_string(), v)),
            ((self.m_light_probe_outside_hull_strategy).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeOutsideHullStrategy".to_string(), v)),
            ((self.m_camera_relative_shadow_culling).inspect_value(path, offset, limit))
                .map(|v| ("m_CameraRelativeShadowCulling".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
