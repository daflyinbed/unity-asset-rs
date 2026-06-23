//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::platform_settings_data::PlatformSettingsData;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct EditorUserBuildSettings {
    pub parent: Object,
    pub m_build_location: Vec<String>,
    pub m_active_build_target: i32,
    pub m_selected_build_target_group: i32,
    pub m_active_build_target_group: Option<i32>,
    pub m_selected_build_target: Option<i32>,
    pub m_active_platform_guid: Option<Guid>,
    pub m_selected_standalone_target: i32,
    pub m_architecture_flags: i32,
    pub m_active_build_platform_group_name: Option<String>,
    pub m_selected_wii_subtarget: Option<i32>,
    pub m_selected_psp2_subtarget: Option<i32>,
    pub m_selected_facebook_target: Option<i32>,
    pub m_selected_build_platform_group_name: Option<String>,
    pub m_development: bool,
    pub m_selected_wii_debug_level: Option<i32>,
    pub m_selected_ps3_subtarget: Option<i32>,
    pub m_selected_ps4_subtarget: Option<i32>,
    pub m_connect_profiler: bool,
    pub m_selected_xbox_subtarget: Option<i32>,
    pub m_ps4_hardware_target: Option<i32>,
    pub m_selected_standalone_build_subtarget: Option<i32>,
    pub m_allow_debugging: bool,
    pub m_selected_xbox_run_method: Option<i32>,
    pub m_selected_psm_subtarget: Option<i32>,
    pub m_active_standalone_build_subtarget: Option<i32>,
    pub m_web_player_streamed: Option<bool>,
    pub m_selected_ios_build_type: Option<i32>,
    pub m_selected_wii_u_debug_level: Option<i32>,
    pub m_web_player_offline_deployment: Option<bool>,
    pub m_selected_wii_u_build_output: Option<i32>,
    pub m_android_build_system: Option<i32>,
    pub m_web_player_na_cl_support: Option<bool>,
    pub m_selected_wii_u_boot_mode: Option<i32>,
    pub m_android_build_type: Option<i32>,
    pub m_install_in_build_folder: bool,
    pub m_android_release_minification: Option<i32>,
    pub m_symlink_libraries: Option<bool>,
    pub m_android_debug_minification: Option<i32>,
    pub m_selected_android_subtarget: i32,
    pub m_selected_android_etc2_fallback: Option<i32>,
    pub m_android_device_socket_address: Option<String>,
    pub m_explicit_null_checks: Option<bool>,
    pub m_web_gl_use_pre_built_unity_engine: Option<bool>,
    pub m_android_current_deployment_target_id: Option<String>,
    pub m_enable_headless_mode: Option<bool>,
    pub m_selected_compression_type: Option<Vec<(String, i32)>>,
    pub m_xbox_compressed_xex: Option<bool>,
    pub m_web_player_deploy_online: Option<bool>,
    pub m_need_submission_materials: Option<bool>,
    pub m_symlink_trampoline: Option<bool>,
    pub m_facebook_access_token: Option<String>,
    pub m_compress_with_ps_arc: Option<bool>,
    pub m_selected_metro_build_type: Option<i32>,
    pub m_selected_metro_target: Option<i32>,
    pub m_build_with_deep_profiling_support: Option<bool>,
    pub m_dataless_player: Option<bool>,
    pub m_selected_metro_sdk: Option<i32>,
    pub m_build_scripts_only: Option<bool>,
    pub m_wait_for_player_connection: Option<bool>,
    pub m_selected_diagnostic_setting: Option<Vec<(String, i32)>>,
    pub m_selected_black_berry_subtarget: Option<i32>,
    pub m_generate_metro_reference_projects: Option<bool>,
    pub m_selected_metro_build_and_run_deploy_target: Option<i32>,
    pub m_selected_black_berry_build_type: Option<i32>,
    pub m_web_gl_optimization_level: Option<i32>,
    pub m_symlink_sources: Option<bool>,
    pub m_wsa_holographic_remoting: Option<bool>,
    pub m_selected_tizen_subtarget: Option<i32>,
    pub m_selected_wsasdk: Option<i32>,
    pub m_compress_files_in_package: Option<bool>,
    pub m_build_with_code_coverage: Option<bool>,
    pub m_selected_wsa_build_and_run_deploy_target: Option<i32>,
    pub m_explicit_divide_by_zero_checks: Option<bool>,
    pub m_move_package_to_disc_outer_edge: Option<bool>,
    pub m_generate_wsa_reference_projects: Option<bool>,
    pub m_wii_u_enable_net_api: Option<bool>,
    pub m_force_installation: Option<bool>,
    pub m_export_as_google_android_project: Option<bool>,
    pub m_xbox_one_streaming_install_launch_chunk_range: Option<i32>,
    pub m_selected_wsauwp_build_type: Option<i32>,
    pub m_explicit_array_bounds_checks: Option<bool>,
    pub m_selected_xbox_one_deploy_method: Option<i32>,
    pub m_selected_wsa_subtarget: Option<i32>,
    pub m_wsa_dot_net_native_enabled: Option<Vec<bool>>,
    pub m_create_solution_file_for_switch: Option<bool>,
    pub m_create_rom_file_for_switch: Option<bool>,
    pub m_selected_wsauwpsdk: Option<String>,
    pub m_nvn_graphics_debugger_for_switch: Option<bool>,
    pub m_nvn_shader_debugging: Option<bool>,
    pub m_generate_nintendo_switch_shader_info: Option<bool>,
    pub m_enable_rom_compression_for_switch: Option<bool>,
    pub m_save_adf_for_switch: Option<bool>,
    pub m_nvn_draw_validation_light: Option<bool>,
    pub m_force_optimize_script_compilation: Option<bool>,
    pub m_xbox_one_username: Option<String>,
    pub m_enable_debug_pad_for_switch: Option<bool>,
    pub m_rom_compression_type_for_switch: Option<i32>,
    pub m_nvn_draw_validation_heavy: Option<bool>,
    pub m_xbox_one_network_share_path: Option<String>,
    pub m_redirect_writes_to_host_mount_for_switch: Option<bool>,
    pub m_nvn_draw_validation: Option<bool>,
    pub m_enable_heap_inspector_for_switch: Option<bool>,
    pub m_rom_compression_level_for_switch: Option<i32>,
    pub m_rom_compression_config_for_switch: Option<String>,
    pub m_platform_settings: Option<Vec<(String, PlatformSettingsData)>>,
    pub m_htcs_script_debugging_for_switch: Option<bool>,
    pub m_build_app_bundle: Option<bool>,
    pub m_use_legacy_nvn_pool_allocator_for_switch: Option<bool>,
    pub m_android_create_symbols_zip: Option<bool>,
    pub m_ps5_keep_package_files: Option<bool>,
    pub m_android_create_symbols: Option<i32>,
    pub m_override_max_texture_size: Option<i32>,
    pub m_selected_wsauwpvs_version: Option<String>,
    pub m_android_use_legacy_sdk_tools: Option<bool>,
    pub m_selected_ps5_subtarget: Option<i32>,
    pub m_override_texture_compression: Option<i32>,
    pub m_nvn_aftermath: Option<bool>,
    pub m_selected_ps5_compression_type: Option<i32>,
    pub m_enable_memory_tracker_for_switch: Option<bool>,
    pub m_nvn_aftermath_level: Option<i32>,
    pub m_selected_wsa_min_uwpsdk: Option<String>,
    pub m_selected_ps5_compression_level: Option<i32>,
    pub m_il2_cpp_code_generation: Option<i32>,
    pub m_wait_for_switch_memory_tracker_on_startup: Option<bool>,
    pub m_facebook_create_package_for_submission: Option<bool>,
    pub m_selected_wsa_architecture: Option<String>,
    pub m_ps5_workspace_name: Option<String>,
    pub m_windows_device_portal_address: Option<String>,
    pub m_enable_host_io_for_switch: Option<bool>,
    pub m_windows_device_portal_username: Option<String>,
    pub m_selected_web_gl_subtarget: Option<i32>,
    pub m_selected_xbox_one_deploy_drive: Option<i32>,
    pub m_enable_unpublishable_errors_for_switch: Option<bool>,
    pub m_web_gl_client_browser_type: Option<i32>,
    pub m_web_gl_client_browser_path: Option<String>,
    pub m_web_gl_client_platform: Option<i32>,
    pub m_macos_xcode_build_config: Option<i32>,
    pub m_selected_embedded_linux_architecture: Option<i32>,
    pub m_selected_windows_build_and_run_deploy_target: Option<i32>,
    pub m_remote_device_info: Option<bool>,
    pub m_remote_device_address: Option<String>,
    pub m_remote_device_username: Option<String>,
    pub m_remote_device_exports: Option<String>,
    pub m_path_on_remote_device: Option<String>,
    pub m_selected_qnx_os_version: Option<i32>,
    pub m_selected_qnx_architecture: Option<i32>,
    pub m_active_profile_path: Option<String>,
    pub m_active_build_profile: Option<PPtr>,
    pub m_build_output_to_build_metadata_map: Option<Vec<(String, String)>>,
}

impl crate::Parse for EditorUserBuildSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_build_location = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_active_build_target = r.read_i32()?;
        let m_selected_build_target_group = r.read_i32()?;
        let m_active_build_target_group = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_build_target = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_active_platform_guid = if *ver >= crate::UnityVersion::new(6000, 0, 4) {
            Some(Guid::parse(r, ver)?)
        } else {
            None
        };
        let m_selected_standalone_target = r.read_i32()?;
        let m_architecture_flags = r.read_i32()?;
        let m_active_build_platform_group_name = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_selected_wii_subtarget = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_psp2_subtarget = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_facebook_target = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_build_platform_group_name = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_development = r.read_bool()?;
        let m_selected_wii_debug_level = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_ps3_subtarget = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_ps4_subtarget = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_connect_profiler = r.read_bool()?;
        let m_selected_xbox_subtarget = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_ps4_hardware_target = if *ver >= crate::UnityVersion::new(5, 3, 7) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_standalone_build_subtarget = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_allow_debugging = r.read_bool()?;
        let m_selected_xbox_run_method = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_psm_subtarget = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_active_standalone_build_subtarget = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_web_player_streamed = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_ios_build_type = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_wii_u_debug_level = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_web_player_offline_deployment = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_wii_u_build_output = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_android_build_system = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_web_player_na_cl_support = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_wii_u_boot_mode = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_android_build_type = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_install_in_build_folder = r.read_bool()?;
        let m_android_release_minification = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_symlink_libraries = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_android_debug_minification = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_android_subtarget = r.read_i32()?;
        let m_selected_android_etc2_fallback = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_android_device_socket_address = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_explicit_null_checks = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_web_gl_use_pre_built_unity_engine = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_android_current_deployment_target_id = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_headless_mode = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_compression_type = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_xbox_compressed_xex = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_web_player_deploy_online = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_need_submission_materials = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_symlink_trampoline = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_facebook_access_token = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_compress_with_ps_arc = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_metro_build_type = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_metro_target = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_build_with_deep_profiling_support = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_dataless_player = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_metro_sdk = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_build_scripts_only = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_wait_for_player_connection = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_diagnostic_setting = if *ver >= crate::UnityVersion::new(2022, 3, 70) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_selected_black_berry_subtarget = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_generate_metro_reference_projects = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_selected_metro_build_and_run_deploy_target = if *ver >= crate::UnityVersion::new(4, 5, 3) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_black_berry_build_type = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_web_gl_optimization_level = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_symlink_sources = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_wsa_holographic_remoting = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_tizen_subtarget = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_selected_wsasdk = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_compress_files_in_package = if *ver >= crate::UnityVersion::new(5, 5, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_build_with_code_coverage = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_wsa_build_and_run_deploy_target = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_explicit_divide_by_zero_checks = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_move_package_to_disc_outer_edge = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_generate_wsa_reference_projects = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_wii_u_enable_net_api = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_force_installation = if *ver >= crate::UnityVersion::new(5, 6, 3) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_export_as_google_android_project = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_xbox_one_streaming_install_launch_chunk_range = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_wsauwp_build_type = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_explicit_array_bounds_checks = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_xbox_one_deploy_method = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_wsa_subtarget = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_wsa_dot_net_native_enabled = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_create_solution_file_for_switch = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_create_rom_file_for_switch = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_wsauwpsdk = if *ver >= crate::UnityVersion::new(5, 5, 2) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_nvn_graphics_debugger_for_switch = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_nvn_shader_debugging = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_generate_nintendo_switch_shader_info = if *ver >= crate::UnityVersion::new(2018, 4, 35) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_rom_compression_for_switch = if *ver >= crate::UnityVersion::new(2019, 4, 38) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_save_adf_for_switch = if *ver >= crate::UnityVersion::new(2019, 4, 38) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_nvn_draw_validation_light = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_force_optimize_script_compilation = if *ver >= crate::UnityVersion::new(5, 2, 2) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_xbox_one_username = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_debug_pad_for_switch = if *ver >= crate::UnityVersion::new(5, 6, 2) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_rom_compression_type_for_switch = if *ver >= crate::UnityVersion::new(2019, 4, 38) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_nvn_draw_validation_heavy = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_xbox_one_network_share_path = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_redirect_writes_to_host_mount_for_switch = if *ver >= crate::UnityVersion::new(5, 6, 2) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_nvn_draw_validation = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_heap_inspector_for_switch = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_rom_compression_level_for_switch = if *ver >= crate::UnityVersion::new(2019, 4, 38) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_rom_compression_config_for_switch = if *ver >= crate::UnityVersion::new(2019, 4, 38) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_platform_settings = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_htcs_script_debugging_for_switch = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_build_app_bundle = if *ver >= crate::UnityVersion::new(2017, 4, 17) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_legacy_nvn_pool_allocator_for_switch = if *ver >= crate::UnityVersion::new(2019, 4, 24) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_android_create_symbols_zip = if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ps5_keep_package_files = if *ver >= crate::UnityVersion::new(2019, 4, 23) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_android_create_symbols = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_override_max_texture_size = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_wsauwpvs_version = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_android_use_legacy_sdk_tools = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_ps5_subtarget = if *ver >= crate::UnityVersion::new(2019, 4, 23) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_override_texture_compression = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_nvn_aftermath = if *ver >= crate::UnityVersion::new(2022, 3, 49) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_selected_ps5_compression_type = if *ver >= crate::UnityVersion::new(2019, 4, 23) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_memory_tracker_for_switch = if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_nvn_aftermath_level = if *ver >= crate::UnityVersion::new(6000, 0, 61) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_wsa_min_uwpsdk = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_selected_ps5_compression_level = if *ver >= crate::UnityVersion::new(2019, 4, 23) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_il2_cpp_code_generation = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_wait_for_switch_memory_tracker_on_startup = if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_facebook_create_package_for_submission = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_selected_wsa_architecture = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ps5_workspace_name = if *ver >= crate::UnityVersion::new(2019, 4, 23) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_windows_device_portal_address = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_host_io_for_switch = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_windows_device_portal_username = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_selected_web_gl_subtarget = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_xbox_one_deploy_drive = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_unpublishable_errors_for_switch = if *ver >= crate::UnityVersion::new(2022, 3, 57) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_web_gl_client_browser_type = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_web_gl_client_browser_path = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_web_gl_client_platform = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_macos_xcode_build_config = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_embedded_linux_architecture = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_windows_build_and_run_deploy_target = if *ver >= crate::UnityVersion::new(2023, 1, 10) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_remote_device_info = if *ver >= crate::UnityVersion::new(2021, 2, 14) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_remote_device_address = if *ver >= crate::UnityVersion::new(2021, 2, 14) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_remote_device_username = if *ver >= crate::UnityVersion::new(2021, 2, 14) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_remote_device_exports = if *ver >= crate::UnityVersion::new(2021, 2, 14) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_path_on_remote_device = if *ver >= crate::UnityVersion::new(2021, 2, 14) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_selected_qnx_os_version = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_selected_qnx_architecture = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_active_profile_path = if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_active_build_profile = if *ver >= crate::UnityVersion::new(6000, 0, 20) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_build_output_to_build_metadata_map = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(EditorUserBuildSettings {
            parent,
            m_build_location,
            m_active_build_target,
            m_selected_build_target_group,
            m_active_build_target_group,
            m_selected_build_target,
            m_active_platform_guid,
            m_selected_standalone_target,
            m_architecture_flags,
            m_active_build_platform_group_name,
            m_selected_wii_subtarget,
            m_selected_psp2_subtarget,
            m_selected_facebook_target,
            m_selected_build_platform_group_name,
            m_development,
            m_selected_wii_debug_level,
            m_selected_ps3_subtarget,
            m_selected_ps4_subtarget,
            m_connect_profiler,
            m_selected_xbox_subtarget,
            m_ps4_hardware_target,
            m_selected_standalone_build_subtarget,
            m_allow_debugging,
            m_selected_xbox_run_method,
            m_selected_psm_subtarget,
            m_active_standalone_build_subtarget,
            m_web_player_streamed,
            m_selected_ios_build_type,
            m_selected_wii_u_debug_level,
            m_web_player_offline_deployment,
            m_selected_wii_u_build_output,
            m_android_build_system,
            m_web_player_na_cl_support,
            m_selected_wii_u_boot_mode,
            m_android_build_type,
            m_install_in_build_folder,
            m_android_release_minification,
            m_symlink_libraries,
            m_android_debug_minification,
            m_selected_android_subtarget,
            m_selected_android_etc2_fallback,
            m_android_device_socket_address,
            m_explicit_null_checks,
            m_web_gl_use_pre_built_unity_engine,
            m_android_current_deployment_target_id,
            m_enable_headless_mode,
            m_selected_compression_type,
            m_xbox_compressed_xex,
            m_web_player_deploy_online,
            m_need_submission_materials,
            m_symlink_trampoline,
            m_facebook_access_token,
            m_compress_with_ps_arc,
            m_selected_metro_build_type,
            m_selected_metro_target,
            m_build_with_deep_profiling_support,
            m_dataless_player,
            m_selected_metro_sdk,
            m_build_scripts_only,
            m_wait_for_player_connection,
            m_selected_diagnostic_setting,
            m_selected_black_berry_subtarget,
            m_generate_metro_reference_projects,
            m_selected_metro_build_and_run_deploy_target,
            m_selected_black_berry_build_type,
            m_web_gl_optimization_level,
            m_symlink_sources,
            m_wsa_holographic_remoting,
            m_selected_tizen_subtarget,
            m_selected_wsasdk,
            m_compress_files_in_package,
            m_build_with_code_coverage,
            m_selected_wsa_build_and_run_deploy_target,
            m_explicit_divide_by_zero_checks,
            m_move_package_to_disc_outer_edge,
            m_generate_wsa_reference_projects,
            m_wii_u_enable_net_api,
            m_force_installation,
            m_export_as_google_android_project,
            m_xbox_one_streaming_install_launch_chunk_range,
            m_selected_wsauwp_build_type,
            m_explicit_array_bounds_checks,
            m_selected_xbox_one_deploy_method,
            m_selected_wsa_subtarget,
            m_wsa_dot_net_native_enabled,
            m_create_solution_file_for_switch,
            m_create_rom_file_for_switch,
            m_selected_wsauwpsdk,
            m_nvn_graphics_debugger_for_switch,
            m_nvn_shader_debugging,
            m_generate_nintendo_switch_shader_info,
            m_enable_rom_compression_for_switch,
            m_save_adf_for_switch,
            m_nvn_draw_validation_light,
            m_force_optimize_script_compilation,
            m_xbox_one_username,
            m_enable_debug_pad_for_switch,
            m_rom_compression_type_for_switch,
            m_nvn_draw_validation_heavy,
            m_xbox_one_network_share_path,
            m_redirect_writes_to_host_mount_for_switch,
            m_nvn_draw_validation,
            m_enable_heap_inspector_for_switch,
            m_rom_compression_level_for_switch,
            m_rom_compression_config_for_switch,
            m_platform_settings,
            m_htcs_script_debugging_for_switch,
            m_build_app_bundle,
            m_use_legacy_nvn_pool_allocator_for_switch,
            m_android_create_symbols_zip,
            m_ps5_keep_package_files,
            m_android_create_symbols,
            m_override_max_texture_size,
            m_selected_wsauwpvs_version,
            m_android_use_legacy_sdk_tools,
            m_selected_ps5_subtarget,
            m_override_texture_compression,
            m_nvn_aftermath,
            m_selected_ps5_compression_type,
            m_enable_memory_tracker_for_switch,
            m_nvn_aftermath_level,
            m_selected_wsa_min_uwpsdk,
            m_selected_ps5_compression_level,
            m_il2_cpp_code_generation,
            m_wait_for_switch_memory_tracker_on_startup,
            m_facebook_create_package_for_submission,
            m_selected_wsa_architecture,
            m_ps5_workspace_name,
            m_windows_device_portal_address,
            m_enable_host_io_for_switch,
            m_windows_device_portal_username,
            m_selected_web_gl_subtarget,
            m_selected_xbox_one_deploy_drive,
            m_enable_unpublishable_errors_for_switch,
            m_web_gl_client_browser_type,
            m_web_gl_client_browser_path,
            m_web_gl_client_platform,
            m_macos_xcode_build_config,
            m_selected_embedded_linux_architecture,
            m_selected_windows_build_and_run_deploy_target,
            m_remote_device_info,
            m_remote_device_address,
            m_remote_device_username,
            m_remote_device_exports,
            m_path_on_remote_device,
            m_selected_qnx_os_version,
            m_selected_qnx_architecture,
            m_active_profile_path,
            m_active_build_profile,
            m_build_output_to_build_metadata_map,
        })
    }
}
