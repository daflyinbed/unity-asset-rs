//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::collab_editor_settings::CollabEditorSettings;


/// 版本枚举：EditorSettingsMExternalVersionControlSupportVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorSettingsMExternalVersionControlSupportVariant {
    Default(i32),
    V4_0(String),
}

impl crate::Parse for EditorSettingsMExternalVersionControlSupportVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorSettingsMSerializationModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorSettingsMSerializationModeVariant {
    Default(i32),
    V5_5(i32),
}

impl crate::Parse for EditorSettingsMSerializationModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorSettingsMEnableTextureStreamingInPlayModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorSettingsMEnableTextureStreamingInPlayModeVariant {
    Default(bool),
    V2019_1(bool),
}

impl crate::Parse for EditorSettingsMEnableTextureStreamingInPlayModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorSettingsMAsyncShaderCompilationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorSettingsMAsyncShaderCompilationVariant {
    Default(bool),
    V2020_1(bool),
}

impl crate::Parse for EditorSettingsMAsyncShaderCompilationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorSettingsMCacheServerEnableUploadVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorSettingsMCacheServerEnableUploadVariant {
    Default(bool),
    V2020_1(bool),
}

impl crate::Parse for EditorSettingsMCacheServerEnableUploadVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorSettingsMCacheServerEnableTlsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorSettingsMCacheServerEnableTlsVariant {
    Default(bool),
    V6000_5(bool),
}

impl crate::Parse for EditorSettingsMCacheServerEnableTlsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct EditorSettings {
    pub m_external_version_control_support: Option<EditorSettingsMExternalVersionControlSupportVariant>,
    pub m_serialization_mode: Option<EditorSettingsMSerializationModeVariant>,
    pub m_web_security_emulation_enabled: Option<i32>,
    pub m_line_endings_for_new_scripts: Option<i32>,
    pub m_web_security_emulation_host_url: Option<String>,
    pub m_default_behavior_mode: Option<i32>,
    pub m_sprite_packer_mode: Option<i32>,
    pub m_prefab_regular_environment: Option<PPtr>,
    pub m_sprite_packer_padding_power: Option<i32>,
    pub m_prefab_ui_environment: Option<PPtr>,
    pub m_project_generation_included_extensions: Option<String>,
    pub m_etc_texture_compressor_behavior: Option<i32>,
    pub m_project_generation_root_namespace: Option<String>,
    pub m_etc_texture_fast_compressor: Option<i32>,
    pub m_sprite_packer_cache_size: Option<i32>,
    pub m_user_generated_project_suffix: Option<String>,
    pub m_etc_texture_normal_compressor: Option<i32>,
    pub m_bc7_texture_compressor: Option<i32>,
    pub m_collab_editor_settings: Option<CollabEditorSettings>,
    pub m_etc_texture_best_compressor: Option<i32>,
    pub m_enable_texture_streaming_in_edit_mode: Option<bool>,
    pub m_enable_texture_streaming_in_play_mode: Option<EditorSettingsMEnableTextureStreamingInPlayModeVariant>,
    pub m_async_shader_compilation: Option<EditorSettingsMAsyncShaderCompilationVariant>,
    pub m_caching_shader_preprocessor: Option<bool>,
    pub m_enter_play_mode_options_enabled: Option<bool>,
    pub m_prefab_mode_allow_auto_save: Option<bool>,
    pub m_enable_editor_async_cpu_texture_loading: Option<bool>,
    pub m_show_lightmap_resolution_overlay: Option<bool>,
    pub m_enter_play_mode_options: Option<i32>,
    pub m_game_object_naming_digits: Option<i32>,
    pub m_block_shaders: Option<bool>,
    pub m_game_object_naming_scheme: Option<i32>,
    pub m_unlock_block_shaders: Option<bool>,
    pub m_use_legacy_probe_sample_count: Option<bool>,
    pub m_asset_naming_uses_space: Option<bool>,
    pub m_asset_pipeline_mode: Option<i32>,
    pub m_cache_server_mode: Option<i32>,
    pub m_serialize_inline_mappings_on_one_line: Option<bool>,
    pub m_cache_server_endpoint: Option<String>,
    pub m_disable_cookies_in_lightmapper: Option<bool>,
    pub m_cache_server_namespace_prefix: Option<String>,
    pub m_inspector_use_imgui_default_inspector: Option<bool>,
    pub m_cache_server_enable_download: Option<bool>,
    pub m_cache_server_enable_upload: Option<EditorSettingsMCacheServerEnableUploadVariant>,
    pub m_cache_server_validation_mode: Option<i32>,
    pub m_refresh_import_mode: Option<i32>,
    pub m_shadowmask_stitching: Option<bool>,
    pub m_cache_server_enable_auth: Option<bool>,
    pub m_cache_server_enable_tls: Option<EditorSettingsMCacheServerEnableTlsVariant>,
    pub m_cache_server_download_batch_size: Option<i32>,
    pub m_cache_server_import_result_caching_enabled: Option<bool>,
    pub m_enable_enlighten_baked_gi: Option<bool>,
    pub m_referenced_clips_exact_naming: Option<bool>,
    pub m_force_asset_unload_and_gc_on_scene_load: Option<bool>,
    pub m_hide_build_profile_classic_platforms: Option<bool>,
}

impl crate::Parse for EditorSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_external_version_control_support = None;
        let mut m_serialization_mode = None;
        let mut m_web_security_emulation_enabled = None;
        let mut m_line_endings_for_new_scripts = None;
        let mut m_web_security_emulation_host_url = None;
        let mut m_default_behavior_mode = None;
        let mut m_sprite_packer_mode = None;
        let mut m_prefab_regular_environment = None;
        let mut m_sprite_packer_padding_power = None;
        let mut m_prefab_ui_environment = None;
        let mut m_project_generation_included_extensions = None;
        let mut m_etc_texture_compressor_behavior = None;
        let mut m_project_generation_root_namespace = None;
        let mut m_etc_texture_fast_compressor = None;
        let mut m_sprite_packer_cache_size = None;
        let mut m_user_generated_project_suffix = None;
        let mut m_etc_texture_normal_compressor = None;
        let mut m_bc7_texture_compressor = None;
        let mut m_collab_editor_settings = None;
        let mut m_etc_texture_best_compressor = None;
        let mut m_enable_texture_streaming_in_edit_mode = None;
        let mut m_enable_texture_streaming_in_play_mode = None;
        let mut m_async_shader_compilation = None;
        let mut m_caching_shader_preprocessor = None;
        let mut m_enter_play_mode_options_enabled = None;
        let mut m_prefab_mode_allow_auto_save = None;
        let mut m_enable_editor_async_cpu_texture_loading = None;
        let mut m_show_lightmap_resolution_overlay = None;
        let mut m_enter_play_mode_options = None;
        let mut m_game_object_naming_digits = None;
        let mut m_block_shaders = None;
        let mut m_game_object_naming_scheme = None;
        let mut m_unlock_block_shaders = None;
        let mut m_use_legacy_probe_sample_count = None;
        let mut m_asset_naming_uses_space = None;
        let mut m_asset_pipeline_mode = None;
        let mut m_cache_server_mode = None;
        let mut m_serialize_inline_mappings_on_one_line = None;
        let mut m_cache_server_endpoint = None;
        let mut m_disable_cookies_in_lightmapper = None;
        let mut m_cache_server_namespace_prefix = None;
        let mut m_inspector_use_imgui_default_inspector = None;
        let mut m_cache_server_enable_download = None;
        let mut m_cache_server_enable_upload = None;
        let mut m_cache_server_validation_mode = None;
        let mut m_refresh_import_mode = None;
        let mut m_shadowmask_stitching = None;
        let mut m_cache_server_enable_auth = None;
        let mut m_cache_server_enable_tls = None;
        let mut m_cache_server_download_batch_size = None;
        let mut m_cache_server_import_result_caching_enabled = None;
        let mut m_enable_enlighten_baked_gi = None;
        let mut m_referenced_clips_exact_naming = None;
        let mut m_force_asset_unload_and_gc_on_scene_load = None;
        let mut m_hide_build_profile_classic_platforms = None;
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_block_shaders = Some(rdr.read_bool()?);
            m_unlock_block_shaders = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_shadowmask_stitching = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_import_result_caching_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
            m_referenced_clips_exact_naming = Some(rdr.read_bool()?);
            m_force_asset_unload_and_gc_on_scene_load = Some(rdr.read_bool()?);
            m_hide_build_profile_classic_platforms = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_block_shaders = Some(rdr.read_bool()?);
            m_unlock_block_shaders = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_shadowmask_stitching = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_import_result_caching_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
            m_referenced_clips_exact_naming = Some(rdr.read_bool()?);
            m_force_asset_unload_and_gc_on_scene_load = Some(rdr.read_bool()?);
            m_hide_build_profile_classic_platforms = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_block_shaders = Some(rdr.read_bool()?);
            m_unlock_block_shaders = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_shadowmask_stitching = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
            m_referenced_clips_exact_naming = Some(rdr.read_bool()?);
            m_force_asset_unload_and_gc_on_scene_load = Some(rdr.read_bool()?);
            m_hide_build_profile_classic_platforms = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_shadowmask_stitching = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
            m_referenced_clips_exact_naming = Some(rdr.read_bool()?);
            m_force_asset_unload_and_gc_on_scene_load = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_shadowmask_stitching = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
            m_referenced_clips_exact_naming = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 66) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_shadowmask_stitching = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
            m_referenced_clips_exact_naming = Some(rdr.read_bool()?);
            m_force_asset_unload_and_gc_on_scene_load = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 45) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_shadowmask_stitching = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
            m_referenced_clips_exact_naming = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
            m_referenced_clips_exact_naming = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_inspector_use_imgui_default_inspector = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
            m_enable_enlighten_baked_gi = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 16) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_cache_size = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_enable_editor_async_cpu_texture_loading = Some(rdr.read_bool()?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 10) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_caching_shader_preprocessor = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
            m_cache_server_download_batch_size = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 3) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_caching_shader_preprocessor = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_bc7_texture_compressor = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_caching_shader_preprocessor = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_refresh_import_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_caching_shader_preprocessor = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 35) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_caching_shader_preprocessor = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_caching_shader_preprocessor = Some(rdr.read_bool()?);
            m_prefab_mode_allow_auto_save = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_caching_shader_preprocessor = Some(rdr.read_bool()?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_game_object_naming_digits = Some(rdr.read_i32()?);
            m_game_object_naming_scheme = Some(rdr.read_i32()?);
            m_asset_naming_uses_space = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_serialize_inline_mappings_on_one_line = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_disable_cookies_in_lightmapper = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_enable_auth = Some(rdr.read_bool()?);
            m_cache_server_enable_tls = Some(EditorSettingsMCacheServerEnableTlsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 40) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_show_lightmap_resolution_overlay = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
            m_cache_server_validation_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_enter_play_mode_options_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enter_play_mode_options = Some(rdr.read_i32()?);
            m_show_lightmap_resolution_overlay = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_use_legacy_probe_sample_count = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_server_endpoint = Some(rdr.read_aligned_string()?);
            m_cache_server_namespace_prefix = Some(rdr.read_aligned_string()?);
            m_cache_server_enable_download = Some(rdr.read_bool()?);
            m_cache_server_enable_upload = Some(EditorSettingsMCacheServerEnableUploadVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
            m_show_lightmap_resolution_overlay = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
            m_enable_texture_streaming_in_edit_mode = Some(rdr.read_bool()?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
            m_async_shader_compilation = Some(EditorSettingsMAsyncShaderCompilationVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_prefab_regular_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_prefab_ui_environment = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_user_generated_project_suffix = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
            m_enable_texture_streaming_in_play_mode = Some(EditorSettingsMEnableTextureStreamingInPlayModeVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_line_endings_for_new_scripts = Some(rdr.read_i32()?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_user_generated_project_suffix = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_etc_texture_compressor_behavior = Some(rdr.read_i32()?);
            m_etc_texture_fast_compressor = Some(rdr.read_i32()?);
            m_etc_texture_normal_compressor = Some(rdr.read_i32()?);
            m_etc_texture_best_compressor = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_user_generated_project_suffix = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_user_generated_project_suffix = Some(rdr.read_aligned_string()?);
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
            m_user_generated_project_suffix = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_web_security_emulation_enabled = Some(rdr.read_i32()?);
            m_web_security_emulation_host_url = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
            m_project_generation_included_extensions = Some(rdr.read_aligned_string()?);
            m_project_generation_root_namespace = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_web_security_emulation_enabled = Some(rdr.read_i32()?);
            m_web_security_emulation_host_url = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
            m_sprite_packer_padding_power = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_web_security_emulation_enabled = Some(rdr.read_i32()?);
            m_web_security_emulation_host_url = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_default_behavior_mode = Some(rdr.read_i32()?);
            m_sprite_packer_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_serialization_mode = Some(EditorSettingsMSerializationModeVariant::parse(rdr, ver)?);
            m_web_security_emulation_enabled = Some(rdr.read_i32()?);
            m_web_security_emulation_host_url = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
            m_external_version_control_support = Some(EditorSettingsMExternalVersionControlSupportVariant::parse(rdr, ver)?);
            m_web_security_emulation_enabled = Some(rdr.read_i32()?);
            m_web_security_emulation_host_url = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        }

        Ok(EditorSettings {
            m_external_version_control_support,
            m_serialization_mode,
            m_web_security_emulation_enabled,
            m_line_endings_for_new_scripts,
            m_web_security_emulation_host_url,
            m_default_behavior_mode,
            m_sprite_packer_mode,
            m_prefab_regular_environment,
            m_sprite_packer_padding_power,
            m_prefab_ui_environment,
            m_project_generation_included_extensions,
            m_etc_texture_compressor_behavior,
            m_project_generation_root_namespace,
            m_etc_texture_fast_compressor,
            m_sprite_packer_cache_size,
            m_user_generated_project_suffix,
            m_etc_texture_normal_compressor,
            m_bc7_texture_compressor,
            m_collab_editor_settings,
            m_etc_texture_best_compressor,
            m_enable_texture_streaming_in_edit_mode,
            m_enable_texture_streaming_in_play_mode,
            m_async_shader_compilation,
            m_caching_shader_preprocessor,
            m_enter_play_mode_options_enabled,
            m_prefab_mode_allow_auto_save,
            m_enable_editor_async_cpu_texture_loading,
            m_show_lightmap_resolution_overlay,
            m_enter_play_mode_options,
            m_game_object_naming_digits,
            m_block_shaders,
            m_game_object_naming_scheme,
            m_unlock_block_shaders,
            m_use_legacy_probe_sample_count,
            m_asset_naming_uses_space,
            m_asset_pipeline_mode,
            m_cache_server_mode,
            m_serialize_inline_mappings_on_one_line,
            m_cache_server_endpoint,
            m_disable_cookies_in_lightmapper,
            m_cache_server_namespace_prefix,
            m_inspector_use_imgui_default_inspector,
            m_cache_server_enable_download,
            m_cache_server_enable_upload,
            m_cache_server_validation_mode,
            m_refresh_import_mode,
            m_shadowmask_stitching,
            m_cache_server_enable_auth,
            m_cache_server_enable_tls,
            m_cache_server_download_batch_size,
            m_cache_server_import_result_caching_enabled,
            m_enable_enlighten_baked_gi,
            m_referenced_clips_exact_naming,
            m_force_asset_unload_and_gc_on_scene_load,
            m_hide_build_profile_classic_platforms,
        })
    }
}
