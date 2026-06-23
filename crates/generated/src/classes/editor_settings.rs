//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::collab_editor_settings::CollabEditorSettings;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct EditorSettings {
    pub parent: Object,
    pub m_external_version_control_support: Option<i32>,
    pub m_serialization_mode: Option<i32>,
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
    pub m_enable_texture_streaming_in_play_mode: Option<bool>,
    pub m_async_shader_compilation: Option<bool>,
    pub m_enable_roslyn_analyzers: Option<bool>,
    pub m_enter_play_mode_options_enabled: Option<bool>,
    pub m_caching_shader_preprocessor: Option<bool>,
    pub m_show_lightmap_resolution_overlay: Option<bool>,
    pub m_enter_play_mode_options: Option<i32>,
    pub m_prefab_mode_allow_auto_save: Option<bool>,
    pub m_enable_editor_async_cpu_texture_loading: Option<bool>,
    pub m_use_legacy_probe_sample_count: Option<bool>,
    pub m_serialize_inline_mappings_on_one_line: Option<bool>,
    pub m_game_object_naming_digits: Option<i32>,
    pub m_block_shaders: Option<bool>,
    pub m_asset_pipeline_mode: Option<i32>,
    pub m_game_object_naming_scheme: Option<i32>,
    pub m_unlock_block_shaders: Option<bool>,
    pub m_cache_server_mode: Option<i32>,
    pub m_disable_cookies_in_lightmapper: Option<bool>,
    pub m_asset_naming_uses_space: Option<bool>,
    pub m_cache_server_endpoint: Option<String>,
    pub m_cache_server_namespace_prefix: Option<String>,
    pub m_cache_server_enable_download: Option<bool>,
    pub m_cache_server_enable_upload: Option<bool>,
    pub m_inspector_use_imgui_default_inspector: Option<bool>,
    pub m_cache_server_enable_auth: Option<bool>,
    pub m_cache_server_enable_tls: Option<bool>,
    pub m_cache_server_validation_mode: Option<i32>,
    pub m_refresh_import_mode: Option<i32>,
    pub m_recalculate_environment_lighting: Option<bool>,
    pub m_shadowmask_stitching: Option<bool>,
    pub m_cache_server_download_batch_size: Option<i32>,
    pub m_cache_server_import_result_caching_enabled: Option<bool>,
    pub m_enable_enlighten_baked_gi: Option<bool>,
    pub m_referenced_clips_exact_naming: Option<bool>,
    pub m_force_asset_unload_and_gc_on_scene_load: Option<bool>,
    pub m_hide_build_profile_classic_platforms: Option<bool>,
}

impl crate::Parse for EditorSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_external_version_control_support = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_serialization_mode = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_web_security_emulation_enabled = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_line_endings_for_new_scripts = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_web_security_emulation_host_url = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_default_behavior_mode = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sprite_packer_mode = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_prefab_regular_environment = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_sprite_packer_padding_power = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_prefab_ui_environment = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_project_generation_included_extensions = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_etc_texture_compressor_behavior = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_project_generation_root_namespace = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_etc_texture_fast_compressor = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sprite_packer_cache_size = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_user_generated_project_suffix = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_etc_texture_normal_compressor = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_bc7_texture_compressor = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_collab_editor_settings = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = CollabEditorSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_etc_texture_best_compressor = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_texture_streaming_in_edit_mode = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_texture_streaming_in_play_mode = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_async_shader_compilation = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_roslyn_analyzers = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enter_play_mode_options_enabled = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_caching_shader_preprocessor = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_show_lightmap_resolution_overlay = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enter_play_mode_options = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_prefab_mode_allow_auto_save = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_editor_async_cpu_texture_loading = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_legacy_probe_sample_count = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_serialize_inline_mappings_on_one_line = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_game_object_naming_digits = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_block_shaders = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_asset_pipeline_mode = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_game_object_naming_scheme = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_unlock_block_shaders = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_cache_server_mode = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_disable_cookies_in_lightmapper = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_naming_uses_space = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cache_server_endpoint = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cache_server_namespace_prefix = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cache_server_enable_download = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_cache_server_enable_upload = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_inspector_use_imgui_default_inspector = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cache_server_enable_auth = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_cache_server_enable_tls = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cache_server_validation_mode = if *ver >= crate::UnityVersion::new(2019, 4, 40) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_refresh_import_mode = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_recalculate_environment_lighting = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_shadowmask_stitching = if *ver >= crate::UnityVersion::new(6000, 0, 45) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cache_server_download_batch_size = if *ver >= crate::UnityVersion::new(2021, 3, 10) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_cache_server_import_result_caching_enabled = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_enlighten_baked_gi = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_referenced_clips_exact_naming = if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_force_asset_unload_and_gc_on_scene_load = if *ver >= crate::UnityVersion::new(6000, 0, 66) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_hide_build_profile_classic_platforms = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(EditorSettings {
            parent,
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
            m_enable_roslyn_analyzers,
            m_enter_play_mode_options_enabled,
            m_caching_shader_preprocessor,
            m_show_lightmap_resolution_overlay,
            m_enter_play_mode_options,
            m_prefab_mode_allow_auto_save,
            m_enable_editor_async_cpu_texture_loading,
            m_use_legacy_probe_sample_count,
            m_serialize_inline_mappings_on_one_line,
            m_game_object_naming_digits,
            m_block_shaders,
            m_asset_pipeline_mode,
            m_game_object_naming_scheme,
            m_unlock_block_shaders,
            m_cache_server_mode,
            m_disable_cookies_in_lightmapper,
            m_asset_naming_uses_space,
            m_cache_server_endpoint,
            m_cache_server_namespace_prefix,
            m_cache_server_enable_download,
            m_cache_server_enable_upload,
            m_inspector_use_imgui_default_inspector,
            m_cache_server_enable_auth,
            m_cache_server_enable_tls,
            m_cache_server_validation_mode,
            m_refresh_import_mode,
            m_recalculate_environment_lighting,
            m_shadowmask_stitching,
            m_cache_server_download_batch_size,
            m_cache_server_import_result_caching_enabled,
            m_enable_enlighten_baked_gi,
            m_referenced_clips_exact_naming,
            m_force_asset_unload_and_gc_on_scene_load,
            m_hide_build_profile_classic_platforms,
        })
    }
}
