//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::platform_settings_data::PlatformSettingsData;

#[derive(Debug, Clone)]
pub struct EditorUserBuildSettings {
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
        let mut m_build_location = None;
        let mut m_active_build_target = None;
        let mut m_selected_build_target_group = None;
        let mut m_active_build_target_group = None;
        let mut m_selected_build_target = None;
        let mut m_active_platform_guid = None;
        let mut m_selected_standalone_target = None;
        let mut m_architecture_flags = None;
        let mut m_active_build_platform_group_name = None;
        let mut m_selected_wii_subtarget = None;
        let mut m_selected_psp2_subtarget = None;
        let mut m_selected_facebook_target = None;
        let mut m_selected_build_platform_group_name = None;
        let mut m_development = None;
        let mut m_selected_wii_debug_level = None;
        let mut m_selected_ps3_subtarget = None;
        let mut m_selected_ps4_subtarget = None;
        let mut m_connect_profiler = None;
        let mut m_selected_xbox_subtarget = None;
        let mut m_ps4_hardware_target = None;
        let mut m_selected_standalone_build_subtarget = None;
        let mut m_allow_debugging = None;
        let mut m_selected_xbox_run_method = None;
        let mut m_selected_psm_subtarget = None;
        let mut m_active_standalone_build_subtarget = None;
        let mut m_web_player_streamed = None;
        let mut m_selected_ios_build_type = None;
        let mut m_selected_wii_u_debug_level = None;
        let mut m_web_player_offline_deployment = None;
        let mut m_selected_wii_u_build_output = None;
        let mut m_android_build_system = None;
        let mut m_web_player_na_cl_support = None;
        let mut m_selected_wii_u_boot_mode = None;
        let mut m_android_build_type = None;
        let mut m_install_in_build_folder = None;
        let mut m_android_release_minification = None;
        let mut m_symlink_libraries = None;
        let mut m_android_debug_minification = None;
        let mut m_selected_android_subtarget = None;
        let mut m_selected_android_etc2_fallback = None;
        let mut m_android_device_socket_address = None;
        let mut m_explicit_null_checks = None;
        let mut m_web_gl_use_pre_built_unity_engine = None;
        let mut m_android_current_deployment_target_id = None;
        let mut m_enable_headless_mode = None;
        let mut m_selected_compression_type = None;
        let mut m_xbox_compressed_xex = None;
        let mut m_web_player_deploy_online = None;
        let mut m_need_submission_materials = None;
        let mut m_symlink_trampoline = None;
        let mut m_facebook_access_token = None;
        let mut m_compress_with_ps_arc = None;
        let mut m_selected_metro_build_type = None;
        let mut m_selected_metro_target = None;
        let mut m_build_with_deep_profiling_support = None;
        let mut m_dataless_player = None;
        let mut m_selected_metro_sdk = None;
        let mut m_build_scripts_only = None;
        let mut m_wait_for_player_connection = None;
        let mut m_selected_diagnostic_setting = None;
        let mut m_selected_black_berry_subtarget = None;
        let mut m_generate_metro_reference_projects = None;
        let mut m_selected_metro_build_and_run_deploy_target = None;
        let mut m_selected_black_berry_build_type = None;
        let mut m_web_gl_optimization_level = None;
        let mut m_symlink_sources = None;
        let mut m_wsa_holographic_remoting = None;
        let mut m_selected_tizen_subtarget = None;
        let mut m_selected_wsasdk = None;
        let mut m_compress_files_in_package = None;
        let mut m_build_with_code_coverage = None;
        let mut m_selected_wsa_build_and_run_deploy_target = None;
        let mut m_explicit_divide_by_zero_checks = None;
        let mut m_move_package_to_disc_outer_edge = None;
        let mut m_generate_wsa_reference_projects = None;
        let mut m_wii_u_enable_net_api = None;
        let mut m_force_installation = None;
        let mut m_export_as_google_android_project = None;
        let mut m_xbox_one_streaming_install_launch_chunk_range = None;
        let mut m_selected_wsauwp_build_type = None;
        let mut m_explicit_array_bounds_checks = None;
        let mut m_selected_xbox_one_deploy_method = None;
        let mut m_selected_wsa_subtarget = None;
        let mut m_wsa_dot_net_native_enabled = None;
        let mut m_create_solution_file_for_switch = None;
        let mut m_create_rom_file_for_switch = None;
        let mut m_selected_wsauwpsdk = None;
        let mut m_nvn_graphics_debugger_for_switch = None;
        let mut m_nvn_shader_debugging = None;
        let mut m_generate_nintendo_switch_shader_info = None;
        let mut m_enable_rom_compression_for_switch = None;
        let mut m_save_adf_for_switch = None;
        let mut m_nvn_draw_validation_light = None;
        let mut m_force_optimize_script_compilation = None;
        let mut m_xbox_one_username = None;
        let mut m_enable_debug_pad_for_switch = None;
        let mut m_rom_compression_type_for_switch = None;
        let mut m_nvn_draw_validation_heavy = None;
        let mut m_xbox_one_network_share_path = None;
        let mut m_redirect_writes_to_host_mount_for_switch = None;
        let mut m_nvn_draw_validation = None;
        let mut m_enable_heap_inspector_for_switch = None;
        let mut m_rom_compression_level_for_switch = None;
        let mut m_rom_compression_config_for_switch = None;
        let mut m_platform_settings = None;
        let mut m_htcs_script_debugging_for_switch = None;
        let mut m_build_app_bundle = None;
        let mut m_use_legacy_nvn_pool_allocator_for_switch = None;
        let mut m_android_create_symbols_zip = None;
        let mut m_ps5_keep_package_files = None;
        let mut m_android_create_symbols = None;
        let mut m_override_max_texture_size = None;
        let mut m_selected_wsauwpvs_version = None;
        let mut m_android_use_legacy_sdk_tools = None;
        let mut m_selected_ps5_subtarget = None;
        let mut m_override_texture_compression = None;
        let mut m_nvn_aftermath = None;
        let mut m_selected_ps5_compression_type = None;
        let mut m_enable_memory_tracker_for_switch = None;
        let mut m_nvn_aftermath_level = None;
        let mut m_selected_wsa_min_uwpsdk = None;
        let mut m_selected_ps5_compression_level = None;
        let mut m_il2_cpp_code_generation = None;
        let mut m_wait_for_switch_memory_tracker_on_startup = None;
        let mut m_facebook_create_package_for_submission = None;
        let mut m_selected_wsa_architecture = None;
        let mut m_ps5_workspace_name = None;
        let mut m_windows_device_portal_address = None;
        let mut m_enable_host_io_for_switch = None;
        let mut m_windows_device_portal_username = None;
        let mut m_selected_web_gl_subtarget = None;
        let mut m_selected_xbox_one_deploy_drive = None;
        let mut m_enable_unpublishable_errors_for_switch = None;
        let mut m_web_gl_client_browser_type = None;
        let mut m_web_gl_client_browser_path = None;
        let mut m_web_gl_client_platform = None;
        let mut m_macos_xcode_build_config = None;
        let mut m_selected_embedded_linux_architecture = None;
        let mut m_selected_windows_build_and_run_deploy_target = None;
        let mut m_remote_device_info = None;
        let mut m_remote_device_address = None;
        let mut m_remote_device_username = None;
        let mut m_remote_device_exports = None;
        let mut m_path_on_remote_device = None;
        let mut m_selected_qnx_os_version = None;
        let mut m_selected_qnx_architecture = None;
        let mut m_active_profile_path = None;
        let mut m_active_build_profile = None;
        let mut m_build_output_to_build_metadata_map = None;
            m_build_location = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_active_build_target = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 0, 4) {
            m_active_platform_guid = Some(Guid::parse(r, ver)?);
            m_selected_build_target = Some(r.read_i32()?);
            m_active_build_target_group = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_selected_build_target = Some(r.read_i32()?);
            m_active_build_target_group = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_active_build_target_group = Some(r.read_i32()?);
        } else {
        }
            m_selected_build_target_group = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_active_build_platform_group_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_build_platform_group_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else {
        }
            m_selected_standalone_target = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_selected_standalone_build_subtarget = Some(r.read_i32()?);
            m_active_standalone_build_subtarget = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_selected_facebook_target = Some(r.read_i32()?);
        } else {
        }
            m_architecture_flags = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_android_device_socket_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_android_current_deployment_target_id = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_compression_type = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_selected_android_etc2_fallback = Some(r.read_i32()?);
            m_android_device_socket_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_android_current_deployment_target_id = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_compression_type = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 3, 70) {
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_selected_android_etc2_fallback = Some(r.read_i32()?);
            m_android_device_socket_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_android_current_deployment_target_id = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_compression_type = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_diagnostic_setting = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_selected_android_etc2_fallback = Some(r.read_i32()?);
            m_android_device_socket_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_android_current_deployment_target_id = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_compression_type = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_selected_android_etc2_fallback = Some(r.read_i32()?);
            m_android_device_socket_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_android_current_deployment_target_id = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_compression_type = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_facebook_access_token = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_selected_android_etc2_fallback = Some(r.read_i32()?);
            m_android_device_socket_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_android_current_deployment_target_id = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_compression_type = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_facebook_access_token = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_selected_android_etc2_fallback = Some(r.read_i32()?);
            m_android_device_socket_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_compression_type = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_facebook_access_token = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_android_device_socket_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_compression_type = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_facebook_access_token = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_android_release_minification = Some(r.read_i32()?);
            m_android_debug_minification = Some(r.read_i32()?);
            m_facebook_access_token = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
            m_android_build_type = Some(r.read_i32()?);
            m_facebook_access_token = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_android_build_system = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 2) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_xbox_run_method = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_selected_ps3_subtarget = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_xbox_run_method = Some(r.read_i32()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 7) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_ps4_hardware_target = Some(r.read_i32()?);
            m_selected_ps3_subtarget = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_xbox_run_method = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_selected_ps3_subtarget = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_wii_u_debug_level = Some(r.read_i32()?);
            m_selected_wii_u_build_output = Some(r.read_i32()?);
            m_selected_wii_u_boot_mode = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_xbox_run_method = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps4_subtarget = Some(r.read_i32()?);
            m_selected_ps3_subtarget = Some(r.read_i32()?);
            m_selected_psm_subtarget = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_xbox_run_method = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_selected_psp2_subtarget = Some(r.read_i32()?);
            m_selected_ps3_subtarget = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_xbox_run_method = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_selected_wii_subtarget = Some(r.read_i32()?);
            m_selected_ps3_subtarget = Some(r.read_i32()?);
            m_selected_wii_debug_level = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_xbox_run_method = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_selected_wii_subtarget = Some(r.read_i32()?);
            m_selected_wii_debug_level = Some(r.read_i32()?);
            m_selected_xbox_subtarget = Some(r.read_i32()?);
            m_selected_xbox_run_method = Some(r.read_i32()?);
        } else {
            m_selected_wii_subtarget = Some(r.read_i32()?);
        }
            m_development = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_dataless_player = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_dataless_player = Some(r.read_bool()?);
            m_web_gl_use_pre_built_unity_engine = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_web_gl_use_pre_built_unity_engine = Some(r.read_bool()?);
        } else {
        }
            m_connect_profiler = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_build_with_deep_profiling_support = Some(r.read_bool()?);
            m_build_with_code_coverage = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_build_with_deep_profiling_support = Some(r.read_bool()?);
        } else {
        }
            m_allow_debugging = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_wait_for_player_connection = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_wait_for_player_connection = Some(r.read_bool()?);
            m_wsa_holographic_remoting = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_wsa_holographic_remoting = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_web_player_streamed = Some(r.read_bool()?);
            m_web_player_offline_deployment = Some(r.read_bool()?);
        } else {
            m_web_player_streamed = Some(r.read_bool()?);
            m_web_player_offline_deployment = Some(r.read_bool()?);
            m_web_player_na_cl_support = Some(r.read_bool()?);
        }
            m_install_in_build_folder = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_aftermath_level = Some(r.read_i32()?);
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_enable_host_io_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_aftermath = Some(r.read_bool()?);
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_enable_host_io_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 61) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_aftermath_level = Some(r.read_i32()?);
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_enable_host_io_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_aftermath = Some(r.read_bool()?);
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_enable_host_io_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_aftermath = Some(r.read_bool()?);
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 4) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 57) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_aftermath = Some(r.read_bool()?);
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_enable_unpublishable_errors_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 49) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_aftermath = Some(r.read_bool()?);
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 5) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_il2_cpp_code_generation = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(r.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_il2_cpp_code_generation = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 17) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_il2_cpp_code_generation = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_il2_cpp_code_generation = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_il2_cpp_code_generation = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_override_max_texture_size = Some(r.read_i32()?);
            m_override_texture_compression = Some(r.read_i32()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_symlink_sources = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 5) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 31) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 7) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 4) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation_light = Some(r.read_bool()?);
            m_nvn_draw_validation_heavy = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 4, 38) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_enable_rom_compression_for_switch = Some(r.read_bool()?);
            m_save_adf_for_switch = Some(r.read_bool()?);
            r.align(4)?;
            m_rom_compression_type_for_switch = Some(r.read_i32()?);
            m_rom_compression_level_for_switch = Some(r.read_i32()?);
            m_rom_compression_config_for_switch = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
            m_ps5_keep_package_files = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_ps5_subtarget = Some(r.read_i32()?);
            m_selected_ps5_compression_type = Some(r.read_i32()?);
            m_selected_ps5_compression_level = Some(r.read_i32()?);
            m_ps5_workspace_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 4, 25) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
            m_ps5_keep_package_files = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_ps5_subtarget = Some(r.read_i32()?);
            m_selected_ps5_compression_type = Some(r.read_i32()?);
            m_selected_ps5_compression_level = Some(r.read_i32()?);
            m_ps5_workspace_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 4, 24) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
            m_ps5_keep_package_files = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_ps5_subtarget = Some(r.read_i32()?);
            m_selected_ps5_compression_type = Some(r.read_i32()?);
            m_selected_ps5_compression_level = Some(r.read_i32()?);
            m_ps5_workspace_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 4, 23) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
            m_ps5_keep_package_files = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_ps5_subtarget = Some(r.read_i32()?);
            m_selected_ps5_compression_type = Some(r.read_i32()?);
            m_selected_ps5_compression_level = Some(r.read_i32()?);
            m_ps5_workspace_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 11) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 4, 35) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_generate_nintendo_switch_shader_info = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_android_use_legacy_sdk_tools = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_android_use_legacy_sdk_tools = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
            m_android_create_symbols_zip = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_nvn_shader_debugging = Some(r.read_bool()?);
            r.align(4)?;
            m_nvn_draw_validation = Some(r.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_android_use_legacy_sdk_tools = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_explicit_array_bounds_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 4, 17) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
            m_build_app_bundle = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 3) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_force_installation = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 2) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
            m_enable_debug_pad_for_switch = Some(r.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
            m_create_solution_file_for_switch = Some(r.read_bool()?);
            m_create_rom_file_for_switch = Some(r.read_bool()?);
            m_nvn_graphics_debugger_for_switch = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 1) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_compress_files_in_package = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_export_as_google_android_project = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_wii_u_enable_net_api = Some(r.read_bool()?);
            m_web_gl_optimization_level = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_symlink_trampoline = Some(r.read_bool()?);
            m_selected_ios_build_type = Some(r.read_i32()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_web_player_deploy_online = Some(r.read_bool()?);
            r.align(4)?;
            m_web_gl_optimization_level = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_need_submission_materials = Some(r.read_bool()?);
            m_compress_with_ps_arc = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_build_scripts_only = Some(r.read_bool()?);
            m_web_player_deploy_online = Some(r.read_bool()?);
            r.align(4)?;
            m_web_gl_optimization_level = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_enable_headless_mode = Some(r.read_bool()?);
            m_web_player_deploy_online = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_web_player_deploy_online = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_symlink_libraries = Some(r.read_bool()?);
            m_explicit_null_checks = Some(r.read_bool()?);
            m_xbox_compressed_xex = Some(r.read_bool()?);
            m_web_player_deploy_online = Some(r.read_bool()?);
            r.align(4)?;
        } else {
            m_symlink_libraries = Some(r.read_bool()?);
        }
            m_selected_android_subtarget = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_web_gl_client_platform = Some(r.read_i32()?);
            m_web_gl_client_browser_type = Some(r.read_i32()?);
            m_web_gl_client_browser_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_selected_windows_build_and_run_deploy_target = Some(r.read_i32()?);
            r.align(4)?;
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_active_build_profile = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_web_gl_client_platform = Some(r.read_i32()?);
            m_web_gl_client_browser_type = Some(r.read_i32()?);
            m_web_gl_client_browser_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_selected_windows_build_and_run_deploy_target = Some(r.read_i32()?);
            r.align(4)?;
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_active_build_profile = Some(crate::PPtr::parse(r, ver)?);
            m_build_output_to_build_metadata_map = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 0, 20) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_web_gl_client_platform = Some(r.read_i32()?);
            m_web_gl_client_browser_type = Some(r.read_i32()?);
            m_web_gl_client_browser_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_selected_windows_build_and_run_deploy_target = Some(r.read_i32()?);
            r.align(4)?;
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_active_build_profile = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_web_gl_client_platform = Some(r.read_i32()?);
            m_web_gl_client_browser_type = Some(r.read_i32()?);
            m_web_gl_client_browser_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_selected_windows_build_and_run_deploy_target = Some(r.read_i32()?);
            r.align(4)?;
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_active_profile_path = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_web_gl_client_platform = Some(r.read_i32()?);
            m_web_gl_client_browser_type = Some(r.read_i32()?);
            m_web_gl_client_browser_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_selected_windows_build_and_run_deploy_target = Some(r.read_i32()?);
            r.align(4)?;
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_web_gl_client_browser_type = Some(r.read_i32()?);
            m_web_gl_client_browser_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_selected_windows_build_and_run_deploy_target = Some(r.read_i32()?);
            r.align(4)?;
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_web_gl_client_browser_type = Some(r.read_i32()?);
            m_web_gl_client_browser_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 10) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_selected_windows_build_and_run_deploy_target = Some(r.read_i32()?);
            r.align(4)?;
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_selected_qnx_os_version = Some(r.read_i32()?);
            m_selected_qnx_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 14) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
            m_remote_device_info = Some(r.read_bool()?);
            r.align(4)?;
            m_remote_device_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_remote_device_exports = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_path_on_remote_device = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_selected_web_gl_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_embedded_linux_architecture = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_macos_xcode_build_config = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_windows_device_portal_address = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_windows_device_portal_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_min_uwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_architecture = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsauwpvs_version = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_facebook_create_package_for_submission = Some(r.read_bool()?);
            r.align(4)?;
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_facebook_create_package_for_submission = Some(r.read_bool()?);
            r.align(4)?;
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_force_optimize_script_compilation = Some(r.read_bool()?);
            m_facebook_create_package_for_submission = Some(r.read_bool()?);
            r.align(4)?;
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_force_optimize_script_compilation = Some(r.read_bool()?);
            m_facebook_create_package_for_submission = Some(r.read_bool()?);
            r.align(4)?;
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 2) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsauwpsdk = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_force_optimize_script_compilation = Some(r.read_bool()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_selected_wsa_subtarget = Some(r.read_i32()?);
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_force_optimize_script_compilation = Some(r.read_bool()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_force_optimize_script_compilation = Some(r.read_bool()?);
            m_platform_settings = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_wsa_dot_net_native_enabled = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_selected_black_berry_subtarget = Some(r.read_i32()?);
            m_selected_black_berry_build_type = Some(r.read_i32()?);
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_xbox_one_username = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_xbox_one_network_share_path = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_force_optimize_script_compilation = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 2) {
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_black_berry_subtarget = Some(r.read_i32()?);
            m_selected_black_berry_build_type = Some(r.read_i32()?);
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
            m_force_optimize_script_compilation = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsauwp_build_type = Some(r.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_black_berry_subtarget = Some(r.read_i32()?);
            m_selected_black_berry_build_type = Some(r.read_i32()?);
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_selected_wsasdk = Some(r.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_wsa_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_black_berry_subtarget = Some(r.read_i32()?);
            m_selected_black_berry_build_type = Some(r.read_i32()?);
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(r.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 3) {
            m_selected_metro_target = Some(r.read_i32()?);
            m_selected_metro_build_type = Some(r.read_i32()?);
            m_selected_metro_sdk = Some(r.read_i32()?);
            m_selected_metro_build_and_run_deploy_target = Some(r.read_i32()?);
            m_generate_metro_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_black_berry_subtarget = Some(r.read_i32()?);
            m_selected_black_berry_build_type = Some(r.read_i32()?);
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_selected_metro_target = Some(r.read_i32()?);
            m_selected_metro_build_type = Some(r.read_i32()?);
            m_selected_metro_sdk = Some(r.read_i32()?);
            m_generate_metro_reference_projects = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_black_berry_subtarget = Some(r.read_i32()?);
            m_selected_black_berry_build_type = Some(r.read_i32()?);
            m_selected_tizen_subtarget = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_selected_metro_target = Some(r.read_i32()?);
            m_selected_metro_build_type = Some(r.read_i32()?);
            m_selected_metro_sdk = Some(r.read_i32()?);
            m_selected_black_berry_subtarget = Some(r.read_i32()?);
            m_selected_black_berry_build_type = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_selected_metro_target = Some(r.read_i32()?);
            m_selected_metro_build_type = Some(r.read_i32()?);
            m_selected_black_berry_subtarget = Some(r.read_i32()?);
            m_selected_black_berry_build_type = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_selected_metro_build_type = Some(r.read_i32()?);
        } else {
        }

        Ok(EditorUserBuildSettings {
            m_build_location: m_build_location.unwrap(),
            m_active_build_target: m_active_build_target.unwrap(),
            m_selected_build_target_group: m_selected_build_target_group.unwrap(),
            m_active_build_target_group,
            m_selected_build_target,
            m_active_platform_guid,
            m_selected_standalone_target: m_selected_standalone_target.unwrap(),
            m_architecture_flags: m_architecture_flags.unwrap(),
            m_active_build_platform_group_name,
            m_selected_wii_subtarget,
            m_selected_psp2_subtarget,
            m_selected_facebook_target,
            m_selected_build_platform_group_name,
            m_development: m_development.unwrap(),
            m_selected_wii_debug_level,
            m_selected_ps3_subtarget,
            m_selected_ps4_subtarget,
            m_connect_profiler: m_connect_profiler.unwrap(),
            m_selected_xbox_subtarget,
            m_ps4_hardware_target,
            m_selected_standalone_build_subtarget,
            m_allow_debugging: m_allow_debugging.unwrap(),
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
            m_install_in_build_folder: m_install_in_build_folder.unwrap(),
            m_android_release_minification,
            m_symlink_libraries,
            m_android_debug_minification,
            m_selected_android_subtarget: m_selected_android_subtarget.unwrap(),
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
