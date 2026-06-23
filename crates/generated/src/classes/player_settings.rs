//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::android_device_filter_data::AndroidDeviceFilterData;
use crate::subclasses::aspect_ratios::AspectRatios;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::splash_screen_logo::SplashScreenLogo;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vr_settings::VrSettings;
use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct PlayerSettings {
    pub parent: GlobalGameManager,
    pub android_profiler: bool,
    pub product_guid: Option<Guid>,
    pub default_screen_orientation: i32,
    pub target_device: i32,
    pub android_filter_touches_when_obscured: Option<bool>,
    pub target_platform: Option<i32>,
    pub target_resolution: Option<i32>,
    pub target_gles_graphics: Option<i32>,
    pub use_on_demand_resources: Option<bool>,
    pub android_enable_sustained_performance_mode: Option<bool>,
    pub accelerometer_frequency: Option<i32>,
    pub target_ios_graphics: Option<i32>,
    pub company_name: String,
    pub product_name: String,
    pub android_license_public_key: Option<String>,
    pub default_cursor: Option<PPtr>,
    pub default_screen_width: i32,
    pub cursor_hotspot: Option<Vector2f>,
    pub default_screen_height: i32,
    pub cloud_project_id: Option<String>,
    pub m_show_unity_splash_screen: Option<bool>,
    pub m_splash_screen_style: Option<i32>,
    pub default_screen_width_web: i32,
    pub m_virtual_reality_splash_screen: Option<PPtr>,
    pub m_splash_screen_background_color: Option<ColorRgba>,
    pub default_screen_height_web: i32,
    pub m_rendering_path: Option<i32>,
    pub m_show_unity_splash_logo: Option<bool>,
    pub display_resolution_dialog: Option<i32>,
    pub m_splash_screen_overlay_opacity: Option<f32>,
    pub allowed_autorotate_to_portrait: bool,
    pub m_active_color_space: Option<i32>,
    pub m_splash_screen_animation: Option<i32>,
    pub allowed_autorotate_to_portrait_upside_down: bool,
    pub m_mt_rendering: Option<bool>,
    pub m_mobile_rendering_path: Option<i32>,
    pub m_splash_screen_logo_style: Option<i32>,
    pub allowed_autorotate_to_landscape_right: bool,
    pub ios_show_activity_indicator_on_loading: Option<i32>,
    pub m_splash_screen_draw_mode: Option<i32>,
    pub allowed_autorotate_to_landscape_left: bool,
    pub android_show_activity_indicator_on_loading: Option<i32>,
    pub m_use_dx11: Option<bool>,
    pub m_splash_screen_background_animation_zoom: Option<f32>,
    pub use_os_autorotation: bool,
    pub m_mobile_mt_rendering: Option<bool>,
    pub m_splash_screen_logo_animation_zoom: Option<f32>,
    pub use32_bit_display_buffer: bool,
    pub m_stereoscopic3_d: Option<bool>,
    pub m_splash_screen_background_landscape_aspect: Option<f32>,
    pub default_is_full_screen: Option<bool>,
    pub m_splash_screen_background_portrait_aspect: Option<f32>,
    pub use_alpha_in_dashboard: Option<bool>,
    pub m_stack_trace_types: Option<Vec<i32>>,
    pub m_splash_screen_background_landscape_uvs: Option<Rectf>,
    pub run_in_background: bool,
    pub ios_app_in_background_behavior: Option<i32>,
    pub m_splash_screen_background_portrait_uvs: Option<Rectf>,
    pub capture_single_screen: Option<bool>,
    pub m_splash_screen_logos: Option<Vec<SplashScreenLogo>>,
    pub override_i_pod_music: Option<bool>,
    pub m_splash_screen_background_landscape: Option<PPtr>,
    pub use_player_log: bool,
    pub use24_bit_depth_buffer: Option<bool>,
    pub ios_allow_http_download: Option<bool>,
    pub tizen_show_activity_indicator_on_loading: Option<i32>,
    pub m_splash_screen_background_portrait: Option<PPtr>,
    pub use_mac_app_store_validation: bool,
    pub xbox_skin_on_gpu: Option<bool>,
    pub m_holographic_tracking_loss_screen: Option<PPtr>,
    pub m_supported_aspect_ratios: Option<AspectRatios>,
    pub m_unity_splash_logo: Option<PPtr>,
    pub default_is_native_resolution: Option<bool>,
    pub disable_depth_and_stencil_buffers: Option<bool>,
    pub enable_hw_statistics: Option<bool>,
    pub m_stereo_rendering_path: Option<i32>,
    pub xbox_enable_avatar: Option<bool>,
    pub strip_physics: Option<bool>,
    pub prepare_ios_for_recording: Option<bool>,
    pub xbox_enable_kinect: Option<bool>,
    pub resizable_window: Option<bool>,
    pub xbox_enable_kinect_auto_tracking: Option<bool>,
    pub xbox_enable_speech: Option<bool>,
    pub mobile_mt_rendering_baked: Option<bool>,
    pub unsupported_msaa_fallback: Option<i32>,
    pub m_sprite_batch_vertex_threshold: Option<i32>,
    pub wii_hio2_usage: Option<i32>,
    pub force_single_instance: Option<bool>,
    pub submit_analytics: Option<bool>,
    pub m_sprite_batch_max_vertex_count: Option<i32>,
    pub wii_loading_screen_rect_placement: Option<i32>,
    pub mute_other_audio_sources: Option<bool>,
    pub mip_stripping: Option<bool>,
    pub aid: Option<Hash128>,
    pub wii_loading_screen_background: Option<ColorRgba>,
    pub bake_collision_meshes: Option<bool>,
    pub number_of_mips_stripped: Option<i32>,
    pub player_min_open_gles_version: Option<i32>,
    pub wii_loading_screen_period: Option<i32>,
    pub xbox_enable_fitness: Option<bool>,
    pub gpu_skinning: Option<bool>,
    pub wii_loading_screen_file_name: Option<String>,
    pub mac_fullscreen_mode: Option<i32>,
    pub ios_use_custom_app_background_behavior: Option<bool>,
    pub wii_loading_screen_rect: Option<Rectf>,
    pub xbox_speech_db: Option<u32>,
    pub xbox_pix_texture_capture: Option<bool>,
    pub debug_unload_mode: Option<i32>,
    pub xbox_enable_head_orientation: Option<bool>,
    pub number_of_mips_stripped_per_mipmap_limit_group: Option<Vec<(String, i32)>>,
    pub graphics_jobs: Option<bool>,
    pub visible_in_background: Option<bool>,
    pub preserve_framebuffer_alpha: Option<bool>,
    pub d3d9_fullscreen_mode: Option<i32>,
    pub xbox_enable_guest: Option<bool>,
    pub android_blit_type: Option<i32>,
    pub android_start_in_fullscreen: Option<bool>,
    pub i_phone_bundle_identifier: Option<String>,
    pub d3d11_force_exclusive_mode: Option<bool>,
    pub d3d11_fullscreen_mode: Option<i32>,
    pub allow_fullscreen_switch: Option<bool>,
    pub android_render_outside_safe_area: Option<bool>,
    pub video_memory_for_vertex_buffers: Option<i32>,
    pub android_use_swappy: Option<bool>,
    pub mac_retina_support: Option<bool>,
    pub android_resizable_window: Option<bool>,
    pub metro_enable_independent_input_source: Option<bool>,
    pub xbox_one_resolution: Option<i32>,
    pub n3ds_disable_stereoscopic_view: Option<bool>,
    pub defer_system_gestures_mode: Option<i32>,
    pub force_ios_speakers_when_recording: Option<bool>,
    pub android_default_window_width: Option<i32>,
    pub metro_enable_low_latency_presentation_api: Option<bool>,
    pub ps3_splash_screen: Option<PPtr>,
    pub n3ds_enable_shared_list_opt: Option<bool>,
    pub hide_home_button: Option<bool>,
    pub android_default_window_height: Option<i32>,
    pub adjust_iosfps_using_thermal_state: Option<bool>,
    pub n3ds_enable_v_sync: Option<bool>,
    pub android_minimum_window_width: Option<i32>,
    pub thermal_state_serious_iosfps: Option<i32>,
    pub psp2_power_mode: Option<i32>,
    pub xbox_enable_pix_sampling: Option<bool>,
    pub android_minimum_window_height: Option<i32>,
    pub thermal_state_critical_iosfps: Option<i32>,
    pub psp2_acquire_bgm: Option<bool>,
    pub xbox_enable_enable_render_thread_runs_jobs: Option<bool>,
    pub android_fullscreen_mode: Option<i32>,
    pub android_display_options: Option<i32>,
    pub bundle_identifier: Option<String>,
    pub ui_use16_bit_depth_buffer: Option<bool>,
    pub mac_app_store_category: Option<String>,
    pub android_resizeable_activity: Option<bool>,
    pub bundle_version: Option<String>,
    pub preloaded_assets: Option<Vec<PPtr>>,
    pub wii_utv_resolution: Option<i32>,
    pub ignore_alpha_clear: Option<bool>,
    pub android_requested_visible_insets: Option<i32>,
    pub wii_u_game_pad_msaa: Option<i32>,
    pub android_system_bars_behavior: Option<i32>,
    pub wii_u_supports_nunchuk: Option<bool>,
    pub android_auto_rotation_behavior: Option<i32>,
    pub xbox_one_disable_kinect_gpu_reservation: Option<bool>,
    pub wii_u_supports_classic_controller: Option<bool>,
    pub xbox_one_mono_logging_level: Option<i32>,
    pub android_predictive_back_support: Option<bool>,
    pub android_application_entry: Option<i32>,
    pub virtual_reality_supported: Option<bool>,
    pub wii_u_supports_balance_board: Option<bool>,
    pub xbox_one_logging_level: Option<i32>,
    pub use_flip_model_swapchain: Option<bool>,
    pub project_id: Option<String>,
    pub wii_u_supports_motion_plus: Option<bool>,
    pub project_name: Option<String>,
    pub wii_u_supports_pro_controller: Option<bool>,
    pub organization_id: Option<String>,
    pub wii_u_allow_screen_capture: Option<bool>,
    pub graphics_job_mode: Option<i32>,
    pub cloud_enabled: Option<bool>,
    pub wii_u_controller_count: Option<i32>,
    pub audio_spatial_experience: Option<i32>,
    pub dedicated_server_optimizations: Option<bool>,
    pub fullscreen_mode: Option<i32>,
    pub call_on_disable_on_asset_bundle_unload: Option<bool>,
    pub metal_framebuffer_only: Option<bool>,
    pub xbox_one_s_resolution: Option<i32>,
    pub xbox_one_x_resolution: Option<i32>,
    pub mesh_deformation: Option<i32>,
    pub single_pass_stereo_rendering: Option<bool>,
    pub xbox_one_disable_esram: Option<bool>,
    pub protect_graphics_memory: Option<bool>,
    pub xbox_one_present_immediate_threshold: Option<u32>,
    pub xbox_one_enable_type_optimization: Option<bool>,
    pub switch_queue_command_memory: Option<i32>,
    pub vulkan_enable_set_srgb_write: Option<bool>,
    pub switch_queue_control_memory: Option<i32>,
    pub switch_queue_compute_memory: Option<i32>,
    pub switch_nvn_shader_pools_granularity: Option<i32>,
    pub switch_nvn_default_pools_granularity: Option<i32>,
    pub metro_input_source: Option<i32>,
    pub switch_nvn_other_pools_granularity: Option<i32>,
    pub wsa_transparent_swapchain: Option<bool>,
    pub vulkan_num_swapchain_buffers: Option<u32>,
    pub m_holographic_pause_on_tracking_loss: Option<bool>,
    pub vulkan_use_sw_command_buffers: Option<bool>,
    pub switch_nvn_max_public_texture_id_count: Option<i32>,
    pub stadia_present_mode: Option<i32>,
    pub switch_nvn_max_public_sampler_id_count: Option<i32>,
    pub stadia_target_framerate: Option<i32>,
    pub xbox_one_enable7th_core: Option<bool>,
    pub vr_settings: Option<VrSettings>,
    pub is_wsa_holographic_remoting_enabled: Option<bool>,
    pub vulkan_enable_pre_transform: Option<bool>,
    pub enable_frame_timing_stats: Option<bool>,
    pub metal_use_metal_display_link: Option<bool>,
    pub use_hdr_display: Option<bool>,
    pub vulkan_enable_late_acquire_next_image: Option<bool>,
    pub m_color_gamuts: Option<Vec<i32>>,
    pub target_pixel_density: Option<i32>,
    pub resolution_scaling_mode: Option<i32>,
    pub android_supported_aspect_ratio: Option<i32>,
    pub switch_gpu_scratch_pool_granularity: Option<i32>,
    pub enable_new_input_system: Option<bool>,
    pub android_max_aspect_ratio: Option<f32>,
    pub switch_nvn_graphics_firmware_memory: Option<i32>,
    pub switch_allow_gpu_scratch_shrinking: Option<bool>,
    pub d3dhdr_bit_depth: Option<i32>,
    pub vulkan_enable_command_buffer_recycling: Option<bool>,
    pub switch_max_worker_multiple: Option<i32>,
    pub framebuffer_depth_memoryless_mode: Option<i32>,
    pub enable_native_platform_backends_for_new_input_system: Option<bool>,
    pub disable_old_input_manager_support: Option<bool>,
    pub legacy_clamp_blend_shape_weights: Option<bool>,
    pub load_store_debug_mode_enabled: Option<bool>,
    pub switch_graphics_jobs_sync_after_kick: Option<bool>,
    pub invalidated_pattern_texture: Option<PPtr>,
    pub virtual_texturing_support_enabled: Option<bool>,
    pub active_input_handler: Option<i32>,
    pub enable_open_gl_profiler_gpu_recorders: Option<bool>,
    pub vision_os_bundle_version: Option<String>,
    pub quality_settings_names: Option<Vec<String>>,
    pub tv_os_bundle_version: Option<String>,
    pub reset_resolution_on_window_resize: Option<bool>,
    pub hdr_bit_depth: Option<i32>,
    pub allow_hdr_display_support: Option<bool>,
    pub windows_gamepad_backend_hint: Option<i32>,
    pub player_data_path: Option<String>,
    pub force_srgb_blit: Option<bool>,
    pub android_min_aspect_ratio: Option<f32>,
    pub upload_cleared_texture_data_after_creation_from_script: Option<bool>,
    pub insecure_http_option: Option<i32>,
    pub hmi_loading_image: Option<PPtr>,
    pub enable_gamepad_input: Option<bool>,
    pub cpu_configuration: Option<Vec<i32>>,
    pub platform_requires_readable_assets: Option<bool>,
    pub enable_direct_storage: Option<bool>,
    pub android_vulkan_deny_filter_list: Option<Vec<AndroidDeviceFilterData>>,
    pub android_vulkan_allow_filter_list: Option<Vec<AndroidDeviceFilterData>>,
    pub android_vulkan_device_filter_list_asset: Option<PPtr>,
    pub d3d12_device_filter_list_asset: Option<PPtr>,
    pub allowed_http_connections: Option<i32>,
}

impl crate::Parse for PlayerSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let android_profiler = r.read_bool()?;
        r.align(4)?;
        let product_guid = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(Guid::parse(r, ver)?)
        } else {
            None
        };
        let default_screen_orientation = r.read_i32()?;
        let target_device = r.read_i32()?;
        let android_filter_touches_when_obscured = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let target_platform = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let target_resolution = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let target_gles_graphics = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let use_on_demand_resources = if *ver >= crate::UnityVersion::new(5, 2, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_enable_sustained_performance_mode = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let accelerometer_frequency = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let target_ios_graphics = if *ver >= crate::UnityVersion::new(4, 6, 3) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let company_name = r.read_aligned_string()?;
        r.align(4)?;
        let product_name = r.read_aligned_string()?;
        r.align(4)?;
        let android_license_public_key = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let default_cursor = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let default_screen_width = r.read_i32()?;
        let cursor_hotspot = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let default_screen_height = r.read_i32()?;
        let cloud_project_id = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_show_unity_splash_screen = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_splash_screen_style = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let default_screen_width_web = r.read_i32()?;
        let m_virtual_reality_splash_screen = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_splash_screen_background_color = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let default_screen_height_web = r.read_i32()?;
        let m_rendering_path = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_show_unity_splash_logo = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let display_resolution_dialog = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_splash_screen_overlay_opacity = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let allowed_autorotate_to_portrait = r.read_bool()?;
        let m_active_color_space = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_splash_screen_animation = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let allowed_autorotate_to_portrait_upside_down = r.read_bool()?;
        let m_mt_rendering = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mobile_rendering_path = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_splash_screen_logo_style = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let allowed_autorotate_to_landscape_right = r.read_bool()?;
        let ios_show_activity_indicator_on_loading = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_splash_screen_draw_mode = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let allowed_autorotate_to_landscape_left = r.read_bool()?;
        let android_show_activity_indicator_on_loading = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_use_dx11 = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_splash_screen_background_animation_zoom = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let use_os_autorotation = r.read_bool()?;
        let m_mobile_mt_rendering = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_splash_screen_logo_animation_zoom = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let use32_bit_display_buffer = r.read_bool()?;
        r.align(4)?;
        let m_stereoscopic3_d = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_splash_screen_background_landscape_aspect = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let default_is_full_screen = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_splash_screen_background_portrait_aspect = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let use_alpha_in_dashboard = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_stack_trace_types = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_splash_screen_background_landscape_uvs = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(Rectf::parse(r, ver)?)
        } else {
            None
        };
        let run_in_background = r.read_bool()?;
        let ios_app_in_background_behavior = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_splash_screen_background_portrait_uvs = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(Rectf::parse(r, ver)?)
        } else {
            None
        };
        let capture_single_screen = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_splash_screen_logos = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SplashScreenLogo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let override_i_pod_music = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_splash_screen_background_landscape = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let use_player_log = r.read_bool()?;
        let use24_bit_depth_buffer = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let ios_allow_http_download = if *ver >= crate::UnityVersion::new(5, 2, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let tizen_show_activity_indicator_on_loading = if *ver >= crate::UnityVersion::new(5, 4, 3) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_splash_screen_background_portrait = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let use_mac_app_store_validation = r.read_bool()?;
        let xbox_skin_on_gpu = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_holographic_tracking_loss_screen = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_supported_aspect_ratios = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = AspectRatios::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_unity_splash_logo = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let default_is_native_resolution = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let disable_depth_and_stencil_buffers = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let enable_hw_statistics = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_stereo_rendering_path = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let xbox_enable_avatar = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let strip_physics = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let prepare_ios_for_recording = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let xbox_enable_kinect = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let resizable_window = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let xbox_enable_kinect_auto_tracking = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let xbox_enable_speech = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let mobile_mt_rendering_baked = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let unsupported_msaa_fallback = if *ver >= crate::UnityVersion::new(2021, 3, 32) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sprite_batch_vertex_threshold = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let wii_hio2_usage = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let force_single_instance = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let submit_analytics = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_sprite_batch_max_vertex_count = if *ver >= crate::UnityVersion::new(6000, 0, 20) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let wii_loading_screen_rect_placement = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let mute_other_audio_sources = if *ver >= crate::UnityVersion::new(5, 3, 8) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let mip_stripping = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let aid = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(Hash128::parse(r, ver)?)
        } else {
            None
        };
        let wii_loading_screen_background = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let bake_collision_meshes = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let number_of_mips_stripped = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let player_min_open_gles_version = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let wii_loading_screen_period = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let xbox_enable_fitness = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let gpu_skinning = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let wii_loading_screen_file_name = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let mac_fullscreen_mode = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let ios_use_custom_app_background_behavior = if *ver >= crate::UnityVersion::new(2017, 4, 40) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let wii_loading_screen_rect = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(Rectf::parse(r, ver)?)
        } else {
            None
        };
        let xbox_speech_db = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let xbox_pix_texture_capture = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let debug_unload_mode = if *ver >= crate::UnityVersion::new(3, 5, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let xbox_enable_head_orientation = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let number_of_mips_stripped_per_mipmap_limit_group = if *ver >= crate::UnityVersion::new(2022, 2, 3) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let graphics_jobs = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let visible_in_background = if *ver >= crate::UnityVersion::new(4, 5, 4) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let preserve_framebuffer_alpha = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let d3d9_fullscreen_mode = if *ver >= crate::UnityVersion::new(4, 5, 3) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let xbox_enable_guest = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_blit_type = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_start_in_fullscreen = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let i_phone_bundle_identifier = if *ver >= crate::UnityVersion::new(4, 1, 2) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let d3d11_force_exclusive_mode = if *ver >= crate::UnityVersion::new(4, 5, 5) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let d3d11_fullscreen_mode = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let allow_fullscreen_switch = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_render_outside_safe_area = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let video_memory_for_vertex_buffers = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let android_use_swappy = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let mac_retina_support = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let android_resizable_window = if *ver >= crate::UnityVersion::new(2019, 4, 31) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let metro_enable_independent_input_source = if *ver >= crate::UnityVersion::new(4, 3, 3) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let xbox_one_resolution = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let n3ds_disable_stereoscopic_view = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let defer_system_gestures_mode = if *ver >= crate::UnityVersion::new(5, 6, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let force_ios_speakers_when_recording = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let android_default_window_width = if *ver >= crate::UnityVersion::new(2019, 4, 31) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let metro_enable_low_latency_presentation_api = if *ver >= crate::UnityVersion::new(4, 3, 3) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let ps3_splash_screen = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let n3ds_enable_shared_list_opt = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let hide_home_button = if *ver >= crate::UnityVersion::new(5, 6, 5) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let android_default_window_height = if *ver >= crate::UnityVersion::new(2019, 4, 31) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let adjust_iosfps_using_thermal_state = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let n3ds_enable_v_sync = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_minimum_window_width = if *ver >= crate::UnityVersion::new(2019, 4, 31) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let thermal_state_serious_iosfps = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let psp2_power_mode = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let xbox_enable_pix_sampling = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_minimum_window_height = if *ver >= crate::UnityVersion::new(2019, 4, 31) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let thermal_state_critical_iosfps = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let psp2_acquire_bgm = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let xbox_enable_enable_render_thread_runs_jobs = if *ver >= crate::UnityVersion::new(5, 3, 6) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_fullscreen_mode = if *ver >= crate::UnityVersion::new(2019, 4, 31) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let android_display_options = if *ver >= crate::UnityVersion::new(2022, 3, 68) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let bundle_identifier = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let ui_use16_bit_depth_buffer = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let mac_app_store_category = if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_resizeable_activity = if *ver >= crate::UnityVersion::new(6000, 0, 4) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let bundle_version = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let preloaded_assets = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let wii_utv_resolution = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let ignore_alpha_clear = if *ver >= crate::UnityVersion::new(5, 3, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_requested_visible_insets = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let wii_u_game_pad_msaa = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let android_system_bars_behavior = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let wii_u_supports_nunchuk = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let android_auto_rotation_behavior = if *ver >= crate::UnityVersion::new(2021, 3, 35) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let xbox_one_disable_kinect_gpu_reservation = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let wii_u_supports_classic_controller = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let xbox_one_mono_logging_level = if *ver >= crate::UnityVersion::new(5, 3, 6) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let android_predictive_back_support = if *ver >= crate::UnityVersion::new(2021, 3, 51) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let android_application_entry = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let virtual_reality_supported = if *ver >= crate::UnityVersion::new(5, 1, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let wii_u_supports_balance_board = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let xbox_one_logging_level = if *ver >= crate::UnityVersion::new(5, 3, 7) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let use_flip_model_swapchain = if *ver >= crate::UnityVersion::new(2019, 1, 7) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let project_id = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let wii_u_supports_motion_plus = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let project_name = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let wii_u_supports_pro_controller = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let organization_id = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let wii_u_allow_screen_capture = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let graphics_job_mode = if *ver >= crate::UnityVersion::new(5, 5, 2) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let cloud_enabled = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let wii_u_controller_count = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let audio_spatial_experience = if *ver >= crate::UnityVersion::new(2022, 3, 49) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let dedicated_server_optimizations = if *ver >= crate::UnityVersion::new(2022, 3, 8) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let fullscreen_mode = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let call_on_disable_on_asset_bundle_unload = if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let metal_framebuffer_only = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let xbox_one_s_resolution = if *ver >= crate::UnityVersion::new(5, 5, 6) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let xbox_one_x_resolution = if *ver >= crate::UnityVersion::new(5, 5, 6) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let mesh_deformation = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let single_pass_stereo_rendering = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let xbox_one_disable_esram = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let protect_graphics_memory = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let xbox_one_present_immediate_threshold = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let xbox_one_enable_type_optimization = if *ver >= crate::UnityVersion::new(2018, 4, 17) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let switch_queue_command_memory = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let vulkan_enable_set_srgb_write = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let switch_queue_control_memory = if *ver >= crate::UnityVersion::new(2018, 3, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let switch_queue_compute_memory = if *ver >= crate::UnityVersion::new(2018, 3, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let switch_nvn_shader_pools_granularity = if *ver >= crate::UnityVersion::new(2018, 3, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let switch_nvn_default_pools_granularity = if *ver >= crate::UnityVersion::new(2018, 3, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let metro_input_source = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let switch_nvn_other_pools_granularity = if *ver >= crate::UnityVersion::new(2018, 3, 5) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let wsa_transparent_swapchain = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let vulkan_num_swapchain_buffers = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_holographic_pause_on_tracking_loss = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let vulkan_use_sw_command_buffers = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let switch_nvn_max_public_texture_id_count = if *ver >= crate::UnityVersion::new(2018, 4, 26) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let stadia_present_mode = if *ver >= crate::UnityVersion::new(2019, 3, 15) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let switch_nvn_max_public_sampler_id_count = if *ver >= crate::UnityVersion::new(2018, 4, 26) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let stadia_target_framerate = if *ver >= crate::UnityVersion::new(2019, 3, 15) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let xbox_one_enable7th_core = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let vr_settings = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = VrSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let is_wsa_holographic_remoting_enabled = if *ver >= crate::UnityVersion::new(2018, 2, 9) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let vulkan_enable_pre_transform = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let enable_frame_timing_stats = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let metal_use_metal_display_link = if *ver >= crate::UnityVersion::new(6000, 3, 11) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let use_hdr_display = if *ver >= crate::UnityVersion::new(5, 5, 4) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let vulkan_enable_late_acquire_next_image = if *ver >= crate::UnityVersion::new(2019, 4, 6) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_color_gamuts = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let target_pixel_density = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let resolution_scaling_mode = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_supported_aspect_ratio = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let switch_gpu_scratch_pool_granularity = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let enable_new_input_system = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let android_max_aspect_ratio = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let switch_nvn_graphics_firmware_memory = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let switch_allow_gpu_scratch_shrinking = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let d3dhdr_bit_depth = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let vulkan_enable_command_buffer_recycling = if *ver >= crate::UnityVersion::new(2020, 3, 18) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let switch_max_worker_multiple = if *ver >= crate::UnityVersion::new(2021, 3, 30) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let framebuffer_depth_memoryless_mode = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let enable_native_platform_backends_for_new_input_system = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let disable_old_input_manager_support = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let legacy_clamp_blend_shape_weights = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let load_store_debug_mode_enabled = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let switch_graphics_jobs_sync_after_kick = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let invalidated_pattern_texture = if *ver >= crate::UnityVersion::new(2022, 2, 3) {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let virtual_texturing_support_enabled = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let active_input_handler = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let enable_open_gl_profiler_gpu_recorders = if *ver >= crate::UnityVersion::new(2021, 3, 2) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let vision_os_bundle_version = if *ver >= crate::UnityVersion::new(2022, 3, 18) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let quality_settings_names = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let tv_os_bundle_version = if *ver >= crate::UnityVersion::new(2022, 3, 18) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let reset_resolution_on_window_resize = if *ver >= crate::UnityVersion::new(2020, 3, 35) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let hdr_bit_depth = if *ver >= crate::UnityVersion::new(2022, 2, 15) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let allow_hdr_display_support = if *ver >= crate::UnityVersion::new(2022, 3, 11) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let windows_gamepad_backend_hint = if *ver >= crate::UnityVersion::new(2020, 3, 42) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let player_data_path = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let force_srgb_blit = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let android_min_aspect_ratio = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let upload_cleared_texture_data_after_creation_from_script = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let insecure_http_option = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let hmi_loading_image = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let enable_gamepad_input = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let cpu_configuration = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let platform_requires_readable_assets = if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let enable_direct_storage = if *ver >= crate::UnityVersion::new(6000, 4, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_vulkan_deny_filter_list = if *ver >= crate::UnityVersion::new(6000, 0, 5) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_vulkan_allow_filter_list = if *ver >= crate::UnityVersion::new(6000, 0, 5) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let android_vulkan_device_filter_list_asset = if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let d3d12_device_filter_list_asset = if *ver >= crate::UnityVersion::new(6000, 2, 7) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let allowed_http_connections = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(PlayerSettings {
            parent,
            android_profiler,
            product_guid,
            default_screen_orientation,
            target_device,
            android_filter_touches_when_obscured,
            target_platform,
            target_resolution,
            target_gles_graphics,
            use_on_demand_resources,
            android_enable_sustained_performance_mode,
            accelerometer_frequency,
            target_ios_graphics,
            company_name,
            product_name,
            android_license_public_key,
            default_cursor,
            default_screen_width,
            cursor_hotspot,
            default_screen_height,
            cloud_project_id,
            m_show_unity_splash_screen,
            m_splash_screen_style,
            default_screen_width_web,
            m_virtual_reality_splash_screen,
            m_splash_screen_background_color,
            default_screen_height_web,
            m_rendering_path,
            m_show_unity_splash_logo,
            display_resolution_dialog,
            m_splash_screen_overlay_opacity,
            allowed_autorotate_to_portrait,
            m_active_color_space,
            m_splash_screen_animation,
            allowed_autorotate_to_portrait_upside_down,
            m_mt_rendering,
            m_mobile_rendering_path,
            m_splash_screen_logo_style,
            allowed_autorotate_to_landscape_right,
            ios_show_activity_indicator_on_loading,
            m_splash_screen_draw_mode,
            allowed_autorotate_to_landscape_left,
            android_show_activity_indicator_on_loading,
            m_use_dx11,
            m_splash_screen_background_animation_zoom,
            use_os_autorotation,
            m_mobile_mt_rendering,
            m_splash_screen_logo_animation_zoom,
            use32_bit_display_buffer,
            m_stereoscopic3_d,
            m_splash_screen_background_landscape_aspect,
            default_is_full_screen,
            m_splash_screen_background_portrait_aspect,
            use_alpha_in_dashboard,
            m_stack_trace_types,
            m_splash_screen_background_landscape_uvs,
            run_in_background,
            ios_app_in_background_behavior,
            m_splash_screen_background_portrait_uvs,
            capture_single_screen,
            m_splash_screen_logos,
            override_i_pod_music,
            m_splash_screen_background_landscape,
            use_player_log,
            use24_bit_depth_buffer,
            ios_allow_http_download,
            tizen_show_activity_indicator_on_loading,
            m_splash_screen_background_portrait,
            use_mac_app_store_validation,
            xbox_skin_on_gpu,
            m_holographic_tracking_loss_screen,
            m_supported_aspect_ratios,
            m_unity_splash_logo,
            default_is_native_resolution,
            disable_depth_and_stencil_buffers,
            enable_hw_statistics,
            m_stereo_rendering_path,
            xbox_enable_avatar,
            strip_physics,
            prepare_ios_for_recording,
            xbox_enable_kinect,
            resizable_window,
            xbox_enable_kinect_auto_tracking,
            xbox_enable_speech,
            mobile_mt_rendering_baked,
            unsupported_msaa_fallback,
            m_sprite_batch_vertex_threshold,
            wii_hio2_usage,
            force_single_instance,
            submit_analytics,
            m_sprite_batch_max_vertex_count,
            wii_loading_screen_rect_placement,
            mute_other_audio_sources,
            mip_stripping,
            aid,
            wii_loading_screen_background,
            bake_collision_meshes,
            number_of_mips_stripped,
            player_min_open_gles_version,
            wii_loading_screen_period,
            xbox_enable_fitness,
            gpu_skinning,
            wii_loading_screen_file_name,
            mac_fullscreen_mode,
            ios_use_custom_app_background_behavior,
            wii_loading_screen_rect,
            xbox_speech_db,
            xbox_pix_texture_capture,
            debug_unload_mode,
            xbox_enable_head_orientation,
            number_of_mips_stripped_per_mipmap_limit_group,
            graphics_jobs,
            visible_in_background,
            preserve_framebuffer_alpha,
            d3d9_fullscreen_mode,
            xbox_enable_guest,
            android_blit_type,
            android_start_in_fullscreen,
            i_phone_bundle_identifier,
            d3d11_force_exclusive_mode,
            d3d11_fullscreen_mode,
            allow_fullscreen_switch,
            android_render_outside_safe_area,
            video_memory_for_vertex_buffers,
            android_use_swappy,
            mac_retina_support,
            android_resizable_window,
            metro_enable_independent_input_source,
            xbox_one_resolution,
            n3ds_disable_stereoscopic_view,
            defer_system_gestures_mode,
            force_ios_speakers_when_recording,
            android_default_window_width,
            metro_enable_low_latency_presentation_api,
            ps3_splash_screen,
            n3ds_enable_shared_list_opt,
            hide_home_button,
            android_default_window_height,
            adjust_iosfps_using_thermal_state,
            n3ds_enable_v_sync,
            android_minimum_window_width,
            thermal_state_serious_iosfps,
            psp2_power_mode,
            xbox_enable_pix_sampling,
            android_minimum_window_height,
            thermal_state_critical_iosfps,
            psp2_acquire_bgm,
            xbox_enable_enable_render_thread_runs_jobs,
            android_fullscreen_mode,
            android_display_options,
            bundle_identifier,
            ui_use16_bit_depth_buffer,
            mac_app_store_category,
            android_resizeable_activity,
            bundle_version,
            preloaded_assets,
            wii_utv_resolution,
            ignore_alpha_clear,
            android_requested_visible_insets,
            wii_u_game_pad_msaa,
            android_system_bars_behavior,
            wii_u_supports_nunchuk,
            android_auto_rotation_behavior,
            xbox_one_disable_kinect_gpu_reservation,
            wii_u_supports_classic_controller,
            xbox_one_mono_logging_level,
            android_predictive_back_support,
            android_application_entry,
            virtual_reality_supported,
            wii_u_supports_balance_board,
            xbox_one_logging_level,
            use_flip_model_swapchain,
            project_id,
            wii_u_supports_motion_plus,
            project_name,
            wii_u_supports_pro_controller,
            organization_id,
            wii_u_allow_screen_capture,
            graphics_job_mode,
            cloud_enabled,
            wii_u_controller_count,
            audio_spatial_experience,
            dedicated_server_optimizations,
            fullscreen_mode,
            call_on_disable_on_asset_bundle_unload,
            metal_framebuffer_only,
            xbox_one_s_resolution,
            xbox_one_x_resolution,
            mesh_deformation,
            single_pass_stereo_rendering,
            xbox_one_disable_esram,
            protect_graphics_memory,
            xbox_one_present_immediate_threshold,
            xbox_one_enable_type_optimization,
            switch_queue_command_memory,
            vulkan_enable_set_srgb_write,
            switch_queue_control_memory,
            switch_queue_compute_memory,
            switch_nvn_shader_pools_granularity,
            switch_nvn_default_pools_granularity,
            metro_input_source,
            switch_nvn_other_pools_granularity,
            wsa_transparent_swapchain,
            vulkan_num_swapchain_buffers,
            m_holographic_pause_on_tracking_loss,
            vulkan_use_sw_command_buffers,
            switch_nvn_max_public_texture_id_count,
            stadia_present_mode,
            switch_nvn_max_public_sampler_id_count,
            stadia_target_framerate,
            xbox_one_enable7th_core,
            vr_settings,
            is_wsa_holographic_remoting_enabled,
            vulkan_enable_pre_transform,
            enable_frame_timing_stats,
            metal_use_metal_display_link,
            use_hdr_display,
            vulkan_enable_late_acquire_next_image,
            m_color_gamuts,
            target_pixel_density,
            resolution_scaling_mode,
            android_supported_aspect_ratio,
            switch_gpu_scratch_pool_granularity,
            enable_new_input_system,
            android_max_aspect_ratio,
            switch_nvn_graphics_firmware_memory,
            switch_allow_gpu_scratch_shrinking,
            d3dhdr_bit_depth,
            vulkan_enable_command_buffer_recycling,
            switch_max_worker_multiple,
            framebuffer_depth_memoryless_mode,
            enable_native_platform_backends_for_new_input_system,
            disable_old_input_manager_support,
            legacy_clamp_blend_shape_weights,
            load_store_debug_mode_enabled,
            switch_graphics_jobs_sync_after_kick,
            invalidated_pattern_texture,
            virtual_texturing_support_enabled,
            active_input_handler,
            enable_open_gl_profiler_gpu_recorders,
            vision_os_bundle_version,
            quality_settings_names,
            tv_os_bundle_version,
            reset_resolution_on_window_resize,
            hdr_bit_depth,
            allow_hdr_display_support,
            windows_gamepad_backend_hint,
            player_data_path,
            force_srgb_blit,
            android_min_aspect_ratio,
            upload_cleared_texture_data_after_creation_from_script,
            insecure_http_option,
            hmi_loading_image,
            enable_gamepad_input,
            cpu_configuration,
            platform_requires_readable_assets,
            enable_direct_storage,
            android_vulkan_deny_filter_list,
            android_vulkan_allow_filter_list,
            android_vulkan_device_filter_list_asset,
            d3d12_device_filter_list_asset,
            allowed_http_connections,
        })
    }
}
