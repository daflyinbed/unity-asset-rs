//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;
use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct BuildSettings {
    pub parent: GlobalGameManager,
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
    pub build_guid: Option<String>,
    pub has_shadows: bool,
    pub has_advanced_version: bool,
    pub is_embedded: Option<bool>,
    pub enable_dynamic_batching: bool,
    pub is_debug_build: Option<bool>,
    pub m_version: String,
    pub has_soft_shadows: Option<bool>,
    pub is_trial: Option<bool>,
    pub m_auth_token: Option<String>,
    pub has_local_light_shadows: Option<bool>,
    pub uses_on_mouse_events: Option<bool>,
    pub is_wsa_holographic_remoting_enabled: Option<bool>,
    pub runtime_class_hashes: Option<Vec<(i32, u32)>>,
    pub has_oculus_plugin: Option<bool>,
    pub has_cluster_rendering: Option<bool>,
    pub enable_multiple_displays: Option<bool>,
    pub script_hashes: Option<Vec<(Hash128, Hash128)>>,
    pub m_graphics_ap_is: Option<Vec<i32>>,
}

impl crate::Parse for BuildSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let levels = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let scenes = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let has_render_texture = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let preloaded_plugins = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let has_pro_version = r.read_bool()?;
        let enabled_vr_devices = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let is_educational_build = r.read_bool()?;
        let is_no_watermark_build = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let build_tags = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let has_publishing_rights = r.read_bool()?;
        let is_prototyping_build = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let build_guid = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let has_shadows = r.read_bool()?;
        let has_advanced_version = r.read_bool()?;
        let is_embedded = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let enable_dynamic_batching = r.read_bool()?;
        let is_debug_build = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_version = r.read_aligned_string()?;
        r.align(4)?;
        let has_soft_shadows = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let is_trial = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_auth_token = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let has_local_light_shadows = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let uses_on_mouse_events = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let is_wsa_holographic_remoting_enabled = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let runtime_class_hashes = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_u32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let has_oculus_plugin = if *ver >= crate::UnityVersion::new(4, 6, 2) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let has_cluster_rendering = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let enable_multiple_displays = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let script_hashes = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Hash128::parse(r, ver)?; let b = Hash128::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_graphics_ap_is = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };

        Ok(BuildSettings {
            parent,
            levels,
            scenes,
            has_render_texture,
            preloaded_plugins,
            has_pro_version,
            enabled_vr_devices,
            is_educational_build,
            is_no_watermark_build,
            build_tags,
            has_publishing_rights,
            is_prototyping_build,
            build_guid,
            has_shadows,
            has_advanced_version,
            is_embedded,
            enable_dynamic_batching,
            is_debug_build,
            m_version,
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
