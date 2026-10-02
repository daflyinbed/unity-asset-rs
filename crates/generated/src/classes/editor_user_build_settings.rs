//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::platform_settings_data_editor::PlatformSettingsData_Editor;

/// 版本枚举：EditorUserBuildSettingsMBuildLocationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMBuildLocationVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for EditorUserBuildSettingsMBuildLocationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMBuildLocationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMSymlinkLibrariesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMSymlinkLibrariesVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for EditorUserBuildSettingsMSymlinkLibrariesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMSymlinkLibrariesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant {
    Default(i32),
    V4_5(i32),
}

impl crate::Parse for EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant {
    Default(i32),
    V2019_1(i32),
    V2023_1_10(i32),
}

impl crate::Parse for EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 10) {
            let v = rdr.read_i32()?;
            Ok(Self::V2023_1_10(v))
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
            Self::V2023_1_10(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1_10(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMWiiUEnableNetApiVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMWiiUEnableNetApiVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for EditorUserBuildSettingsMWiiUEnableNetApiVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMWiiUEnableNetApiVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant {
    Default(bool),
    V5_6(bool),
}

impl crate::Parse for EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant {
    Default(bool),
    V5_6_2(bool),
}

impl crate::Parse for EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 2) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMNvnShaderDebuggingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMNvnShaderDebuggingVariant {
    Default(bool),
    V6000_0_61(bool),
    V6000_1(bool),
    V6000_4(bool),
}

impl crate::Parse for EditorUserBuildSettingsMNvnShaderDebuggingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_4(v))
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_1(v))
        } else if *ver >= crate::UnityVersion::new(6000, 0, 61) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_0_61(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMNvnShaderDebuggingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_0_61(v) => v.to_value(),
            Self::V6000_1(v) => v.to_value(),
            Self::V6000_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_0_61(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_1(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant {
    Default(bool),
    V5_4(bool),
    V5_6(bool),
}

impl crate::Parse for EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6(v))
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant {
    Default(bool),
    V2017_4_17(bool),
    V2018_1(bool),
    V2018_3(bool),
}

impl crate::Parse for EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3(v))
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2018_1(v))
        } else if *ver >= crate::UnityVersion::new(2017, 4, 17) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_4_17(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_4_17(v) => v.to_value(),
            Self::V2018_1(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_4_17(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMBuildAppBundleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMBuildAppBundleVariant {
    Default(bool),
    V2018_4_13(bool),
    V2019_1(bool),
    V2019_2_11(bool),
    V2021_1(bool),
}

impl crate::Parse for EditorUserBuildSettingsMBuildAppBundleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 2, 11) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_2_11(v))
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_4_13(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMBuildAppBundleVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_13(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
            Self::V2019_2_11(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_13(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_2_11(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant {
    Default(bool),
    V2019_4_23(bool),
    V2020_1(bool),
}

impl crate::Parse for EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 4, 23) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_4_23(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_4_23(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_4_23(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EditorUserBuildSettingsMOverrideTextureCompressionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EditorUserBuildSettingsMOverrideTextureCompressionVariant {
    Default(i32),
    V2022_1(i32),
}

impl crate::Parse for EditorUserBuildSettingsMOverrideTextureCompressionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2022_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EditorUserBuildSettingsMOverrideTextureCompressionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EditorUserBuildSettings {
    pub m_build_location: EditorUserBuildSettingsMBuildLocationVariant,
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
    pub m_symlink_libraries: Option<EditorUserBuildSettingsMSymlinkLibrariesVariant>,
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
    pub m_selected_black_berry_build_type:
        Option<EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant>,
    pub m_web_gl_optimization_level: Option<i32>,
    pub m_wsa_holographic_remoting: Option<bool>,
    pub m_selected_tizen_subtarget: Option<i32>,
    pub m_selected_wsasdk: Option<i32>,
    pub m_compress_files_in_package: Option<bool>,
    pub m_build_with_code_coverage: Option<bool>,
    pub m_selected_wsa_build_and_run_deploy_target:
        Option<EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant>,
    pub m_explicit_divide_by_zero_checks: Option<bool>,
    pub m_symlink_sources: Option<bool>,
    pub m_generate_wsa_reference_projects: Option<bool>,
    pub m_wii_u_enable_net_api: Option<EditorUserBuildSettingsMWiiUEnableNetApiVariant>,
    pub m_move_package_to_disc_outer_edge: Option<bool>,
    pub m_force_installation: Option<bool>,
    pub m_export_as_google_android_project:
        Option<EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant>,
    pub m_xbox_one_streaming_install_launch_chunk_range: Option<i32>,
    pub m_selected_wsauwp_build_type: Option<i32>,
    pub m_selected_xbox_one_deploy_method: Option<i32>,
    pub m_selected_wsa_subtarget: Option<i32>,
    pub m_explicit_array_bounds_checks: Option<bool>,
    pub m_wsa_dot_net_native_enabled: Option<Vec<bool>>,
    pub m_selected_wsauwpsdk: Option<String>,
    pub m_create_solution_file_for_switch: Option<bool>,
    pub m_create_rom_file_for_switch: Option<bool>,
    pub m_nvn_graphics_debugger_for_switch:
        Option<EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant>,
    pub m_enable_rom_compression_for_switch: Option<bool>,
    pub m_nvn_shader_debugging: Option<EditorUserBuildSettingsMNvnShaderDebuggingVariant>,
    pub m_generate_nintendo_switch_shader_info: Option<bool>,
    pub m_save_adf_for_switch: Option<bool>,
    pub m_force_optimize_script_compilation:
        Option<EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant>,
    pub m_xbox_one_username: Option<String>,
    pub m_enable_debug_pad_for_switch: Option<bool>,
    pub m_rom_compression_type_for_switch: Option<i32>,
    pub m_nvn_draw_validation_light: Option<bool>,
    pub m_xbox_one_network_share_path: Option<String>,
    pub m_redirect_writes_to_host_mount_for_switch:
        Option<EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant>,
    pub m_nvn_draw_validation: Option<bool>,
    pub m_rom_compression_level_for_switch: Option<i32>,
    pub m_nvn_draw_validation_heavy: Option<bool>,
    pub m_enable_heap_inspector_for_switch: Option<bool>,
    pub m_rom_compression_config_for_switch: Option<String>,
    pub m_platform_settings: Option<Vec<(String, PlatformSettingsData_Editor)>>,
    pub m_build_app_bundle: Option<EditorUserBuildSettingsMBuildAppBundleVariant>,
    pub m_use_legacy_nvn_pool_allocator_for_switch: Option<bool>,
    pub m_htcs_script_debugging_for_switch: Option<bool>,
    pub m_android_create_symbols_zip:
        Option<EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant>,
    pub m_ps5_keep_package_files: Option<bool>,
    pub m_selected_wsauwpvs_version: Option<String>,
    pub m_android_use_legacy_sdk_tools: Option<bool>,
    pub m_selected_ps5_subtarget: Option<i32>,
    pub m_android_create_symbols: Option<i32>,
    pub m_nvn_aftermath: Option<bool>,
    pub m_selected_ps5_compression_type: Option<i32>,
    pub m_enable_memory_tracker_for_switch: Option<bool>,
    pub m_nvn_aftermath_level: Option<i32>,
    pub m_selected_ps5_compression_level: Option<i32>,
    pub m_wait_for_switch_memory_tracker_on_startup: Option<bool>,
    pub m_selected_wsa_min_uwpsdk: Option<String>,
    pub m_ps5_workspace_name: Option<String>,
    pub m_override_max_texture_size: Option<i32>,
    pub m_selected_wsa_architecture: Option<String>,
    pub m_override_texture_compression:
        Option<EditorUserBuildSettingsMOverrideTextureCompressionVariant>,
    pub m_il2_cpp_code_generation: Option<i32>,
    pub m_facebook_create_package_for_submission: Option<bool>,
    pub m_enable_host_io_for_switch: Option<bool>,
    pub m_windows_device_portal_address: Option<String>,
    pub m_selected_xbox_one_deploy_drive: Option<i32>,
    pub m_windows_device_portal_username: Option<String>,
    pub m_selected_web_gl_subtarget: Option<i32>,
    pub m_enable_unpublishable_errors_for_switch: Option<bool>,
    pub m_web_gl_client_browser_type: Option<i32>,
    pub m_web_gl_client_browser_path: Option<String>,
    pub m_web_gl_client_platform: Option<i32>,
    pub m_macos_xcode_build_config: Option<i32>,
    pub m_selected_windows_build_and_run_deploy_target: Option<i32>,
    pub m_selected_embedded_linux_architecture: Option<i32>,
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
        rdr: &mut R,
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
        let mut m_wsa_holographic_remoting = None;
        let mut m_selected_tizen_subtarget = None;
        let mut m_selected_wsasdk = None;
        let mut m_compress_files_in_package = None;
        let mut m_build_with_code_coverage = None;
        let mut m_selected_wsa_build_and_run_deploy_target = None;
        let mut m_explicit_divide_by_zero_checks = None;
        let mut m_symlink_sources = None;
        let mut m_generate_wsa_reference_projects = None;
        let mut m_wii_u_enable_net_api = None;
        let mut m_move_package_to_disc_outer_edge = None;
        let mut m_force_installation = None;
        let mut m_export_as_google_android_project = None;
        let mut m_xbox_one_streaming_install_launch_chunk_range = None;
        let mut m_selected_wsauwp_build_type = None;
        let mut m_selected_xbox_one_deploy_method = None;
        let mut m_selected_wsa_subtarget = None;
        let mut m_explicit_array_bounds_checks = None;
        let mut m_wsa_dot_net_native_enabled = None;
        let mut m_selected_wsauwpsdk = None;
        let mut m_create_solution_file_for_switch = None;
        let mut m_create_rom_file_for_switch = None;
        let mut m_nvn_graphics_debugger_for_switch = None;
        let mut m_enable_rom_compression_for_switch = None;
        let mut m_nvn_shader_debugging = None;
        let mut m_generate_nintendo_switch_shader_info = None;
        let mut m_save_adf_for_switch = None;
        let mut m_force_optimize_script_compilation = None;
        let mut m_xbox_one_username = None;
        let mut m_enable_debug_pad_for_switch = None;
        let mut m_rom_compression_type_for_switch = None;
        let mut m_nvn_draw_validation_light = None;
        let mut m_xbox_one_network_share_path = None;
        let mut m_redirect_writes_to_host_mount_for_switch = None;
        let mut m_nvn_draw_validation = None;
        let mut m_rom_compression_level_for_switch = None;
        let mut m_nvn_draw_validation_heavy = None;
        let mut m_enable_heap_inspector_for_switch = None;
        let mut m_rom_compression_config_for_switch = None;
        let mut m_platform_settings = None;
        let mut m_build_app_bundle = None;
        let mut m_use_legacy_nvn_pool_allocator_for_switch = None;
        let mut m_htcs_script_debugging_for_switch = None;
        let mut m_android_create_symbols_zip = None;
        let mut m_ps5_keep_package_files = None;
        let mut m_selected_wsauwpvs_version = None;
        let mut m_android_use_legacy_sdk_tools = None;
        let mut m_selected_ps5_subtarget = None;
        let mut m_android_create_symbols = None;
        let mut m_nvn_aftermath = None;
        let mut m_selected_ps5_compression_type = None;
        let mut m_enable_memory_tracker_for_switch = None;
        let mut m_nvn_aftermath_level = None;
        let mut m_selected_ps5_compression_level = None;
        let mut m_wait_for_switch_memory_tracker_on_startup = None;
        let mut m_selected_wsa_min_uwpsdk = None;
        let mut m_ps5_workspace_name = None;
        let mut m_override_max_texture_size = None;
        let mut m_selected_wsa_architecture = None;
        let mut m_override_texture_compression = None;
        let mut m_il2_cpp_code_generation = None;
        let mut m_facebook_create_package_for_submission = None;
        let mut m_enable_host_io_for_switch = None;
        let mut m_windows_device_portal_address = None;
        let mut m_selected_xbox_one_deploy_drive = None;
        let mut m_windows_device_portal_username = None;
        let mut m_selected_web_gl_subtarget = None;
        let mut m_enable_unpublishable_errors_for_switch = None;
        let mut m_web_gl_client_browser_type = None;
        let mut m_web_gl_client_browser_path = None;
        let mut m_web_gl_client_platform = None;
        let mut m_macos_xcode_build_config = None;
        let mut m_selected_windows_build_and_run_deploy_target = None;
        let mut m_selected_embedded_linux_architecture = None;
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
        m_build_location = Some(EditorUserBuildSettingsMBuildLocationVariant::parse(
            rdr, ver,
        )?);
        m_active_build_target = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 0, 4) {
            m_active_platform_guid = Some(Guid::parse(rdr, ver)?);
            m_selected_build_target = Some(rdr.read_i32()?);
            m_active_build_target_group = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_selected_build_target = Some(rdr.read_i32()?);
            m_active_build_target_group = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_active_build_target_group = Some(rdr.read_i32()?);
        } else {
        }
        m_selected_build_target_group = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_active_build_platform_group_name = Some(rdr.read_aligned_string()?);
            m_selected_build_platform_group_name = Some(rdr.read_aligned_string()?);
        } else {
        }
        m_selected_standalone_target = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_selected_standalone_build_subtarget = Some(rdr.read_i32()?);
            m_active_standalone_build_subtarget = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_selected_facebook_target = Some(rdr.read_i32()?);
        } else {
        }
        m_architecture_flags = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_android_device_socket_address = Some(rdr.read_aligned_string()?);
            m_android_current_deployment_target_id = Some(rdr.read_aligned_string()?);
            m_selected_compression_type = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_selected_android_etc2_fallback = Some(rdr.read_i32()?);
            m_android_device_socket_address = Some(rdr.read_aligned_string()?);
            m_android_current_deployment_target_id = Some(rdr.read_aligned_string()?);
            m_selected_compression_type = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 3, 70) {
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_selected_android_etc2_fallback = Some(rdr.read_i32()?);
            m_android_device_socket_address = Some(rdr.read_aligned_string()?);
            m_android_current_deployment_target_id = Some(rdr.read_aligned_string()?);
            m_selected_compression_type = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_diagnostic_setting = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_selected_android_etc2_fallback = Some(rdr.read_i32()?);
            m_android_device_socket_address = Some(rdr.read_aligned_string()?);
            m_android_current_deployment_target_id = Some(rdr.read_aligned_string()?);
            m_selected_compression_type = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_selected_android_etc2_fallback = Some(rdr.read_i32()?);
            m_android_device_socket_address = Some(rdr.read_aligned_string()?);
            m_android_current_deployment_target_id = Some(rdr.read_aligned_string()?);
            m_selected_compression_type = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_facebook_access_token = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_selected_android_etc2_fallback = Some(rdr.read_i32()?);
            m_android_device_socket_address = Some(rdr.read_aligned_string()?);
            m_android_current_deployment_target_id = Some(rdr.read_aligned_string()?);
            m_selected_compression_type = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_facebook_access_token = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_selected_android_etc2_fallback = Some(rdr.read_i32()?);
            m_android_device_socket_address = Some(rdr.read_aligned_string()?);
            m_selected_compression_type = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_facebook_access_token = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_android_device_socket_address = Some(rdr.read_aligned_string()?);
            m_selected_compression_type = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_facebook_access_token = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_android_release_minification = Some(rdr.read_i32()?);
            m_android_debug_minification = Some(rdr.read_i32()?);
            m_facebook_access_token = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
            m_android_build_type = Some(rdr.read_i32()?);
            m_facebook_access_token = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_android_build_system = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 2) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_run_method = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_selected_ps3_subtarget = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_run_method = Some(rdr.read_i32()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 7) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_ps4_hardware_target = Some(rdr.read_i32()?);
            m_selected_ps3_subtarget = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_run_method = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_selected_ps3_subtarget = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_u_debug_level = Some(rdr.read_i32()?);
            m_selected_wii_u_build_output = Some(rdr.read_i32()?);
            m_selected_wii_u_boot_mode = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_run_method = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps4_subtarget = Some(rdr.read_i32()?);
            m_selected_ps3_subtarget = Some(rdr.read_i32()?);
            m_selected_psm_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_run_method = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_selected_psp2_subtarget = Some(rdr.read_i32()?);
            m_selected_ps3_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_run_method = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_selected_wii_subtarget = Some(rdr.read_i32()?);
            m_selected_ps3_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_debug_level = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_run_method = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_selected_wii_subtarget = Some(rdr.read_i32()?);
            m_selected_wii_debug_level = Some(rdr.read_i32()?);
            m_selected_xbox_subtarget = Some(rdr.read_i32()?);
            m_selected_xbox_run_method = Some(rdr.read_i32()?);
        } else {
            m_selected_wii_subtarget = Some(rdr.read_i32()?);
        }
        m_development = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_dataless_player = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_dataless_player = Some(rdr.read_bool()?);
            m_web_gl_use_pre_built_unity_engine = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_web_gl_use_pre_built_unity_engine = Some(rdr.read_bool()?);
        } else {
        }
        m_connect_profiler = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_build_with_deep_profiling_support = Some(rdr.read_bool()?);
            m_build_with_code_coverage = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_build_with_deep_profiling_support = Some(rdr.read_bool()?);
        } else {
        }
        m_allow_debugging = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_wait_for_player_connection = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_wsa_holographic_remoting = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_web_player_streamed = Some(rdr.read_bool()?);
            m_web_player_offline_deployment = Some(rdr.read_bool()?);
        } else {
            m_web_player_streamed = Some(rdr.read_bool()?);
            m_web_player_offline_deployment = Some(rdr.read_bool()?);
            m_web_player_na_cl_support = Some(rdr.read_bool()?);
        }
        m_install_in_build_folder = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_aftermath_level = Some(rdr.read_i32()?);
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_enable_host_io_for_switch = Some(rdr.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_aftermath = Some(rdr.read_bool()?);
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_enable_host_io_for_switch = Some(rdr.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 61) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_aftermath_level = Some(rdr.read_i32()?);
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_enable_host_io_for_switch = Some(rdr.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_aftermath = Some(rdr.read_bool()?);
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_enable_host_io_for_switch = Some(rdr.read_bool()?);
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 4) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 57) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_aftermath = Some(rdr.read_bool()?);
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_enable_unpublishable_errors_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 49) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_aftermath = Some(rdr.read_bool()?);
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 5) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_memory_tracker_for_switch = Some(rdr.read_bool()?);
            m_wait_for_switch_memory_tracker_on_startup = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_il2_cpp_code_generation = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 17) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_il2_cpp_code_generation = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_symlink_sources = Some(rdr.read_bool()?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_override_max_texture_size = Some(rdr.read_i32()?);
            m_override_texture_compression =
                Some(EditorUserBuildSettingsMOverrideTextureCompressionVariant::parse(rdr, ver)?);
            m_il2_cpp_code_generation = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 5) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 31) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 7) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 4) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation_light = Some(rdr.read_bool()?);
            m_nvn_draw_validation_heavy = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_htcs_script_debugging_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 38) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_enable_rom_compression_for_switch = Some(rdr.read_bool()?);
            m_save_adf_for_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_rom_compression_type_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_level_for_switch = Some(rdr.read_i32()?);
            m_rom_compression_config_for_switch = Some(rdr.read_aligned_string()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
            m_ps5_keep_package_files = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_ps5_subtarget = Some(rdr.read_i32()?);
            m_selected_ps5_compression_type = Some(rdr.read_i32()?);
            m_selected_ps5_compression_level = Some(rdr.read_i32()?);
            m_ps5_workspace_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 25) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
            m_ps5_keep_package_files = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_ps5_subtarget = Some(rdr.read_i32()?);
            m_selected_ps5_compression_type = Some(rdr.read_i32()?);
            m_selected_ps5_compression_level = Some(rdr.read_i32()?);
            m_ps5_workspace_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 24) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_use_legacy_nvn_pool_allocator_for_switch = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
            m_ps5_keep_package_files = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_ps5_subtarget = Some(rdr.read_i32()?);
            m_selected_ps5_compression_type = Some(rdr.read_i32()?);
            m_selected_ps5_compression_level = Some(rdr.read_i32()?);
            m_ps5_workspace_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 23) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
            m_ps5_keep_package_files = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_ps5_subtarget = Some(rdr.read_i32()?);
            m_selected_ps5_compression_type = Some(rdr.read_i32()?);
            m_selected_ps5_compression_level = Some(rdr.read_i32()?);
            m_ps5_workspace_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 11) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 35) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_generate_nintendo_switch_shader_info = Some(rdr.read_bool()?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_android_use_legacy_sdk_tools = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_android_use_legacy_sdk_tools = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
            m_android_create_symbols_zip =
                Some(EditorUserBuildSettingsMAndroidCreateSymbolsZipVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_nvn_shader_debugging = Some(
                EditorUserBuildSettingsMNvnShaderDebuggingVariant::parse(rdr, ver)?,
            );
            m_nvn_draw_validation = Some(rdr.read_bool()?);
            m_enable_heap_inspector_for_switch = Some(rdr.read_bool()?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_android_use_legacy_sdk_tools = Some(rdr.read_bool()?);
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_move_package_to_disc_outer_edge = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_explicit_array_bounds_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 4, 17) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_wii_u_enable_net_api = Some(EditorUserBuildSettingsMWiiUEnableNetApiVariant::parse(
                rdr, ver,
            )?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
            m_build_app_bundle = Some(EditorUserBuildSettingsMBuildAppBundleVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 3) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_force_installation = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_wii_u_enable_net_api = Some(EditorUserBuildSettingsMWiiUEnableNetApiVariant::parse(
                rdr, ver,
            )?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 6, 2) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_wii_u_enable_net_api = Some(EditorUserBuildSettingsMWiiUEnableNetApiVariant::parse(
                rdr, ver,
            )?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
            m_enable_debug_pad_for_switch = Some(rdr.read_bool()?);
            m_redirect_writes_to_host_mount_for_switch = Some(
                EditorUserBuildSettingsMRedirectWritesToHostMountForSwitchVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_wii_u_enable_net_api = Some(EditorUserBuildSettingsMWiiUEnableNetApiVariant::parse(
                rdr, ver,
            )?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
            m_create_solution_file_for_switch = Some(rdr.read_bool()?);
            m_create_rom_file_for_switch = Some(rdr.read_bool()?);
            m_nvn_graphics_debugger_for_switch =
                Some(EditorUserBuildSettingsMNvnGraphicsDebuggerForSwitchVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 1) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_compress_files_in_package = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_wii_u_enable_net_api = Some(EditorUserBuildSettingsMWiiUEnableNetApiVariant::parse(
                rdr, ver,
            )?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_wii_u_enable_net_api = Some(EditorUserBuildSettingsMWiiUEnableNetApiVariant::parse(
                rdr, ver,
            )?);
            m_export_as_google_android_project =
                Some(EditorUserBuildSettingsMExportAsGoogleAndroidProjectVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_explicit_divide_by_zero_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_wii_u_enable_net_api = Some(EditorUserBuildSettingsMWiiUEnableNetApiVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_wii_u_enable_net_api = Some(EditorUserBuildSettingsMWiiUEnableNetApiVariant::parse(
                rdr, ver,
            )?);
            m_web_gl_optimization_level = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_symlink_trampoline = Some(rdr.read_bool()?);
            m_selected_ios_build_type = Some(rdr.read_i32()?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_web_player_deploy_online = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_web_gl_optimization_level = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_need_submission_materials = Some(rdr.read_bool()?);
            m_compress_with_ps_arc = Some(rdr.read_bool()?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_build_scripts_only = Some(rdr.read_bool()?);
            m_web_player_deploy_online = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_web_gl_optimization_level = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_enable_headless_mode = Some(rdr.read_bool()?);
            m_web_player_deploy_online = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_web_player_deploy_online = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
            m_explicit_null_checks = Some(rdr.read_bool()?);
            m_xbox_compressed_xex = Some(rdr.read_bool()?);
            m_web_player_deploy_online = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_symlink_libraries = Some(EditorUserBuildSettingsMSymlinkLibrariesVariant::parse(
                rdr, ver,
            )?);
        }
        m_selected_android_subtarget = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_web_gl_client_platform = Some(rdr.read_i32()?);
            m_web_gl_client_browser_type = Some(rdr.read_i32()?);
            m_web_gl_client_browser_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_selected_windows_build_and_run_deploy_target = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_selected_qnx_os_version = Some(rdr.read_i32()?);
            m_selected_qnx_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
            m_active_build_profile = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_web_gl_client_platform = Some(rdr.read_i32()?);
            m_web_gl_client_browser_type = Some(rdr.read_i32()?);
            m_web_gl_client_browser_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_selected_windows_build_and_run_deploy_target = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_selected_qnx_os_version = Some(rdr.read_i32()?);
            m_selected_qnx_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
            m_active_build_profile = Some(crate::PPtr::parse(rdr, ver)?);
            m_build_output_to_build_metadata_map = Some(
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
        } else if *ver >= crate::UnityVersion::new(6000, 0, 20) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_web_gl_client_platform = Some(rdr.read_i32()?);
            m_web_gl_client_browser_type = Some(rdr.read_i32()?);
            m_web_gl_client_browser_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_selected_windows_build_and_run_deploy_target = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_selected_qnx_os_version = Some(rdr.read_i32()?);
            m_selected_qnx_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
            m_active_build_profile = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_web_gl_client_platform = Some(rdr.read_i32()?);
            m_web_gl_client_browser_type = Some(rdr.read_i32()?);
            m_web_gl_client_browser_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_selected_windows_build_and_run_deploy_target = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_selected_qnx_os_version = Some(rdr.read_i32()?);
            m_selected_qnx_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
            m_active_profile_path = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_web_gl_client_platform = Some(rdr.read_i32()?);
            m_web_gl_client_browser_type = Some(rdr.read_i32()?);
            m_web_gl_client_browser_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_selected_windows_build_and_run_deploy_target = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_selected_qnx_os_version = Some(rdr.read_i32()?);
            m_selected_qnx_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_web_gl_client_browser_type = Some(rdr.read_i32()?);
            m_web_gl_client_browser_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_selected_windows_build_and_run_deploy_target = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_selected_qnx_os_version = Some(rdr.read_i32()?);
            m_selected_qnx_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 10) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_selected_windows_build_and_run_deploy_target = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_selected_qnx_os_version = Some(rdr.read_i32()?);
            m_selected_qnx_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_selected_qnx_os_version = Some(rdr.read_i32()?);
            m_selected_qnx_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 14) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
            m_remote_device_info = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_remote_device_address = Some(rdr.read_aligned_string()?);
            m_remote_device_username = Some(rdr.read_aligned_string()?);
            m_remote_device_exports = Some(rdr.read_aligned_string()?);
            m_path_on_remote_device = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_selected_web_gl_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_macos_xcode_build_config = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_selected_embedded_linux_architecture = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_windows_device_portal_address = Some(rdr.read_aligned_string()?);
            m_windows_device_portal_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_min_uwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_architecture = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_drive = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsauwpvs_version = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_facebook_create_package_for_submission = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsasdk = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_force_optimize_script_compilation = Some(
                EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant::parse(rdr, ver)?,
            );
            m_facebook_create_package_for_submission = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 5, 2) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsasdk = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsauwpsdk = Some(rdr.read_aligned_string()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_force_optimize_script_compilation = Some(
                EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant::parse(rdr, ver)?,
            );
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_selected_wsa_subtarget = Some(rdr.read_i32()?);
            m_selected_wsasdk = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_force_optimize_script_compilation = Some(
                EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant::parse(rdr, ver)?,
            );
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_selected_wsasdk = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_force_optimize_script_compilation = Some(
                EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant::parse(rdr, ver)?,
            );
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_selected_wsasdk = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_wsa_dot_net_native_enabled = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_bool())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_selected_black_berry_subtarget = Some(rdr.read_i32()?);
            m_selected_black_berry_build_type =
                Some(EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant::parse(rdr, ver)?);
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_xbox_one_username = Some(rdr.read_aligned_string()?);
            m_xbox_one_network_share_path = Some(rdr.read_aligned_string()?);
            m_force_optimize_script_compilation = Some(
                EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 2, 2) {
            m_selected_wsasdk = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_black_berry_subtarget = Some(rdr.read_i32()?);
            m_selected_black_berry_build_type =
                Some(EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant::parse(rdr, ver)?);
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
            m_force_optimize_script_compilation = Some(
                EditorUserBuildSettingsMForceOptimizeScriptCompilationVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_selected_wsasdk = Some(rdr.read_i32()?);
            m_selected_wsauwp_build_type = Some(rdr.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_black_berry_subtarget = Some(rdr.read_i32()?);
            m_selected_black_berry_build_type =
                Some(EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant::parse(rdr, ver)?);
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_selected_wsasdk = Some(rdr.read_i32()?);
            m_selected_wsa_build_and_run_deploy_target = Some(
                EditorUserBuildSettingsMSelectedWsaBuildAndRunDeployTargetVariant::parse(rdr, ver)?,
            );
            m_generate_wsa_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_black_berry_subtarget = Some(rdr.read_i32()?);
            m_selected_black_berry_build_type =
                Some(EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant::parse(rdr, ver)?);
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_xbox_one_streaming_install_launch_chunk_range = Some(rdr.read_i32()?);
            m_selected_xbox_one_deploy_method = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 3) {
            m_selected_metro_target = Some(rdr.read_i32()?);
            m_selected_metro_build_type = Some(rdr.read_i32()?);
            m_selected_metro_sdk = Some(rdr.read_i32()?);
            m_selected_metro_build_and_run_deploy_target = Some(rdr.read_i32()?);
            m_generate_metro_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_black_berry_subtarget = Some(rdr.read_i32()?);
            m_selected_black_berry_build_type =
                Some(EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant::parse(rdr, ver)?);
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_selected_metro_target = Some(rdr.read_i32()?);
            m_selected_metro_build_type = Some(rdr.read_i32()?);
            m_selected_metro_sdk = Some(rdr.read_i32()?);
            m_generate_metro_reference_projects = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_black_berry_subtarget = Some(rdr.read_i32()?);
            m_selected_black_berry_build_type =
                Some(EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant::parse(rdr, ver)?);
            m_selected_tizen_subtarget = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_selected_metro_target = Some(rdr.read_i32()?);
            m_selected_metro_build_type = Some(rdr.read_i32()?);
            m_selected_metro_sdk = Some(rdr.read_i32()?);
            m_selected_black_berry_subtarget = Some(rdr.read_i32()?);
            m_selected_black_berry_build_type =
                Some(EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_selected_metro_target = Some(rdr.read_i32()?);
            m_selected_metro_build_type = Some(rdr.read_i32()?);
            m_selected_black_berry_subtarget = Some(rdr.read_i32()?);
            m_selected_black_berry_build_type =
                Some(EditorUserBuildSettingsMSelectedBlackBerryBuildTypeVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_selected_metro_build_type = Some(rdr.read_i32()?);
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
            m_wsa_holographic_remoting,
            m_selected_tizen_subtarget,
            m_selected_wsasdk,
            m_compress_files_in_package,
            m_build_with_code_coverage,
            m_selected_wsa_build_and_run_deploy_target,
            m_explicit_divide_by_zero_checks,
            m_symlink_sources,
            m_generate_wsa_reference_projects,
            m_wii_u_enable_net_api,
            m_move_package_to_disc_outer_edge,
            m_force_installation,
            m_export_as_google_android_project,
            m_xbox_one_streaming_install_launch_chunk_range,
            m_selected_wsauwp_build_type,
            m_selected_xbox_one_deploy_method,
            m_selected_wsa_subtarget,
            m_explicit_array_bounds_checks,
            m_wsa_dot_net_native_enabled,
            m_selected_wsauwpsdk,
            m_create_solution_file_for_switch,
            m_create_rom_file_for_switch,
            m_nvn_graphics_debugger_for_switch,
            m_enable_rom_compression_for_switch,
            m_nvn_shader_debugging,
            m_generate_nintendo_switch_shader_info,
            m_save_adf_for_switch,
            m_force_optimize_script_compilation,
            m_xbox_one_username,
            m_enable_debug_pad_for_switch,
            m_rom_compression_type_for_switch,
            m_nvn_draw_validation_light,
            m_xbox_one_network_share_path,
            m_redirect_writes_to_host_mount_for_switch,
            m_nvn_draw_validation,
            m_rom_compression_level_for_switch,
            m_nvn_draw_validation_heavy,
            m_enable_heap_inspector_for_switch,
            m_rom_compression_config_for_switch,
            m_platform_settings,
            m_build_app_bundle,
            m_use_legacy_nvn_pool_allocator_for_switch,
            m_htcs_script_debugging_for_switch,
            m_android_create_symbols_zip,
            m_ps5_keep_package_files,
            m_selected_wsauwpvs_version,
            m_android_use_legacy_sdk_tools,
            m_selected_ps5_subtarget,
            m_android_create_symbols,
            m_nvn_aftermath,
            m_selected_ps5_compression_type,
            m_enable_memory_tracker_for_switch,
            m_nvn_aftermath_level,
            m_selected_ps5_compression_level,
            m_wait_for_switch_memory_tracker_on_startup,
            m_selected_wsa_min_uwpsdk,
            m_ps5_workspace_name,
            m_override_max_texture_size,
            m_selected_wsa_architecture,
            m_override_texture_compression,
            m_il2_cpp_code_generation,
            m_facebook_create_package_for_submission,
            m_enable_host_io_for_switch,
            m_windows_device_portal_address,
            m_selected_xbox_one_deploy_drive,
            m_windows_device_portal_username,
            m_selected_web_gl_subtarget,
            m_enable_unpublishable_errors_for_switch,
            m_web_gl_client_browser_type,
            m_web_gl_client_browser_path,
            m_web_gl_client_platform,
            m_macos_xcode_build_config,
            m_selected_windows_build_and_run_deploy_target,
            m_selected_embedded_linux_architecture,
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

impl crate::ToValue for EditorUserBuildSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_BuildLocation".to_string(),
                self.m_build_location.to_value(),
            )),
            Some((
                "m_ActiveBuildTarget".to_string(),
                self.m_active_build_target.to_value(),
            )),
            Some((
                "m_SelectedBuildTargetGroup".to_string(),
                self.m_selected_build_target_group.to_value(),
            )),
            crate::opt_field(
                "m_ActiveBuildTargetGroup",
                &self.m_active_build_target_group,
            ),
            crate::opt_field("m_SelectedBuildTarget", &self.m_selected_build_target),
            crate::opt_field("m_ActivePlatformGuid", &self.m_active_platform_guid),
            Some((
                "m_SelectedStandaloneTarget".to_string(),
                self.m_selected_standalone_target.to_value(),
            )),
            Some((
                "m_ArchitectureFlags".to_string(),
                self.m_architecture_flags.to_value(),
            )),
            crate::opt_field(
                "m_ActiveBuildPlatformGroupName",
                &self.m_active_build_platform_group_name,
            ),
            crate::opt_field("m_SelectedWiiSubtarget", &self.m_selected_wii_subtarget),
            crate::opt_field("m_SelectedPSP2Subtarget", &self.m_selected_psp2_subtarget),
            crate::opt_field("m_SelectedFacebookTarget", &self.m_selected_facebook_target),
            crate::opt_field(
                "m_SelectedBuildPlatformGroupName",
                &self.m_selected_build_platform_group_name,
            ),
            Some(("m_Development".to_string(), self.m_development.to_value())),
            crate::opt_field("m_SelectedWiiDebugLevel", &self.m_selected_wii_debug_level),
            crate::opt_field("m_SelectedPS3Subtarget", &self.m_selected_ps3_subtarget),
            crate::opt_field("m_SelectedPS4Subtarget", &self.m_selected_ps4_subtarget),
            Some((
                "m_ConnectProfiler".to_string(),
                self.m_connect_profiler.to_value(),
            )),
            crate::opt_field("m_SelectedXboxSubtarget", &self.m_selected_xbox_subtarget),
            crate::opt_field("m_PS4HardwareTarget", &self.m_ps4_hardware_target),
            crate::opt_field(
                "m_SelectedStandaloneBuildSubtarget",
                &self.m_selected_standalone_build_subtarget,
            ),
            Some((
                "m_AllowDebugging".to_string(),
                self.m_allow_debugging.to_value(),
            )),
            crate::opt_field("m_SelectedXboxRunMethod", &self.m_selected_xbox_run_method),
            crate::opt_field("m_SelectedPSMSubtarget", &self.m_selected_psm_subtarget),
            crate::opt_field(
                "m_ActiveStandaloneBuildSubtarget",
                &self.m_active_standalone_build_subtarget,
            ),
            crate::opt_field("m_WebPlayerStreamed", &self.m_web_player_streamed),
            crate::opt_field("m_SelectedIOSBuildType", &self.m_selected_ios_build_type),
            crate::opt_field(
                "m_SelectedWiiUDebugLevel",
                &self.m_selected_wii_u_debug_level,
            ),
            crate::opt_field(
                "m_WebPlayerOfflineDeployment",
                &self.m_web_player_offline_deployment,
            ),
            crate::opt_field(
                "m_SelectedWiiUBuildOutput",
                &self.m_selected_wii_u_build_output,
            ),
            crate::opt_field("m_AndroidBuildSystem", &self.m_android_build_system),
            crate::opt_field("m_WebPlayerNaClSupport", &self.m_web_player_na_cl_support),
            crate::opt_field("m_SelectedWiiUBootMode", &self.m_selected_wii_u_boot_mode),
            crate::opt_field("m_AndroidBuildType", &self.m_android_build_type),
            Some((
                "m_InstallInBuildFolder".to_string(),
                self.m_install_in_build_folder.to_value(),
            )),
            crate::opt_field(
                "m_AndroidReleaseMinification",
                &self.m_android_release_minification,
            ),
            crate::opt_field("m_SymlinkLibraries", &self.m_symlink_libraries),
            crate::opt_field(
                "m_AndroidDebugMinification",
                &self.m_android_debug_minification,
            ),
            Some((
                "m_SelectedAndroidSubtarget".to_string(),
                self.m_selected_android_subtarget.to_value(),
            )),
            crate::opt_field(
                "m_SelectedAndroidETC2Fallback",
                &self.m_selected_android_etc2_fallback,
            ),
            crate::opt_field(
                "m_AndroidDeviceSocketAddress",
                &self.m_android_device_socket_address,
            ),
            crate::opt_field("m_ExplicitNullChecks", &self.m_explicit_null_checks),
            crate::opt_field(
                "m_WebGLUsePreBuiltUnityEngine",
                &self.m_web_gl_use_pre_built_unity_engine,
            ),
            crate::opt_field(
                "m_AndroidCurrentDeploymentTargetId",
                &self.m_android_current_deployment_target_id,
            ),
            crate::opt_field("m_EnableHeadlessMode", &self.m_enable_headless_mode),
            crate::opt_field(
                "m_SelectedCompressionType",
                &self.m_selected_compression_type,
            ),
            crate::opt_field("m_XboxCompressedXex", &self.m_xbox_compressed_xex),
            crate::opt_field("m_WebPlayerDeployOnline", &self.m_web_player_deploy_online),
            crate::opt_field(
                "m_NeedSubmissionMaterials",
                &self.m_need_submission_materials,
            ),
            crate::opt_field("m_SymlinkTrampoline", &self.m_symlink_trampoline),
            crate::opt_field("m_FacebookAccessToken", &self.m_facebook_access_token),
            crate::opt_field("m_CompressWithPsArc", &self.m_compress_with_ps_arc),
            crate::opt_field(
                "m_SelectedMetroBuildType",
                &self.m_selected_metro_build_type,
            ),
            crate::opt_field("m_SelectedMetroTarget", &self.m_selected_metro_target),
            crate::opt_field(
                "m_BuildWithDeepProfilingSupport",
                &self.m_build_with_deep_profiling_support,
            ),
            crate::opt_field("m_DatalessPlayer", &self.m_dataless_player),
            crate::opt_field("m_SelectedMetroSDK", &self.m_selected_metro_sdk),
            crate::opt_field("m_BuildScriptsOnly", &self.m_build_scripts_only),
            crate::opt_field(
                "m_WaitForPlayerConnection",
                &self.m_wait_for_player_connection,
            ),
            crate::opt_field(
                "m_SelectedDiagnosticSetting",
                &self.m_selected_diagnostic_setting,
            ),
            crate::opt_field(
                "m_SelectedBlackBerrySubtarget",
                &self.m_selected_black_berry_subtarget,
            ),
            crate::opt_field(
                "m_GenerateMetroReferenceProjects",
                &self.m_generate_metro_reference_projects,
            ),
            crate::opt_field(
                "m_SelectedMetroBuildAndRunDeployTarget",
                &self.m_selected_metro_build_and_run_deploy_target,
            ),
            crate::opt_field(
                "m_SelectedBlackBerryBuildType",
                &self.m_selected_black_berry_build_type,
            ),
            crate::opt_field(
                "m_WebGLOptimizationLevel",
                &self.m_web_gl_optimization_level,
            ),
            crate::opt_field("m_WsaHolographicRemoting", &self.m_wsa_holographic_remoting),
            crate::opt_field("m_SelectedTizenSubtarget", &self.m_selected_tizen_subtarget),
            crate::opt_field("m_SelectedWSASDK", &self.m_selected_wsasdk),
            crate::opt_field(
                "m_CompressFilesInPackage",
                &self.m_compress_files_in_package,
            ),
            crate::opt_field("m_BuildWithCodeCoverage", &self.m_build_with_code_coverage),
            crate::opt_field(
                "m_SelectedWSABuildAndRunDeployTarget",
                &self.m_selected_wsa_build_and_run_deploy_target,
            ),
            crate::opt_field(
                "m_ExplicitDivideByZeroChecks",
                &self.m_explicit_divide_by_zero_checks,
            ),
            crate::opt_field("m_SymlinkSources", &self.m_symlink_sources),
            crate::opt_field(
                "m_GenerateWSAReferenceProjects",
                &self.m_generate_wsa_reference_projects,
            ),
            crate::opt_field("m_WiiUEnableNetAPI", &self.m_wii_u_enable_net_api),
            crate::opt_field(
                "m_MovePackageToDiscOuterEdge",
                &self.m_move_package_to_disc_outer_edge,
            ),
            crate::opt_field("m_ForceInstallation", &self.m_force_installation),
            crate::opt_field(
                "m_ExportAsGoogleAndroidProject",
                &self.m_export_as_google_android_project,
            ),
            crate::opt_field(
                "m_XboxOneStreamingInstallLaunchChunkRange",
                &self.m_xbox_one_streaming_install_launch_chunk_range,
            ),
            crate::opt_field(
                "m_SelectedWSAUWPBuildType",
                &self.m_selected_wsauwp_build_type,
            ),
            crate::opt_field(
                "m_SelectedXboxOneDeployMethod",
                &self.m_selected_xbox_one_deploy_method,
            ),
            crate::opt_field("m_SelectedWSASubtarget", &self.m_selected_wsa_subtarget),
            crate::opt_field(
                "m_ExplicitArrayBoundsChecks",
                &self.m_explicit_array_bounds_checks,
            ),
            crate::opt_field(
                "m_WSADotNetNativeEnabled",
                &self.m_wsa_dot_net_native_enabled,
            ),
            crate::opt_field("m_SelectedWSAUWPSDK", &self.m_selected_wsauwpsdk),
            crate::opt_field(
                "m_CreateSolutionFileForSwitch",
                &self.m_create_solution_file_for_switch,
            ),
            crate::opt_field(
                "m_CreateRomFileForSwitch",
                &self.m_create_rom_file_for_switch,
            ),
            crate::opt_field(
                "m_NVNGraphicsDebuggerForSwitch",
                &self.m_nvn_graphics_debugger_for_switch,
            ),
            crate::opt_field(
                "m_EnableRomCompressionForSwitch",
                &self.m_enable_rom_compression_for_switch,
            ),
            crate::opt_field("m_NVNShaderDebugging", &self.m_nvn_shader_debugging),
            crate::opt_field(
                "m_GenerateNintendoSwitchShaderInfo",
                &self.m_generate_nintendo_switch_shader_info,
            ),
            crate::opt_field("m_SaveADFForSwitch", &self.m_save_adf_for_switch),
            crate::opt_field(
                "m_ForceOptimizeScriptCompilation",
                &self.m_force_optimize_script_compilation,
            ),
            crate::opt_field("m_XboxOneUsername", &self.m_xbox_one_username),
            crate::opt_field(
                "m_EnableDebugPadForSwitch",
                &self.m_enable_debug_pad_for_switch,
            ),
            crate::opt_field(
                "m_RomCompressionTypeForSwitch",
                &self.m_rom_compression_type_for_switch,
            ),
            crate::opt_field(
                "m_NVNDrawValidationLight",
                &self.m_nvn_draw_validation_light,
            ),
            crate::opt_field(
                "m_XboxOneNetworkSharePath",
                &self.m_xbox_one_network_share_path,
            ),
            crate::opt_field(
                "m_RedirectWritesToHostMountForSwitch",
                &self.m_redirect_writes_to_host_mount_for_switch,
            ),
            crate::opt_field("m_NVNDrawValidation", &self.m_nvn_draw_validation),
            crate::opt_field(
                "m_RomCompressionLevelForSwitch",
                &self.m_rom_compression_level_for_switch,
            ),
            crate::opt_field(
                "m_NVNDrawValidationHeavy",
                &self.m_nvn_draw_validation_heavy,
            ),
            crate::opt_field(
                "m_EnableHeapInspectorForSwitch",
                &self.m_enable_heap_inspector_for_switch,
            ),
            crate::opt_field(
                "m_RomCompressionConfigForSwitch",
                &self.m_rom_compression_config_for_switch,
            ),
            crate::opt_field("m_PlatformSettings", &self.m_platform_settings),
            crate::opt_field("m_BuildAppBundle", &self.m_build_app_bundle),
            crate::opt_field(
                "m_UseLegacyNvnPoolAllocatorForSwitch",
                &self.m_use_legacy_nvn_pool_allocator_for_switch,
            ),
            crate::opt_field(
                "m_HTCSScriptDebuggingForSwitch",
                &self.m_htcs_script_debugging_for_switch,
            ),
            crate::opt_field(
                "m_AndroidCreateSymbolsZip",
                &self.m_android_create_symbols_zip,
            ),
            crate::opt_field("m_PS5KeepPackageFiles", &self.m_ps5_keep_package_files),
            crate::opt_field(
                "m_SelectedWSAUWPVSVersion",
                &self.m_selected_wsauwpvs_version,
            ),
            crate::opt_field(
                "m_AndroidUseLegacySdkTools",
                &self.m_android_use_legacy_sdk_tools,
            ),
            crate::opt_field("m_SelectedPS5Subtarget", &self.m_selected_ps5_subtarget),
            crate::opt_field("m_AndroidCreateSymbols", &self.m_android_create_symbols),
            crate::opt_field("m_NVNAftermath", &self.m_nvn_aftermath),
            crate::opt_field(
                "m_SelectedPS5CompressionType",
                &self.m_selected_ps5_compression_type,
            ),
            crate::opt_field(
                "m_EnableMemoryTrackerForSwitch",
                &self.m_enable_memory_tracker_for_switch,
            ),
            crate::opt_field("m_NVNAftermathLevel", &self.m_nvn_aftermath_level),
            crate::opt_field(
                "m_SelectedPS5CompressionLevel",
                &self.m_selected_ps5_compression_level,
            ),
            crate::opt_field(
                "m_WaitForSwitchMemoryTrackerOnStartup",
                &self.m_wait_for_switch_memory_tracker_on_startup,
            ),
            crate::opt_field("m_SelectedWSAMinUWPSDK", &self.m_selected_wsa_min_uwpsdk),
            crate::opt_field("m_PS5WorkspaceName", &self.m_ps5_workspace_name),
            crate::opt_field(
                "m_OverrideMaxTextureSize",
                &self.m_override_max_texture_size,
            ),
            crate::opt_field(
                "m_SelectedWSAArchitecture",
                &self.m_selected_wsa_architecture,
            ),
            crate::opt_field(
                "m_OverrideTextureCompression",
                &self.m_override_texture_compression,
            ),
            crate::opt_field("m_Il2CppCodeGeneration", &self.m_il2_cpp_code_generation),
            crate::opt_field(
                "m_FacebookCreatePackageForSubmission",
                &self.m_facebook_create_package_for_submission,
            ),
            crate::opt_field("m_EnableHostIOForSwitch", &self.m_enable_host_io_for_switch),
            crate::opt_field(
                "m_WindowsDevicePortalAddress",
                &self.m_windows_device_portal_address,
            ),
            crate::opt_field(
                "m_SelectedXboxOneDeployDrive",
                &self.m_selected_xbox_one_deploy_drive,
            ),
            crate::opt_field(
                "m_WindowsDevicePortalUsername",
                &self.m_windows_device_portal_username,
            ),
            crate::opt_field(
                "m_SelectedWebGLSubtarget",
                &self.m_selected_web_gl_subtarget,
            ),
            crate::opt_field(
                "m_EnableUnpublishableErrorsForSwitch",
                &self.m_enable_unpublishable_errors_for_switch,
            ),
            crate::opt_field(
                "m_WebGLClientBrowserType",
                &self.m_web_gl_client_browser_type,
            ),
            crate::opt_field(
                "m_WebGLClientBrowserPath",
                &self.m_web_gl_client_browser_path,
            ),
            crate::opt_field("m_WebGLClientPlatform", &self.m_web_gl_client_platform),
            crate::opt_field("m_macosXcodeBuildConfig", &self.m_macos_xcode_build_config),
            crate::opt_field(
                "m_SelectedWindowsBuildAndRunDeployTarget",
                &self.m_selected_windows_build_and_run_deploy_target,
            ),
            crate::opt_field(
                "m_SelectedEmbeddedLinuxArchitecture",
                &self.m_selected_embedded_linux_architecture,
            ),
            crate::opt_field("m_RemoteDeviceInfo", &self.m_remote_device_info),
            crate::opt_field("m_RemoteDeviceAddress", &self.m_remote_device_address),
            crate::opt_field("m_RemoteDeviceUsername", &self.m_remote_device_username),
            crate::opt_field("m_RemoteDeviceExports", &self.m_remote_device_exports),
            crate::opt_field("m_PathOnRemoteDevice", &self.m_path_on_remote_device),
            crate::opt_field("m_SelectedQNXOsVersion", &self.m_selected_qnx_os_version),
            crate::opt_field(
                "m_SelectedQNXArchitecture",
                &self.m_selected_qnx_architecture,
            ),
            crate::opt_field("m_ActiveProfilePath", &self.m_active_profile_path),
            crate::opt_field("m_ActiveBuildProfile", &self.m_active_build_profile),
            crate::opt_field(
                "m_BuildOutputToBuildMetadataMap",
                &self.m_build_output_to_build_metadata_map,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BuildLocation" => (self.m_build_location).inspect_value(path, offset, limit),
                "m_ActiveBuildTarget" => {
                    (self.m_active_build_target).inspect_value(path, offset, limit)
                }
                "m_SelectedBuildTargetGroup" => {
                    (self.m_selected_build_target_group).inspect_value(path, offset, limit)
                }
                "m_ActiveBuildTargetGroup" => {
                    (self.m_active_build_target_group).inspect_value(path, offset, limit)
                }
                "m_SelectedBuildTarget" => {
                    (self.m_selected_build_target).inspect_value(path, offset, limit)
                }
                "m_ActivePlatformGuid" => {
                    (self.m_active_platform_guid).inspect_value(path, offset, limit)
                }
                "m_SelectedStandaloneTarget" => {
                    (self.m_selected_standalone_target).inspect_value(path, offset, limit)
                }
                "m_ArchitectureFlags" => {
                    (self.m_architecture_flags).inspect_value(path, offset, limit)
                }
                "m_ActiveBuildPlatformGroupName" => {
                    (self.m_active_build_platform_group_name).inspect_value(path, offset, limit)
                }
                "m_SelectedWiiSubtarget" => {
                    (self.m_selected_wii_subtarget).inspect_value(path, offset, limit)
                }
                "m_SelectedPSP2Subtarget" => {
                    (self.m_selected_psp2_subtarget).inspect_value(path, offset, limit)
                }
                "m_SelectedFacebookTarget" => {
                    (self.m_selected_facebook_target).inspect_value(path, offset, limit)
                }
                "m_SelectedBuildPlatformGroupName" => {
                    (self.m_selected_build_platform_group_name).inspect_value(path, offset, limit)
                }
                "m_Development" => (self.m_development).inspect_value(path, offset, limit),
                "m_SelectedWiiDebugLevel" => {
                    (self.m_selected_wii_debug_level).inspect_value(path, offset, limit)
                }
                "m_SelectedPS3Subtarget" => {
                    (self.m_selected_ps3_subtarget).inspect_value(path, offset, limit)
                }
                "m_SelectedPS4Subtarget" => {
                    (self.m_selected_ps4_subtarget).inspect_value(path, offset, limit)
                }
                "m_ConnectProfiler" => (self.m_connect_profiler).inspect_value(path, offset, limit),
                "m_SelectedXboxSubtarget" => {
                    (self.m_selected_xbox_subtarget).inspect_value(path, offset, limit)
                }
                "m_PS4HardwareTarget" => {
                    (self.m_ps4_hardware_target).inspect_value(path, offset, limit)
                }
                "m_SelectedStandaloneBuildSubtarget" => {
                    (self.m_selected_standalone_build_subtarget).inspect_value(path, offset, limit)
                }
                "m_AllowDebugging" => (self.m_allow_debugging).inspect_value(path, offset, limit),
                "m_SelectedXboxRunMethod" => {
                    (self.m_selected_xbox_run_method).inspect_value(path, offset, limit)
                }
                "m_SelectedPSMSubtarget" => {
                    (self.m_selected_psm_subtarget).inspect_value(path, offset, limit)
                }
                "m_ActiveStandaloneBuildSubtarget" => {
                    (self.m_active_standalone_build_subtarget).inspect_value(path, offset, limit)
                }
                "m_WebPlayerStreamed" => {
                    (self.m_web_player_streamed).inspect_value(path, offset, limit)
                }
                "m_SelectedIOSBuildType" => {
                    (self.m_selected_ios_build_type).inspect_value(path, offset, limit)
                }
                "m_SelectedWiiUDebugLevel" => {
                    (self.m_selected_wii_u_debug_level).inspect_value(path, offset, limit)
                }
                "m_WebPlayerOfflineDeployment" => {
                    (self.m_web_player_offline_deployment).inspect_value(path, offset, limit)
                }
                "m_SelectedWiiUBuildOutput" => {
                    (self.m_selected_wii_u_build_output).inspect_value(path, offset, limit)
                }
                "m_AndroidBuildSystem" => {
                    (self.m_android_build_system).inspect_value(path, offset, limit)
                }
                "m_WebPlayerNaClSupport" => {
                    (self.m_web_player_na_cl_support).inspect_value(path, offset, limit)
                }
                "m_SelectedWiiUBootMode" => {
                    (self.m_selected_wii_u_boot_mode).inspect_value(path, offset, limit)
                }
                "m_AndroidBuildType" => {
                    (self.m_android_build_type).inspect_value(path, offset, limit)
                }
                "m_InstallInBuildFolder" => {
                    (self.m_install_in_build_folder).inspect_value(path, offset, limit)
                }
                "m_AndroidReleaseMinification" => {
                    (self.m_android_release_minification).inspect_value(path, offset, limit)
                }
                "m_SymlinkLibraries" => {
                    (self.m_symlink_libraries).inspect_value(path, offset, limit)
                }
                "m_AndroidDebugMinification" => {
                    (self.m_android_debug_minification).inspect_value(path, offset, limit)
                }
                "m_SelectedAndroidSubtarget" => {
                    (self.m_selected_android_subtarget).inspect_value(path, offset, limit)
                }
                "m_SelectedAndroidETC2Fallback" => {
                    (self.m_selected_android_etc2_fallback).inspect_value(path, offset, limit)
                }
                "m_AndroidDeviceSocketAddress" => {
                    (self.m_android_device_socket_address).inspect_value(path, offset, limit)
                }
                "m_ExplicitNullChecks" => {
                    (self.m_explicit_null_checks).inspect_value(path, offset, limit)
                }
                "m_WebGLUsePreBuiltUnityEngine" => {
                    (self.m_web_gl_use_pre_built_unity_engine).inspect_value(path, offset, limit)
                }
                "m_AndroidCurrentDeploymentTargetId" => {
                    (self.m_android_current_deployment_target_id).inspect_value(path, offset, limit)
                }
                "m_EnableHeadlessMode" => {
                    (self.m_enable_headless_mode).inspect_value(path, offset, limit)
                }
                "m_SelectedCompressionType" => {
                    (self.m_selected_compression_type).inspect_value(path, offset, limit)
                }
                "m_XboxCompressedXex" => {
                    (self.m_xbox_compressed_xex).inspect_value(path, offset, limit)
                }
                "m_WebPlayerDeployOnline" => {
                    (self.m_web_player_deploy_online).inspect_value(path, offset, limit)
                }
                "m_NeedSubmissionMaterials" => {
                    (self.m_need_submission_materials).inspect_value(path, offset, limit)
                }
                "m_SymlinkTrampoline" => {
                    (self.m_symlink_trampoline).inspect_value(path, offset, limit)
                }
                "m_FacebookAccessToken" => {
                    (self.m_facebook_access_token).inspect_value(path, offset, limit)
                }
                "m_CompressWithPsArc" => {
                    (self.m_compress_with_ps_arc).inspect_value(path, offset, limit)
                }
                "m_SelectedMetroBuildType" => {
                    (self.m_selected_metro_build_type).inspect_value(path, offset, limit)
                }
                "m_SelectedMetroTarget" => {
                    (self.m_selected_metro_target).inspect_value(path, offset, limit)
                }
                "m_BuildWithDeepProfilingSupport" => {
                    (self.m_build_with_deep_profiling_support).inspect_value(path, offset, limit)
                }
                "m_DatalessPlayer" => (self.m_dataless_player).inspect_value(path, offset, limit),
                "m_SelectedMetroSDK" => {
                    (self.m_selected_metro_sdk).inspect_value(path, offset, limit)
                }
                "m_BuildScriptsOnly" => {
                    (self.m_build_scripts_only).inspect_value(path, offset, limit)
                }
                "m_WaitForPlayerConnection" => {
                    (self.m_wait_for_player_connection).inspect_value(path, offset, limit)
                }
                "m_SelectedDiagnosticSetting" => {
                    (self.m_selected_diagnostic_setting).inspect_value(path, offset, limit)
                }
                "m_SelectedBlackBerrySubtarget" => {
                    (self.m_selected_black_berry_subtarget).inspect_value(path, offset, limit)
                }
                "m_GenerateMetroReferenceProjects" => {
                    (self.m_generate_metro_reference_projects).inspect_value(path, offset, limit)
                }
                "m_SelectedMetroBuildAndRunDeployTarget" => (self
                    .m_selected_metro_build_and_run_deploy_target)
                    .inspect_value(path, offset, limit),
                "m_SelectedBlackBerryBuildType" => {
                    (self.m_selected_black_berry_build_type).inspect_value(path, offset, limit)
                }
                "m_WebGLOptimizationLevel" => {
                    (self.m_web_gl_optimization_level).inspect_value(path, offset, limit)
                }
                "m_WsaHolographicRemoting" => {
                    (self.m_wsa_holographic_remoting).inspect_value(path, offset, limit)
                }
                "m_SelectedTizenSubtarget" => {
                    (self.m_selected_tizen_subtarget).inspect_value(path, offset, limit)
                }
                "m_SelectedWSASDK" => (self.m_selected_wsasdk).inspect_value(path, offset, limit),
                "m_CompressFilesInPackage" => {
                    (self.m_compress_files_in_package).inspect_value(path, offset, limit)
                }
                "m_BuildWithCodeCoverage" => {
                    (self.m_build_with_code_coverage).inspect_value(path, offset, limit)
                }
                "m_SelectedWSABuildAndRunDeployTarget" => (self
                    .m_selected_wsa_build_and_run_deploy_target)
                    .inspect_value(path, offset, limit),
                "m_ExplicitDivideByZeroChecks" => {
                    (self.m_explicit_divide_by_zero_checks).inspect_value(path, offset, limit)
                }
                "m_SymlinkSources" => (self.m_symlink_sources).inspect_value(path, offset, limit),
                "m_GenerateWSAReferenceProjects" => {
                    (self.m_generate_wsa_reference_projects).inspect_value(path, offset, limit)
                }
                "m_WiiUEnableNetAPI" => {
                    (self.m_wii_u_enable_net_api).inspect_value(path, offset, limit)
                }
                "m_MovePackageToDiscOuterEdge" => {
                    (self.m_move_package_to_disc_outer_edge).inspect_value(path, offset, limit)
                }
                "m_ForceInstallation" => {
                    (self.m_force_installation).inspect_value(path, offset, limit)
                }
                "m_ExportAsGoogleAndroidProject" => {
                    (self.m_export_as_google_android_project).inspect_value(path, offset, limit)
                }
                "m_XboxOneStreamingInstallLaunchChunkRange" => (self
                    .m_xbox_one_streaming_install_launch_chunk_range)
                    .inspect_value(path, offset, limit),
                "m_SelectedWSAUWPBuildType" => {
                    (self.m_selected_wsauwp_build_type).inspect_value(path, offset, limit)
                }
                "m_SelectedXboxOneDeployMethod" => {
                    (self.m_selected_xbox_one_deploy_method).inspect_value(path, offset, limit)
                }
                "m_SelectedWSASubtarget" => {
                    (self.m_selected_wsa_subtarget).inspect_value(path, offset, limit)
                }
                "m_ExplicitArrayBoundsChecks" => {
                    (self.m_explicit_array_bounds_checks).inspect_value(path, offset, limit)
                }
                "m_WSADotNetNativeEnabled" => {
                    (self.m_wsa_dot_net_native_enabled).inspect_value(path, offset, limit)
                }
                "m_SelectedWSAUWPSDK" => {
                    (self.m_selected_wsauwpsdk).inspect_value(path, offset, limit)
                }
                "m_CreateSolutionFileForSwitch" => {
                    (self.m_create_solution_file_for_switch).inspect_value(path, offset, limit)
                }
                "m_CreateRomFileForSwitch" => {
                    (self.m_create_rom_file_for_switch).inspect_value(path, offset, limit)
                }
                "m_NVNGraphicsDebuggerForSwitch" => {
                    (self.m_nvn_graphics_debugger_for_switch).inspect_value(path, offset, limit)
                }
                "m_EnableRomCompressionForSwitch" => {
                    (self.m_enable_rom_compression_for_switch).inspect_value(path, offset, limit)
                }
                "m_NVNShaderDebugging" => {
                    (self.m_nvn_shader_debugging).inspect_value(path, offset, limit)
                }
                "m_GenerateNintendoSwitchShaderInfo" => {
                    (self.m_generate_nintendo_switch_shader_info).inspect_value(path, offset, limit)
                }
                "m_SaveADFForSwitch" => {
                    (self.m_save_adf_for_switch).inspect_value(path, offset, limit)
                }
                "m_ForceOptimizeScriptCompilation" => {
                    (self.m_force_optimize_script_compilation).inspect_value(path, offset, limit)
                }
                "m_XboxOneUsername" => {
                    (self.m_xbox_one_username).inspect_value(path, offset, limit)
                }
                "m_EnableDebugPadForSwitch" => {
                    (self.m_enable_debug_pad_for_switch).inspect_value(path, offset, limit)
                }
                "m_RomCompressionTypeForSwitch" => {
                    (self.m_rom_compression_type_for_switch).inspect_value(path, offset, limit)
                }
                "m_NVNDrawValidationLight" => {
                    (self.m_nvn_draw_validation_light).inspect_value(path, offset, limit)
                }
                "m_XboxOneNetworkSharePath" => {
                    (self.m_xbox_one_network_share_path).inspect_value(path, offset, limit)
                }
                "m_RedirectWritesToHostMountForSwitch" => (self
                    .m_redirect_writes_to_host_mount_for_switch)
                    .inspect_value(path, offset, limit),
                "m_NVNDrawValidation" => {
                    (self.m_nvn_draw_validation).inspect_value(path, offset, limit)
                }
                "m_RomCompressionLevelForSwitch" => {
                    (self.m_rom_compression_level_for_switch).inspect_value(path, offset, limit)
                }
                "m_NVNDrawValidationHeavy" => {
                    (self.m_nvn_draw_validation_heavy).inspect_value(path, offset, limit)
                }
                "m_EnableHeapInspectorForSwitch" => {
                    (self.m_enable_heap_inspector_for_switch).inspect_value(path, offset, limit)
                }
                "m_RomCompressionConfigForSwitch" => {
                    (self.m_rom_compression_config_for_switch).inspect_value(path, offset, limit)
                }
                "m_PlatformSettings" => {
                    (self.m_platform_settings).inspect_value(path, offset, limit)
                }
                "m_BuildAppBundle" => (self.m_build_app_bundle).inspect_value(path, offset, limit),
                "m_UseLegacyNvnPoolAllocatorForSwitch" => (self
                    .m_use_legacy_nvn_pool_allocator_for_switch)
                    .inspect_value(path, offset, limit),
                "m_HTCSScriptDebuggingForSwitch" => {
                    (self.m_htcs_script_debugging_for_switch).inspect_value(path, offset, limit)
                }
                "m_AndroidCreateSymbolsZip" => {
                    (self.m_android_create_symbols_zip).inspect_value(path, offset, limit)
                }
                "m_PS5KeepPackageFiles" => {
                    (self.m_ps5_keep_package_files).inspect_value(path, offset, limit)
                }
                "m_SelectedWSAUWPVSVersion" => {
                    (self.m_selected_wsauwpvs_version).inspect_value(path, offset, limit)
                }
                "m_AndroidUseLegacySdkTools" => {
                    (self.m_android_use_legacy_sdk_tools).inspect_value(path, offset, limit)
                }
                "m_SelectedPS5Subtarget" => {
                    (self.m_selected_ps5_subtarget).inspect_value(path, offset, limit)
                }
                "m_AndroidCreateSymbols" => {
                    (self.m_android_create_symbols).inspect_value(path, offset, limit)
                }
                "m_NVNAftermath" => (self.m_nvn_aftermath).inspect_value(path, offset, limit),
                "m_SelectedPS5CompressionType" => {
                    (self.m_selected_ps5_compression_type).inspect_value(path, offset, limit)
                }
                "m_EnableMemoryTrackerForSwitch" => {
                    (self.m_enable_memory_tracker_for_switch).inspect_value(path, offset, limit)
                }
                "m_NVNAftermathLevel" => {
                    (self.m_nvn_aftermath_level).inspect_value(path, offset, limit)
                }
                "m_SelectedPS5CompressionLevel" => {
                    (self.m_selected_ps5_compression_level).inspect_value(path, offset, limit)
                }
                "m_WaitForSwitchMemoryTrackerOnStartup" => (self
                    .m_wait_for_switch_memory_tracker_on_startup)
                    .inspect_value(path, offset, limit),
                "m_SelectedWSAMinUWPSDK" => {
                    (self.m_selected_wsa_min_uwpsdk).inspect_value(path, offset, limit)
                }
                "m_PS5WorkspaceName" => {
                    (self.m_ps5_workspace_name).inspect_value(path, offset, limit)
                }
                "m_OverrideMaxTextureSize" => {
                    (self.m_override_max_texture_size).inspect_value(path, offset, limit)
                }
                "m_SelectedWSAArchitecture" => {
                    (self.m_selected_wsa_architecture).inspect_value(path, offset, limit)
                }
                "m_OverrideTextureCompression" => {
                    (self.m_override_texture_compression).inspect_value(path, offset, limit)
                }
                "m_Il2CppCodeGeneration" => {
                    (self.m_il2_cpp_code_generation).inspect_value(path, offset, limit)
                }
                "m_FacebookCreatePackageForSubmission" => (self
                    .m_facebook_create_package_for_submission)
                    .inspect_value(path, offset, limit),
                "m_EnableHostIOForSwitch" => {
                    (self.m_enable_host_io_for_switch).inspect_value(path, offset, limit)
                }
                "m_WindowsDevicePortalAddress" => {
                    (self.m_windows_device_portal_address).inspect_value(path, offset, limit)
                }
                "m_SelectedXboxOneDeployDrive" => {
                    (self.m_selected_xbox_one_deploy_drive).inspect_value(path, offset, limit)
                }
                "m_WindowsDevicePortalUsername" => {
                    (self.m_windows_device_portal_username).inspect_value(path, offset, limit)
                }
                "m_SelectedWebGLSubtarget" => {
                    (self.m_selected_web_gl_subtarget).inspect_value(path, offset, limit)
                }
                "m_EnableUnpublishableErrorsForSwitch" => (self
                    .m_enable_unpublishable_errors_for_switch)
                    .inspect_value(path, offset, limit),
                "m_WebGLClientBrowserType" => {
                    (self.m_web_gl_client_browser_type).inspect_value(path, offset, limit)
                }
                "m_WebGLClientBrowserPath" => {
                    (self.m_web_gl_client_browser_path).inspect_value(path, offset, limit)
                }
                "m_WebGLClientPlatform" => {
                    (self.m_web_gl_client_platform).inspect_value(path, offset, limit)
                }
                "m_macosXcodeBuildConfig" => {
                    (self.m_macos_xcode_build_config).inspect_value(path, offset, limit)
                }
                "m_SelectedWindowsBuildAndRunDeployTarget" => (self
                    .m_selected_windows_build_and_run_deploy_target)
                    .inspect_value(path, offset, limit),
                "m_SelectedEmbeddedLinuxArchitecture" => {
                    (self.m_selected_embedded_linux_architecture).inspect_value(path, offset, limit)
                }
                "m_RemoteDeviceInfo" => {
                    (self.m_remote_device_info).inspect_value(path, offset, limit)
                }
                "m_RemoteDeviceAddress" => {
                    (self.m_remote_device_address).inspect_value(path, offset, limit)
                }
                "m_RemoteDeviceUsername" => {
                    (self.m_remote_device_username).inspect_value(path, offset, limit)
                }
                "m_RemoteDeviceExports" => {
                    (self.m_remote_device_exports).inspect_value(path, offset, limit)
                }
                "m_PathOnRemoteDevice" => {
                    (self.m_path_on_remote_device).inspect_value(path, offset, limit)
                }
                "m_SelectedQNXOsVersion" => {
                    (self.m_selected_qnx_os_version).inspect_value(path, offset, limit)
                }
                "m_SelectedQNXArchitecture" => {
                    (self.m_selected_qnx_architecture).inspect_value(path, offset, limit)
                }
                "m_ActiveProfilePath" => {
                    (self.m_active_profile_path).inspect_value(path, offset, limit)
                }
                "m_ActiveBuildProfile" => {
                    (self.m_active_build_profile).inspect_value(path, offset, limit)
                }
                "m_BuildOutputToBuildMetadataMap" => {
                    (self.m_build_output_to_build_metadata_map).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_location).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildLocation".to_string(), v)),
            ((self.m_active_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveBuildTarget".to_string(), v)),
            ((self.m_selected_build_target_group).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedBuildTargetGroup".to_string(), v)),
            ((self.m_active_build_target_group).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveBuildTargetGroup".to_string(), v)),
            ((self.m_selected_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedBuildTarget".to_string(), v)),
            ((self.m_active_platform_guid).inspect_value(path, offset, limit))
                .map(|v| ("m_ActivePlatformGuid".to_string(), v)),
            ((self.m_selected_standalone_target).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedStandaloneTarget".to_string(), v)),
            ((self.m_architecture_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_ArchitectureFlags".to_string(), v)),
            ((self.m_active_build_platform_group_name).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveBuildPlatformGroupName".to_string(), v)),
            ((self.m_selected_wii_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWiiSubtarget".to_string(), v)),
            ((self.m_selected_psp2_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedPSP2Subtarget".to_string(), v)),
            ((self.m_selected_facebook_target).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedFacebookTarget".to_string(), v)),
            ((self.m_selected_build_platform_group_name).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedBuildPlatformGroupName".to_string(), v)),
            ((self.m_development).inspect_value(path, offset, limit))
                .map(|v| ("m_Development".to_string(), v)),
            ((self.m_selected_wii_debug_level).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWiiDebugLevel".to_string(), v)),
            ((self.m_selected_ps3_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedPS3Subtarget".to_string(), v)),
            ((self.m_selected_ps4_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedPS4Subtarget".to_string(), v)),
            ((self.m_connect_profiler).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectProfiler".to_string(), v)),
            ((self.m_selected_xbox_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedXboxSubtarget".to_string(), v)),
            ((self.m_ps4_hardware_target).inspect_value(path, offset, limit))
                .map(|v| ("m_PS4HardwareTarget".to_string(), v)),
            ((self.m_selected_standalone_build_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedStandaloneBuildSubtarget".to_string(), v)),
            ((self.m_allow_debugging).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowDebugging".to_string(), v)),
            ((self.m_selected_xbox_run_method).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedXboxRunMethod".to_string(), v)),
            ((self.m_selected_psm_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedPSMSubtarget".to_string(), v)),
            ((self.m_active_standalone_build_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveStandaloneBuildSubtarget".to_string(), v)),
            ((self.m_web_player_streamed).inspect_value(path, offset, limit))
                .map(|v| ("m_WebPlayerStreamed".to_string(), v)),
            ((self.m_selected_ios_build_type).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedIOSBuildType".to_string(), v)),
            ((self.m_selected_wii_u_debug_level).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWiiUDebugLevel".to_string(), v)),
            ((self.m_web_player_offline_deployment).inspect_value(path, offset, limit))
                .map(|v| ("m_WebPlayerOfflineDeployment".to_string(), v)),
            ((self.m_selected_wii_u_build_output).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWiiUBuildOutput".to_string(), v)),
            ((self.m_android_build_system).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidBuildSystem".to_string(), v)),
            ((self.m_web_player_na_cl_support).inspect_value(path, offset, limit))
                .map(|v| ("m_WebPlayerNaClSupport".to_string(), v)),
            ((self.m_selected_wii_u_boot_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWiiUBootMode".to_string(), v)),
            ((self.m_android_build_type).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidBuildType".to_string(), v)),
            ((self.m_install_in_build_folder).inspect_value(path, offset, limit))
                .map(|v| ("m_InstallInBuildFolder".to_string(), v)),
            ((self.m_android_release_minification).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidReleaseMinification".to_string(), v)),
            ((self.m_symlink_libraries).inspect_value(path, offset, limit))
                .map(|v| ("m_SymlinkLibraries".to_string(), v)),
            ((self.m_android_debug_minification).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidDebugMinification".to_string(), v)),
            ((self.m_selected_android_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedAndroidSubtarget".to_string(), v)),
            ((self.m_selected_android_etc2_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedAndroidETC2Fallback".to_string(), v)),
            ((self.m_android_device_socket_address).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidDeviceSocketAddress".to_string(), v)),
            ((self.m_explicit_null_checks).inspect_value(path, offset, limit))
                .map(|v| ("m_ExplicitNullChecks".to_string(), v)),
            ((self.m_web_gl_use_pre_built_unity_engine).inspect_value(path, offset, limit))
                .map(|v| ("m_WebGLUsePreBuiltUnityEngine".to_string(), v)),
            ((self.m_android_current_deployment_target_id).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidCurrentDeploymentTargetId".to_string(), v)),
            ((self.m_enable_headless_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableHeadlessMode".to_string(), v)),
            ((self.m_selected_compression_type).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedCompressionType".to_string(), v)),
            ((self.m_xbox_compressed_xex).inspect_value(path, offset, limit))
                .map(|v| ("m_XboxCompressedXex".to_string(), v)),
            ((self.m_web_player_deploy_online).inspect_value(path, offset, limit))
                .map(|v| ("m_WebPlayerDeployOnline".to_string(), v)),
            ((self.m_need_submission_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_NeedSubmissionMaterials".to_string(), v)),
            ((self.m_symlink_trampoline).inspect_value(path, offset, limit))
                .map(|v| ("m_SymlinkTrampoline".to_string(), v)),
            ((self.m_facebook_access_token).inspect_value(path, offset, limit))
                .map(|v| ("m_FacebookAccessToken".to_string(), v)),
            ((self.m_compress_with_ps_arc).inspect_value(path, offset, limit))
                .map(|v| ("m_CompressWithPsArc".to_string(), v)),
            ((self.m_selected_metro_build_type).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedMetroBuildType".to_string(), v)),
            ((self.m_selected_metro_target).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedMetroTarget".to_string(), v)),
            ((self.m_build_with_deep_profiling_support).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildWithDeepProfilingSupport".to_string(), v)),
            ((self.m_dataless_player).inspect_value(path, offset, limit))
                .map(|v| ("m_DatalessPlayer".to_string(), v)),
            ((self.m_selected_metro_sdk).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedMetroSDK".to_string(), v)),
            ((self.m_build_scripts_only).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildScriptsOnly".to_string(), v)),
            ((self.m_wait_for_player_connection).inspect_value(path, offset, limit))
                .map(|v| ("m_WaitForPlayerConnection".to_string(), v)),
            ((self.m_selected_diagnostic_setting).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedDiagnosticSetting".to_string(), v)),
            ((self.m_selected_black_berry_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedBlackBerrySubtarget".to_string(), v)),
            ((self.m_generate_metro_reference_projects).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateMetroReferenceProjects".to_string(), v)),
            ((self.m_selected_metro_build_and_run_deploy_target)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_SelectedMetroBuildAndRunDeployTarget".to_string(), v)),
            ((self.m_selected_black_berry_build_type).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedBlackBerryBuildType".to_string(), v)),
            ((self.m_web_gl_optimization_level).inspect_value(path, offset, limit))
                .map(|v| ("m_WebGLOptimizationLevel".to_string(), v)),
            ((self.m_wsa_holographic_remoting).inspect_value(path, offset, limit))
                .map(|v| ("m_WsaHolographicRemoting".to_string(), v)),
            ((self.m_selected_tizen_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedTizenSubtarget".to_string(), v)),
            ((self.m_selected_wsasdk).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWSASDK".to_string(), v)),
            ((self.m_compress_files_in_package).inspect_value(path, offset, limit))
                .map(|v| ("m_CompressFilesInPackage".to_string(), v)),
            ((self.m_build_with_code_coverage).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildWithCodeCoverage".to_string(), v)),
            ((self.m_selected_wsa_build_and_run_deploy_target).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWSABuildAndRunDeployTarget".to_string(), v)),
            ((self.m_explicit_divide_by_zero_checks).inspect_value(path, offset, limit))
                .map(|v| ("m_ExplicitDivideByZeroChecks".to_string(), v)),
            ((self.m_symlink_sources).inspect_value(path, offset, limit))
                .map(|v| ("m_SymlinkSources".to_string(), v)),
            ((self.m_generate_wsa_reference_projects).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateWSAReferenceProjects".to_string(), v)),
            ((self.m_wii_u_enable_net_api).inspect_value(path, offset, limit))
                .map(|v| ("m_WiiUEnableNetAPI".to_string(), v)),
            ((self.m_move_package_to_disc_outer_edge).inspect_value(path, offset, limit))
                .map(|v| ("m_MovePackageToDiscOuterEdge".to_string(), v)),
            ((self.m_force_installation).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceInstallation".to_string(), v)),
            ((self.m_export_as_google_android_project).inspect_value(path, offset, limit))
                .map(|v| ("m_ExportAsGoogleAndroidProject".to_string(), v)),
            ((self.m_xbox_one_streaming_install_launch_chunk_range)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_XboxOneStreamingInstallLaunchChunkRange".to_string(), v)),
            ((self.m_selected_wsauwp_build_type).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWSAUWPBuildType".to_string(), v)),
            ((self.m_selected_xbox_one_deploy_method).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedXboxOneDeployMethod".to_string(), v)),
            ((self.m_selected_wsa_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWSASubtarget".to_string(), v)),
            ((self.m_explicit_array_bounds_checks).inspect_value(path, offset, limit))
                .map(|v| ("m_ExplicitArrayBoundsChecks".to_string(), v)),
            ((self.m_wsa_dot_net_native_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_WSADotNetNativeEnabled".to_string(), v)),
            ((self.m_selected_wsauwpsdk).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWSAUWPSDK".to_string(), v)),
            ((self.m_create_solution_file_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_CreateSolutionFileForSwitch".to_string(), v)),
            ((self.m_create_rom_file_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_CreateRomFileForSwitch".to_string(), v)),
            ((self.m_nvn_graphics_debugger_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_NVNGraphicsDebuggerForSwitch".to_string(), v)),
            ((self.m_enable_rom_compression_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableRomCompressionForSwitch".to_string(), v)),
            ((self.m_nvn_shader_debugging).inspect_value(path, offset, limit))
                .map(|v| ("m_NVNShaderDebugging".to_string(), v)),
            ((self.m_generate_nintendo_switch_shader_info).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateNintendoSwitchShaderInfo".to_string(), v)),
            ((self.m_save_adf_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_SaveADFForSwitch".to_string(), v)),
            ((self.m_force_optimize_script_compilation).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceOptimizeScriptCompilation".to_string(), v)),
            ((self.m_xbox_one_username).inspect_value(path, offset, limit))
                .map(|v| ("m_XboxOneUsername".to_string(), v)),
            ((self.m_enable_debug_pad_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableDebugPadForSwitch".to_string(), v)),
            ((self.m_rom_compression_type_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_RomCompressionTypeForSwitch".to_string(), v)),
            ((self.m_nvn_draw_validation_light).inspect_value(path, offset, limit))
                .map(|v| ("m_NVNDrawValidationLight".to_string(), v)),
            ((self.m_xbox_one_network_share_path).inspect_value(path, offset, limit))
                .map(|v| ("m_XboxOneNetworkSharePath".to_string(), v)),
            ((self.m_redirect_writes_to_host_mount_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_RedirectWritesToHostMountForSwitch".to_string(), v)),
            ((self.m_nvn_draw_validation).inspect_value(path, offset, limit))
                .map(|v| ("m_NVNDrawValidation".to_string(), v)),
            ((self.m_rom_compression_level_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_RomCompressionLevelForSwitch".to_string(), v)),
            ((self.m_nvn_draw_validation_heavy).inspect_value(path, offset, limit))
                .map(|v| ("m_NVNDrawValidationHeavy".to_string(), v)),
            ((self.m_enable_heap_inspector_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableHeapInspectorForSwitch".to_string(), v)),
            ((self.m_rom_compression_config_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_RomCompressionConfigForSwitch".to_string(), v)),
            ((self.m_platform_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_PlatformSettings".to_string(), v)),
            ((self.m_build_app_bundle).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildAppBundle".to_string(), v)),
            ((self.m_use_legacy_nvn_pool_allocator_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLegacyNvnPoolAllocatorForSwitch".to_string(), v)),
            ((self.m_htcs_script_debugging_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_HTCSScriptDebuggingForSwitch".to_string(), v)),
            ((self.m_android_create_symbols_zip).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidCreateSymbolsZip".to_string(), v)),
            ((self.m_ps5_keep_package_files).inspect_value(path, offset, limit))
                .map(|v| ("m_PS5KeepPackageFiles".to_string(), v)),
            ((self.m_selected_wsauwpvs_version).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWSAUWPVSVersion".to_string(), v)),
            ((self.m_android_use_legacy_sdk_tools).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidUseLegacySdkTools".to_string(), v)),
            ((self.m_selected_ps5_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedPS5Subtarget".to_string(), v)),
            ((self.m_android_create_symbols).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidCreateSymbols".to_string(), v)),
            ((self.m_nvn_aftermath).inspect_value(path, offset, limit))
                .map(|v| ("m_NVNAftermath".to_string(), v)),
            ((self.m_selected_ps5_compression_type).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedPS5CompressionType".to_string(), v)),
            ((self.m_enable_memory_tracker_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableMemoryTrackerForSwitch".to_string(), v)),
            ((self.m_nvn_aftermath_level).inspect_value(path, offset, limit))
                .map(|v| ("m_NVNAftermathLevel".to_string(), v)),
            ((self.m_selected_ps5_compression_level).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedPS5CompressionLevel".to_string(), v)),
            ((self.m_wait_for_switch_memory_tracker_on_startup).inspect_value(path, offset, limit))
                .map(|v| ("m_WaitForSwitchMemoryTrackerOnStartup".to_string(), v)),
            ((self.m_selected_wsa_min_uwpsdk).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWSAMinUWPSDK".to_string(), v)),
            ((self.m_ps5_workspace_name).inspect_value(path, offset, limit))
                .map(|v| ("m_PS5WorkspaceName".to_string(), v)),
            ((self.m_override_max_texture_size).inspect_value(path, offset, limit))
                .map(|v| ("m_OverrideMaxTextureSize".to_string(), v)),
            ((self.m_selected_wsa_architecture).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWSAArchitecture".to_string(), v)),
            ((self.m_override_texture_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_OverrideTextureCompression".to_string(), v)),
            ((self.m_il2_cpp_code_generation).inspect_value(path, offset, limit))
                .map(|v| ("m_Il2CppCodeGeneration".to_string(), v)),
            ((self.m_facebook_create_package_for_submission).inspect_value(path, offset, limit))
                .map(|v| ("m_FacebookCreatePackageForSubmission".to_string(), v)),
            ((self.m_enable_host_io_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableHostIOForSwitch".to_string(), v)),
            ((self.m_windows_device_portal_address).inspect_value(path, offset, limit))
                .map(|v| ("m_WindowsDevicePortalAddress".to_string(), v)),
            ((self.m_selected_xbox_one_deploy_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedXboxOneDeployDrive".to_string(), v)),
            ((self.m_windows_device_portal_username).inspect_value(path, offset, limit))
                .map(|v| ("m_WindowsDevicePortalUsername".to_string(), v)),
            ((self.m_selected_web_gl_subtarget).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedWebGLSubtarget".to_string(), v)),
            ((self.m_enable_unpublishable_errors_for_switch).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableUnpublishableErrorsForSwitch".to_string(), v)),
            ((self.m_web_gl_client_browser_type).inspect_value(path, offset, limit))
                .map(|v| ("m_WebGLClientBrowserType".to_string(), v)),
            ((self.m_web_gl_client_browser_path).inspect_value(path, offset, limit))
                .map(|v| ("m_WebGLClientBrowserPath".to_string(), v)),
            ((self.m_web_gl_client_platform).inspect_value(path, offset, limit))
                .map(|v| ("m_WebGLClientPlatform".to_string(), v)),
            ((self.m_macos_xcode_build_config).inspect_value(path, offset, limit))
                .map(|v| ("m_macosXcodeBuildConfig".to_string(), v)),
            ((self.m_selected_windows_build_and_run_deploy_target)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_SelectedWindowsBuildAndRunDeployTarget".to_string(), v)),
            ((self.m_selected_embedded_linux_architecture).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedEmbeddedLinuxArchitecture".to_string(), v)),
            ((self.m_remote_device_info).inspect_value(path, offset, limit))
                .map(|v| ("m_RemoteDeviceInfo".to_string(), v)),
            ((self.m_remote_device_address).inspect_value(path, offset, limit))
                .map(|v| ("m_RemoteDeviceAddress".to_string(), v)),
            ((self.m_remote_device_username).inspect_value(path, offset, limit))
                .map(|v| ("m_RemoteDeviceUsername".to_string(), v)),
            ((self.m_remote_device_exports).inspect_value(path, offset, limit))
                .map(|v| ("m_RemoteDeviceExports".to_string(), v)),
            ((self.m_path_on_remote_device).inspect_value(path, offset, limit))
                .map(|v| ("m_PathOnRemoteDevice".to_string(), v)),
            ((self.m_selected_qnx_os_version).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedQNXOsVersion".to_string(), v)),
            ((self.m_selected_qnx_architecture).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectedQNXArchitecture".to_string(), v)),
            ((self.m_active_profile_path).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveProfilePath".to_string(), v)),
            ((self.m_active_build_profile).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveBuildProfile".to_string(), v)),
            ((self.m_build_output_to_build_metadata_map).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildOutputToBuildMetadataMap".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
