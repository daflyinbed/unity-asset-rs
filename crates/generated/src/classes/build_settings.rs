//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;


/// 版本枚举：BuildSettingsIsDebugBuildVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildSettingsIsDebugBuildVariant {
    Default(bool),
    V4_0(bool),
}

impl crate::Parse for BuildSettingsIsDebugBuildVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BuildSettingsUsesOnMouseEventsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildSettingsUsesOnMouseEventsVariant {
    Default(bool),
    V4_6_2(bool),
}

impl crate::Parse for BuildSettingsUsesOnMouseEventsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 6, 2) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_6_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BuildSettingsRuntimeClassHashesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildSettingsRuntimeClassHashesVariant {
    Default(Vec<(i32, u32)>),
    V5_0(Vec<(i32, Hash128)>),
}

impl crate::Parse for BuildSettingsRuntimeClassHashesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = Hash128::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = rdr.read_u32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BuildSettingsMGraphicsApIsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildSettingsMGraphicsApIsVariant {
    Default(Vec<i32>),
    V2017_1(Vec<i32>),
}

impl crate::Parse for BuildSettingsMGraphicsApIsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuildSettings {
    pub levels: Option<Vec<String>>,
    pub scenes: Option<Vec<String>>,
    pub has_render_texture: Option<bool>,
    pub preloaded_plugins: Option<Vec<String>>,
    pub has_pro_version: bool,
    pub enabled_vr_devices: Option<Vec<String>>,
    pub is_educational_build: bool,
    pub is_no_watermark_build: Option<bool>,
    pub build_tags: Option<Vec<String>>,
    pub has_publishing_rights: bool,
    pub is_prototyping_build: Option<bool>,
    pub build_guid: Option<Guid>,
    pub has_shadows: bool,
    pub has_advanced_version: bool,
    pub is_embedded: Option<bool>,
    pub enable_dynamic_batching: bool,
    pub is_debug_build: Option<BuildSettingsIsDebugBuildVariant>,
    pub m_version: String,
    pub has_soft_shadows: Option<bool>,
    pub is_trial: Option<bool>,
    pub m_auth_token: Option<String>,
    pub has_local_light_shadows: Option<bool>,
    pub uses_on_mouse_events: Option<BuildSettingsUsesOnMouseEventsVariant>,
    pub is_wsa_holographic_remoting_enabled: Option<bool>,
    pub runtime_class_hashes: Option<BuildSettingsRuntimeClassHashesVariant>,
    pub has_oculus_plugin: Option<bool>,
    pub has_cluster_rendering: Option<bool>,
    pub enable_multiple_displays: Option<bool>,
    pub script_hashes: Option<Vec<(Hash128, Hash128)>>,
    pub m_graphics_ap_is: Option<BuildSettingsMGraphicsApIsVariant>,
}

impl crate::Parse for BuildSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut levels = None;
        let mut scenes = None;
        let mut has_render_texture = None;
        let mut preloaded_plugins = None;
        let mut has_pro_version = None;
        let mut enabled_vr_devices = None;
        let mut is_educational_build = None;
        let mut is_no_watermark_build = None;
        let mut build_tags = None;
        let mut has_publishing_rights = None;
        let mut is_prototyping_build = None;
        let mut build_guid = None;
        let mut has_shadows = None;
        let mut has_advanced_version = None;
        let mut is_embedded = None;
        let mut enable_dynamic_batching = None;
        let mut is_debug_build = None;
        let mut m_version = None;
        let mut has_soft_shadows = None;
        let mut is_trial = None;
        let mut m_auth_token = None;
        let mut has_local_light_shadows = None;
        let mut uses_on_mouse_events = None;
        let mut is_wsa_holographic_remoting_enabled = None;
        let mut runtime_class_hashes = None;
        let mut has_oculus_plugin = None;
        let mut has_cluster_rendering = None;
        let mut enable_multiple_displays = None;
        let mut script_hashes = None;
        let mut m_graphics_ap_is = None;
        if *ver >= crate::UnityVersion::new(2022, 2, 2) {
            scenes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            preloaded_plugins = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            enabled_vr_devices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            build_tags = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            scenes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            preloaded_plugins = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            enabled_vr_devices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            build_tags = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            build_guid = Some(Guid::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            scenes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            preloaded_plugins = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            enabled_vr_devices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            scenes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            preloaded_plugins = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            scenes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            preloaded_plugins = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            has_render_texture = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            levels = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            preloaded_plugins = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            has_render_texture = Some(rdr.read_bool()?);
        } else {
            levels = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            has_render_texture = Some(rdr.read_bool()?);
        }
            has_pro_version = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            is_no_watermark_build = Some(rdr.read_bool()?);
            is_prototyping_build = Some(rdr.read_bool()?);
        } else {
        }
            is_educational_build = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            is_embedded = Some(rdr.read_bool()?);
            is_trial = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            is_embedded = Some(rdr.read_bool()?);
        } else {
        }
            has_publishing_rights = Some(rdr.read_bool()?);
            has_shadows = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2018, 2, 9) {
            has_soft_shadows = Some(rdr.read_bool()?);
            has_local_light_shadows = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            has_soft_shadows = Some(rdr.read_bool()?);
            has_local_light_shadows = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            has_soft_shadows = Some(rdr.read_bool()?);
            has_local_light_shadows = Some(rdr.read_bool()?);
        } else {
        }
            has_advanced_version = Some(rdr.read_bool()?);
            enable_dynamic_batching = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 2) {
            uses_on_mouse_events = Some(BuildSettingsUsesOnMouseEventsVariant::parse(rdr, ver)?);
            has_cluster_rendering = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            is_debug_build = Some(BuildSettingsIsDebugBuildVariant::parse(rdr, ver)?);
            uses_on_mouse_events = Some(BuildSettingsUsesOnMouseEventsVariant::parse(rdr, ver)?);
            has_cluster_rendering = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            is_debug_build = Some(BuildSettingsIsDebugBuildVariant::parse(rdr, ver)?);
            uses_on_mouse_events = Some(BuildSettingsUsesOnMouseEventsVariant::parse(rdr, ver)?);
            enable_multiple_displays = Some(rdr.read_bool()?);
            has_cluster_rendering = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 6, 2) {
            is_debug_build = Some(BuildSettingsIsDebugBuildVariant::parse(rdr, ver)?);
            uses_on_mouse_events = Some(BuildSettingsUsesOnMouseEventsVariant::parse(rdr, ver)?);
            has_oculus_plugin = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            is_debug_build = Some(BuildSettingsIsDebugBuildVariant::parse(rdr, ver)?);
            uses_on_mouse_events = Some(BuildSettingsUsesOnMouseEventsVariant::parse(rdr, ver)?);
        } else {
            is_debug_build = Some(BuildSettingsIsDebugBuildVariant::parse(rdr, ver)?);
        }
            m_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 52) {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 1) {
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 46) {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 15) {
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 43) {
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            runtime_class_hashes = Some(BuildSettingsRuntimeClassHashesVariant::parse(rdr, ver)?);
            script_hashes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Hash128::parse(rdr, ver)?; let b = Hash128::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_graphics_ap_is = Some(BuildSettingsMGraphicsApIsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            runtime_class_hashes = Some(BuildSettingsRuntimeClassHashesVariant::parse(rdr, ver)?);
            script_hashes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Hash128::parse(rdr, ver)?; let b = Hash128::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            runtime_class_hashes = Some(BuildSettingsRuntimeClassHashesVariant::parse(rdr, ver)?);
        } else {
            m_auth_token = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        }

        Ok(BuildSettings {
            levels,
            scenes,
            has_render_texture,
            preloaded_plugins,
            has_pro_version: has_pro_version.unwrap(),
            enabled_vr_devices,
            is_educational_build: is_educational_build.unwrap(),
            is_no_watermark_build,
            build_tags,
            has_publishing_rights: has_publishing_rights.unwrap(),
            is_prototyping_build,
            build_guid,
            has_shadows: has_shadows.unwrap(),
            has_advanced_version: has_advanced_version.unwrap(),
            is_embedded,
            enable_dynamic_batching: enable_dynamic_batching.unwrap(),
            is_debug_build,
            m_version: m_version.unwrap(),
            has_soft_shadows,
            is_trial,
            m_auth_token,
            has_local_light_shadows,
            uses_on_mouse_events,
            is_wsa_holographic_remoting_enabled,
            runtime_class_hashes,
            has_oculus_plugin,
            has_cluster_rendering,
            enable_multiple_displays,
            script_hashes,
            m_graphics_ap_is,
        })
    }
}
