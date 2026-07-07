//! 自动生成。请勿手动修改。

use crate::subclasses::config_setting::ConfigSetting;

#[derive(Debug, Clone)]
pub struct EditorUserSettings {
    pub m_vc_user_name: Option<String>,
    pub m_config_values: Option<Vec<(String, String)>>,
    pub m_config_settings: Option<Vec<(String, ConfigSetting)>>,
    pub m_vc_password: Option<String>,
    pub m_vc_automatic_add: bool,
    pub m_vc_server: Option<String>,
    pub m_vc_debug_com: bool,
    pub m_vc_workspace: Option<String>,
    pub m_vc_debug_cmd: bool,
    pub m_vc_debug_out: bool,
    pub m_semantic_merge_mode: Option<i32>,
    pub m_vc_show_failed_checkout: Option<bool>,
    pub m_desired_import_worker_count: Option<i32>,
    pub m_vc_allow_async_update: Option<bool>,
    pub m_vc_overwrite_failed_checkout_assets: Option<bool>,
    pub m_standby_import_worker_count: Option<i32>,
    pub m_asset_pipeline_mode: Option<i32>,
    pub m_asset_pipeline_mode2: Option<i32>,
    pub m_vc_overlay_icons: Option<bool>,
    pub m_vc_project_overlay_icons: Option<bool>,
    pub m_idle_import_worker_shutdown_delay: Option<i32>,
    pub m_cache_server_mode: Option<i32>,
    pub m_vc_hierarchy_overlay_icons: Option<bool>,
    pub m_cache_servers: Option<Vec<String>>,
    pub m_vc_other_overlay_icons: Option<bool>,
    pub m_artifact_garbage_collection: Option<bool>,
    pub m_vc_scan_local_packages_on_connect: Option<bool>,
    pub m_compress_assets_on_import: Option<bool>,
}

impl crate::Parse for EditorUserSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_vc_user_name = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_config_values = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_config_settings = if *ver >= crate::UnityVersion::new(5, 4, 2) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = ConfigSetting::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_vc_password = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_vc_automatic_add = r.read_bool()?;
        let m_vc_server = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_vc_debug_com = r.read_bool()?;
        let m_vc_workspace = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_vc_debug_cmd = r.read_bool()?;
        let m_vc_debug_out = r.read_bool()?;
        let m_semantic_merge_mode = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_vc_show_failed_checkout = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_desired_import_worker_count = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_vc_allow_async_update = if *ver >= crate::UnityVersion::new(5, 6, 6) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_vc_overwrite_failed_checkout_assets = if *ver >= crate::UnityVersion::new(2018, 4, 5) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_standby_import_worker_count = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_asset_pipeline_mode = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_asset_pipeline_mode2 = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_vc_overlay_icons = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_vc_project_overlay_icons = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_idle_import_worker_shutdown_delay = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_cache_server_mode = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_vc_hierarchy_overlay_icons = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_cache_servers = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_vc_other_overlay_icons = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_artifact_garbage_collection = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_vc_scan_local_packages_on_connect = if *ver >= crate::UnityVersion::new(2021, 3, 40) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_compress_assets_on_import = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(EditorUserSettings {
            m_vc_user_name,
            m_config_values,
            m_config_settings,
            m_vc_password,
            m_vc_automatic_add,
            m_vc_server,
            m_vc_debug_com,
            m_vc_workspace,
            m_vc_debug_cmd,
            m_vc_debug_out,
            m_semantic_merge_mode,
            m_vc_show_failed_checkout,
            m_desired_import_worker_count,
            m_vc_allow_async_update,
            m_vc_overwrite_failed_checkout_assets,
            m_standby_import_worker_count,
            m_asset_pipeline_mode,
            m_asset_pipeline_mode2,
            m_vc_overlay_icons,
            m_vc_project_overlay_icons,
            m_idle_import_worker_shutdown_delay,
            m_cache_server_mode,
            m_vc_hierarchy_overlay_icons,
            m_cache_servers,
            m_vc_other_overlay_icons,
            m_artifact_garbage_collection,
            m_vc_scan_local_packages_on_connect,
            m_compress_assets_on_import,
        })
    }
}
