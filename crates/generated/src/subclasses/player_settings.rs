//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aspect_ratios::AspectRatios;
use crate::subclasses::build_target_batching::BuildTargetBatching;
use crate::subclasses::build_target_icons::BuildTargetIcons;

#[derive(Debug, Clone)]
pub struct PlayerSettings {
    pub m_object_hide_flags: u32,
    pub android_profiler: bool,
    pub default_screen_orientation: i32,
    pub target_device: i32,
    pub target_platform: i32,
    pub target_resolution: i32,
    pub company_name: String,
    pub product_name: String,
    pub android_license_public_key: String,
    pub default_screen_width: i32,
    pub default_screen_height: i32,
    pub default_screen_width_web: i32,
    pub default_screen_height_web: i32,
    pub m_rendering_path: i32,
    pub display_resolution_dialog: i32,
    pub allowed_autorotate_to_portrait: bool,
    pub allowed_autorotate_to_portrait_upside_down: bool,
    pub allowed_autorotate_to_landscape_right: bool,
    pub allowed_autorotate_to_landscape_left: bool,
    pub use_os_autorotation: bool,
    pub use32_bit_display_buffer: bool,
    pub default_is_full_screen: bool,
    pub use_alpha_in_dashboard: bool,
    pub run_in_background: bool,
    pub capture_single_screen: bool,
    pub override_i_pod_music: bool,
    pub use_player_log: bool,
    pub use_mac_app_store_validation: bool,
    pub xbox_skin_on_gpu: bool,
    pub m_supported_aspect_ratios: AspectRatios,
    pub i_phone_bundle_identifier: String,
    pub i_phone_bundle_version: String,
    pub android_bundle_version_code: i32,
    pub android_min_sdk_version: i32,
    pub aot_options: String,
    pub api_compatibility_level: i32,
    pub i_phone_stripping_level: i32,
    pub i_phone_script_call_optimization: i32,
    pub force_internet_permission: bool,
    pub force_sd_card_permission: bool,
    pub create_wallpaper: bool,
    pub strip_unused_mesh_channels: bool,
    pub i_phone_sdk_version: i32,
    pub i_phone_target_os_version: i32,
    pub u_i_prerendered_icon: bool,
    pub u_i_requires_persistent_wi_fi: bool,
    pub u_i_status_bar_hidden: bool,
    pub u_i_exit_on_suspend: bool,
    pub u_i_status_bar_style: i32,
    pub i_phone_splash_screen: PPtr,
    pub i_phone_high_res_splash_screen: PPtr,
    pub i_pad_portrait_splash_screen: PPtr,
    pub i_pad_landscape_splash_screen: PPtr,
    pub android_target_device: i32,
    pub android_target_graphics: i32,
    pub android_splash_screen_scale: i32,
    pub android_keystore_name: String,
    pub android_keyalias_name: String,
    pub resolution_dialog_banner: PPtr,
    pub m_build_target_icons: Vec<BuildTargetIcons>,
    pub m_build_target_batching: Vec<BuildTargetBatching>,
    pub web_player_template: String,
    pub m_template_custom_keys: Vec<String>,
    pub m_template_custom_values: Vec<String>,
    pub wii_region: i32,
    pub wii_game_code: String,
    pub wii_game_version: String,
    pub wii_company_code: String,
    pub wii_keyboard_enabled: i32,
    pub wii_supports_nunchuk: bool,
    pub wii_supports_classic_controller: bool,
    pub wii_supports_balance_board: bool,
    pub wii_supports_motion_plus: bool,
    pub wii_controller_count: i32,
    pub xbox_title_id: String,
    pub xbox_image_xex_path: String,
    pub xbox_spa_path: String,
    pub xbox_generate_spa: i32,
    pub xbox_deploy_kinect_resources: i32,
    pub ps3_title_config_path: String,
    pub ps3_dlc_config_path: String,
    pub ps3_thumbnail_path: String,
    pub ps3_background_path: String,
    pub ps3_sound_path: String,
    pub first_streamed_level_with_resources: i32,
    pub unity_rebuild_library_version: i32,
    pub unity_forward_compatible_version: i32,
    pub unity_standard_assets_version: i32,
    pub unity_shaders_version: i32,
}

impl crate::Parse for PlayerSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let android_profiler = r.read_bool()?;
        r.align(4)?;
        let default_screen_orientation = r.read_i32()?;
        let target_device = r.read_i32()?;
        let target_platform = r.read_i32()?;
        let target_resolution = r.read_i32()?;
        r.align(4)?;
        let company_name = r.read_aligned_string()?;
        r.align(4)?;
        let product_name = r.read_aligned_string()?;
        r.align(4)?;
        let android_license_public_key = r.read_aligned_string()?;
        r.align(4)?;
        let default_screen_width = r.read_i32()?;
        let default_screen_height = r.read_i32()?;
        let default_screen_width_web = r.read_i32()?;
        let default_screen_height_web = r.read_i32()?;
        let m_rendering_path = r.read_i32()?;
        let display_resolution_dialog = r.read_i32()?;
        let allowed_autorotate_to_portrait = r.read_bool()?;
        let allowed_autorotate_to_portrait_upside_down = r.read_bool()?;
        let allowed_autorotate_to_landscape_right = r.read_bool()?;
        let allowed_autorotate_to_landscape_left = r.read_bool()?;
        let use_os_autorotation = r.read_bool()?;
        let use32_bit_display_buffer = r.read_bool()?;
        r.align(4)?;
        let default_is_full_screen = r.read_bool()?;
        let use_alpha_in_dashboard = r.read_bool()?;
        let run_in_background = r.read_bool()?;
        let capture_single_screen = r.read_bool()?;
        let override_i_pod_music = r.read_bool()?;
        let use_player_log = r.read_bool()?;
        let use_mac_app_store_validation = r.read_bool()?;
        let xbox_skin_on_gpu = r.read_bool()?;
        r.align(4)?;
        let m_supported_aspect_ratios = AspectRatios::parse(r, ver)?;
        r.align(4)?;
        let i_phone_bundle_identifier = r.read_aligned_string()?;
        r.align(4)?;
        let i_phone_bundle_version = r.read_aligned_string()?;
        r.align(4)?;
        let android_bundle_version_code = r.read_i32()?;
        let android_min_sdk_version = r.read_i32()?;
        let aot_options = r.read_aligned_string()?;
        r.align(4)?;
        let api_compatibility_level = r.read_i32()?;
        let i_phone_stripping_level = r.read_i32()?;
        let i_phone_script_call_optimization = r.read_i32()?;
        let force_internet_permission = r.read_bool()?;
        let force_sd_card_permission = r.read_bool()?;
        let create_wallpaper = r.read_bool()?;
        let strip_unused_mesh_channels = r.read_bool()?;
        r.align(4)?;
        let i_phone_sdk_version = r.read_i32()?;
        let i_phone_target_os_version = r.read_i32()?;
        let u_i_prerendered_icon = r.read_bool()?;
        let u_i_requires_persistent_wi_fi = r.read_bool()?;
        let u_i_status_bar_hidden = r.read_bool()?;
        let u_i_exit_on_suspend = r.read_bool()?;
        let u_i_status_bar_style = r.read_i32()?;
        let i_phone_splash_screen = crate::PPtr::parse(r, ver)?;
        let i_phone_high_res_splash_screen = crate::PPtr::parse(r, ver)?;
        let i_pad_portrait_splash_screen = crate::PPtr::parse(r, ver)?;
        let i_pad_landscape_splash_screen = crate::PPtr::parse(r, ver)?;
        let android_target_device = r.read_i32()?;
        let android_target_graphics = r.read_i32()?;
        let android_splash_screen_scale = r.read_i32()?;
        let android_keystore_name = r.read_aligned_string()?;
        r.align(4)?;
        let android_keyalias_name = r.read_aligned_string()?;
        r.align(4)?;
        let resolution_dialog_banner = crate::PPtr::parse(r, ver)?;
        let m_build_target_icons = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildTargetIcons::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_build_target_batching = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildTargetBatching::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let web_player_template = r.read_aligned_string()?;
        r.align(4)?;
        let m_template_custom_keys = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_template_custom_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let wii_region = r.read_i32()?;
        let wii_game_code = r.read_aligned_string()?;
        r.align(4)?;
        let wii_game_version = r.read_aligned_string()?;
        r.align(4)?;
        let wii_company_code = r.read_aligned_string()?;
        r.align(4)?;
        let wii_keyboard_enabled = r.read_i32()?;
        let wii_supports_nunchuk = r.read_bool()?;
        let wii_supports_classic_controller = r.read_bool()?;
        let wii_supports_balance_board = r.read_bool()?;
        let wii_supports_motion_plus = r.read_bool()?;
        let wii_controller_count = r.read_i32()?;
        let xbox_title_id = r.read_aligned_string()?;
        r.align(4)?;
        let xbox_image_xex_path = r.read_aligned_string()?;
        r.align(4)?;
        let xbox_spa_path = r.read_aligned_string()?;
        r.align(4)?;
        let xbox_generate_spa = r.read_i32()?;
        let xbox_deploy_kinect_resources = r.read_i32()?;
        let ps3_title_config_path = r.read_aligned_string()?;
        r.align(4)?;
        let ps3_dlc_config_path = r.read_aligned_string()?;
        r.align(4)?;
        let ps3_thumbnail_path = r.read_aligned_string()?;
        r.align(4)?;
        let ps3_background_path = r.read_aligned_string()?;
        r.align(4)?;
        let ps3_sound_path = r.read_aligned_string()?;
        r.align(4)?;
        let first_streamed_level_with_resources = r.read_i32()?;
        let unity_rebuild_library_version = r.read_i32()?;
        let unity_forward_compatible_version = r.read_i32()?;
        let unity_standard_assets_version = r.read_i32()?;
        let unity_shaders_version = r.read_i32()?;

        Ok(PlayerSettings {
            m_object_hide_flags,
            android_profiler,
            default_screen_orientation,
            target_device,
            target_platform,
            target_resolution,
            company_name,
            product_name,
            android_license_public_key,
            default_screen_width,
            default_screen_height,
            default_screen_width_web,
            default_screen_height_web,
            m_rendering_path,
            display_resolution_dialog,
            allowed_autorotate_to_portrait,
            allowed_autorotate_to_portrait_upside_down,
            allowed_autorotate_to_landscape_right,
            allowed_autorotate_to_landscape_left,
            use_os_autorotation,
            use32_bit_display_buffer,
            default_is_full_screen,
            use_alpha_in_dashboard,
            run_in_background,
            capture_single_screen,
            override_i_pod_music,
            use_player_log,
            use_mac_app_store_validation,
            xbox_skin_on_gpu,
            m_supported_aspect_ratios,
            i_phone_bundle_identifier,
            i_phone_bundle_version,
            android_bundle_version_code,
            android_min_sdk_version,
            aot_options,
            api_compatibility_level,
            i_phone_stripping_level,
            i_phone_script_call_optimization,
            force_internet_permission,
            force_sd_card_permission,
            create_wallpaper,
            strip_unused_mesh_channels,
            i_phone_sdk_version,
            i_phone_target_os_version,
            u_i_prerendered_icon,
            u_i_requires_persistent_wi_fi,
            u_i_status_bar_hidden,
            u_i_exit_on_suspend,
            u_i_status_bar_style,
            i_phone_splash_screen,
            i_phone_high_res_splash_screen,
            i_pad_portrait_splash_screen,
            i_pad_landscape_splash_screen,
            android_target_device,
            android_target_graphics,
            android_splash_screen_scale,
            android_keystore_name,
            android_keyalias_name,
            resolution_dialog_banner,
            m_build_target_icons,
            m_build_target_batching,
            web_player_template,
            m_template_custom_keys,
            m_template_custom_values,
            wii_region,
            wii_game_code,
            wii_game_version,
            wii_company_code,
            wii_keyboard_enabled,
            wii_supports_nunchuk,
            wii_supports_classic_controller,
            wii_supports_balance_board,
            wii_supports_motion_plus,
            wii_controller_count,
            xbox_title_id,
            xbox_image_xex_path,
            xbox_spa_path,
            xbox_generate_spa,
            xbox_deploy_kinect_resources,
            ps3_title_config_path,
            ps3_dlc_config_path,
            ps3_thumbnail_path,
            ps3_background_path,
            ps3_sound_path,
            first_streamed_level_with_resources,
            unity_rebuild_library_version,
            unity_forward_compatible_version,
            unity_standard_assets_version,
            unity_shaders_version,
        })
    }
}
