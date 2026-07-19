//! 自动生成。请勿手动修改。

use crate::subclasses::config_setting::ConfigSetting;

/// 版本枚举：EditorUserSettingsMVcDebugOutVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserSettingsMVcDebugOutVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for EditorUserSettingsMVcDebugOutVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorUserSettingsMSemanticMergeModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserSettingsMSemanticMergeModeVariant {
    Default(i32),
    V5_1(i32),
}

impl crate::Parse for EditorUserSettingsMSemanticMergeModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_1(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorUserSettingsMVcShowFailedCheckoutVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserSettingsMVcShowFailedCheckoutVariant {
    Default(bool),
    V5_6_6(bool),
    V2017_1(bool),
    V2017_1_3(bool),
    V2017_2(bool),
    V2017_2_1(bool),
}

impl crate::Parse for EditorUserSettingsMVcShowFailedCheckoutVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 1) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2_1(v))
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_2(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 3) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_1_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 6) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6_6(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorUserSettingsMVcAllowAsyncUpdateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserSettingsMVcAllowAsyncUpdateVariant {
    Default(bool),
    V2021_2(bool),
}

impl crate::Parse for EditorUserSettingsMVcAllowAsyncUpdateVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：EditorUserSettingsMArtifactGarbageCollectionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserSettingsMArtifactGarbageCollectionVariant {
    Default(bool),
    V2023_2(bool),
}

impl crate::Parse for EditorUserSettingsMArtifactGarbageCollectionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
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
    pub m_vc_debug_out: EditorUserSettingsMVcDebugOutVariant,
    pub m_semantic_merge_mode: Option<EditorUserSettingsMSemanticMergeModeVariant>,
    pub m_vc_show_failed_checkout: Option<EditorUserSettingsMVcShowFailedCheckoutVariant>,
    pub m_desired_import_worker_count: Option<i32>,
    pub m_vc_allow_async_update: Option<EditorUserSettingsMVcAllowAsyncUpdateVariant>,
    pub m_vc_overwrite_failed_checkout_assets: Option<bool>,
    pub m_standby_import_worker_count: Option<i32>,
    pub m_vc_overlay_icons: Option<bool>,
    pub m_vc_project_overlay_icons: Option<bool>,
    pub m_idle_import_worker_shutdown_delay: Option<i32>,
    pub m_asset_pipeline_mode: Option<i32>,
    pub m_vc_hierarchy_overlay_icons: Option<bool>,
    pub m_cache_server_mode: Option<i32>,
    pub m_vc_other_overlay_icons: Option<bool>,
    pub m_cache_servers: Option<Vec<String>>,
    pub m_artifact_garbage_collection: Option<EditorUserSettingsMArtifactGarbageCollectionVariant>,
    pub m_vc_scan_local_packages_on_connect: Option<bool>,
    pub m_compress_assets_on_import: Option<bool>,
}

impl crate::Parse for EditorUserSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_vc_user_name = None;
        let mut m_config_values = None;
        let mut m_config_settings = None;
        let mut m_vc_password = None;
        let mut m_vc_automatic_add = None;
        let mut m_vc_server = None;
        let mut m_vc_debug_com = None;
        let mut m_vc_workspace = None;
        let mut m_vc_debug_cmd = None;
        let mut m_vc_debug_out = None;
        let mut m_semantic_merge_mode = None;
        let mut m_vc_show_failed_checkout = None;
        let mut m_desired_import_worker_count = None;
        let mut m_vc_allow_async_update = None;
        let mut m_vc_overwrite_failed_checkout_assets = None;
        let mut m_standby_import_worker_count = None;
        let mut m_vc_overlay_icons = None;
        let mut m_vc_project_overlay_icons = None;
        let mut m_idle_import_worker_shutdown_delay = None;
        let mut m_asset_pipeline_mode = None;
        let mut m_vc_hierarchy_overlay_icons = None;
        let mut m_cache_server_mode = None;
        let mut m_vc_other_overlay_icons = None;
        let mut m_cache_servers = None;
        let mut m_artifact_garbage_collection = None;
        let mut m_vc_scan_local_packages_on_connect = None;
        let mut m_compress_assets_on_import = None;
        if *ver >= crate::UnityVersion::new(5, 4, 2) {
            m_config_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = ConfigSetting::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_config_values = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
            m_vc_user_name = Some(rdr.read_aligned_string()?);
            m_vc_password = Some(rdr.read_aligned_string()?);
            m_vc_server = Some(rdr.read_aligned_string()?);
            m_vc_workspace = Some(rdr.read_aligned_string()?);
        }
        m_vc_automatic_add = Some(rdr.read_bool()?);
        m_vc_debug_com = Some(rdr.read_bool()?);
        m_vc_debug_cmd = Some(rdr.read_bool()?);
        m_vc_debug_out = Some(EditorUserSettingsMVcDebugOutVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 0, 7) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_desired_import_worker_count = Some(rdr.read_i32()?);
            m_standby_import_worker_count = Some(rdr.read_i32()?);
            m_idle_import_worker_shutdown_delay = Some(rdr.read_i32()?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_project_overlay_icons = Some(rdr.read_bool()?);
            m_vc_hierarchy_overlay_icons = Some(rdr.read_bool()?);
            m_vc_other_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
            m_vc_scan_local_packages_on_connect = Some(rdr.read_bool()?);
            m_artifact_garbage_collection = Some(
                EditorUserSettingsMArtifactGarbageCollectionVariant::parse(rdr, ver)?,
            );
            m_compress_assets_on_import = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_desired_import_worker_count = Some(rdr.read_i32()?);
            m_standby_import_worker_count = Some(rdr.read_i32()?);
            m_idle_import_worker_shutdown_delay = Some(rdr.read_i32()?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_project_overlay_icons = Some(rdr.read_bool()?);
            m_vc_hierarchy_overlay_icons = Some(rdr.read_bool()?);
            m_vc_other_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
            m_artifact_garbage_collection = Some(
                EditorUserSettingsMArtifactGarbageCollectionVariant::parse(rdr, ver)?,
            );
            m_compress_assets_on_import = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_desired_import_worker_count = Some(rdr.read_i32()?);
            m_standby_import_worker_count = Some(rdr.read_i32()?);
            m_idle_import_worker_shutdown_delay = Some(rdr.read_i32()?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_project_overlay_icons = Some(rdr.read_bool()?);
            m_vc_hierarchy_overlay_icons = Some(rdr.read_bool()?);
            m_vc_other_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
            m_artifact_garbage_collection = Some(
                EditorUserSettingsMArtifactGarbageCollectionVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 3, 35) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_desired_import_worker_count = Some(rdr.read_i32()?);
            m_standby_import_worker_count = Some(rdr.read_i32()?);
            m_idle_import_worker_shutdown_delay = Some(rdr.read_i32()?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_project_overlay_icons = Some(rdr.read_bool()?);
            m_vc_hierarchy_overlay_icons = Some(rdr.read_bool()?);
            m_vc_other_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
            m_vc_scan_local_packages_on_connect = Some(rdr.read_bool()?);
            m_artifact_garbage_collection = Some(
                EditorUserSettingsMArtifactGarbageCollectionVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_desired_import_worker_count = Some(rdr.read_i32()?);
            m_standby_import_worker_count = Some(rdr.read_i32()?);
            m_idle_import_worker_shutdown_delay = Some(rdr.read_i32()?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_project_overlay_icons = Some(rdr.read_bool()?);
            m_vc_hierarchy_overlay_icons = Some(rdr.read_bool()?);
            m_vc_other_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
            m_artifact_garbage_collection = Some(
                EditorUserSettingsMArtifactGarbageCollectionVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 3, 40) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_desired_import_worker_count = Some(rdr.read_i32()?);
            m_standby_import_worker_count = Some(rdr.read_i32()?);
            m_idle_import_worker_shutdown_delay = Some(rdr.read_i32()?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_project_overlay_icons = Some(rdr.read_bool()?);
            m_vc_hierarchy_overlay_icons = Some(rdr.read_bool()?);
            m_vc_other_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
            m_vc_scan_local_packages_on_connect = Some(rdr.read_bool()?);
            m_artifact_garbage_collection = Some(
                EditorUserSettingsMArtifactGarbageCollectionVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_desired_import_worker_count = Some(rdr.read_i32()?);
            m_standby_import_worker_count = Some(rdr.read_i32()?);
            m_idle_import_worker_shutdown_delay = Some(rdr.read_i32()?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_project_overlay_icons = Some(rdr.read_bool()?);
            m_vc_hierarchy_overlay_icons = Some(rdr.read_bool()?);
            m_vc_other_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
            m_artifact_garbage_collection = Some(
                EditorUserSettingsMArtifactGarbageCollectionVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_project_overlay_icons = Some(rdr.read_bool()?);
            m_vc_hierarchy_overlay_icons = Some(rdr.read_bool()?);
            m_vc_other_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_overlay_icons = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
            m_asset_pipeline_mode = Some(rdr.read_i32()?);
            m_cache_server_mode = Some(rdr.read_i32()?);
            m_cache_servers = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 11) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 5) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_overwrite_failed_checkout_assets = Some(rdr.read_bool()?);
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 1) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 1, 3) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 6, 6) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
            m_vc_allow_async_update = Some(EditorUserSettingsMVcAllowAsyncUpdateVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
            m_vc_show_failed_checkout = Some(
                EditorUserSettingsMVcShowFailedCheckoutVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_semantic_merge_mode = Some(EditorUserSettingsMSemanticMergeModeVariant::parse(
                rdr, ver,
            )?);
        } else {
        }

        Ok(EditorUserSettings {
            m_vc_user_name,
            m_config_values,
            m_config_settings,
            m_vc_password,
            m_vc_automatic_add: m_vc_automatic_add.unwrap(),
            m_vc_server,
            m_vc_debug_com: m_vc_debug_com.unwrap(),
            m_vc_workspace,
            m_vc_debug_cmd: m_vc_debug_cmd.unwrap(),
            m_vc_debug_out: m_vc_debug_out.unwrap(),
            m_semantic_merge_mode,
            m_vc_show_failed_checkout,
            m_desired_import_worker_count,
            m_vc_allow_async_update,
            m_vc_overwrite_failed_checkout_assets,
            m_standby_import_worker_count,
            m_vc_overlay_icons,
            m_vc_project_overlay_icons,
            m_idle_import_worker_shutdown_delay,
            m_asset_pipeline_mode,
            m_vc_hierarchy_overlay_icons,
            m_cache_server_mode,
            m_vc_other_overlay_icons,
            m_cache_servers,
            m_artifact_garbage_collection,
            m_vc_scan_local_packages_on_connect,
            m_compress_assets_on_import,
        })
    }
}
