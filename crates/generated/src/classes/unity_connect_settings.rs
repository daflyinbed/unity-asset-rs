//! 自动生成。请勿手动修改。

use crate::classes::unity_ads_settings::UnityAdsSettings;
use crate::subclasses::crash_reporting_settings::CrashReportingSettings;
use crate::subclasses::insights_settings::InsightsSettings;
use crate::subclasses::performance_reporting_settings::PerformanceReportingSettings;
use crate::subclasses::unity_analytics_settings::UnityAnalyticsSettings;
use crate::subclasses::unity_purchasing_settings::UnityPurchasingSettings;


/// 版本枚举：UnityConnectSettingsMTestConfigUrlVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum UnityConnectSettingsMTestConfigUrlVariant {
    Default(String),
    V5_6(String),
}

impl crate::Parse for UnityConnectSettingsMTestConfigUrlVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct UnityConnectSettings {
    pub unity_purchasing_settings: UnityPurchasingSettings,
    pub m_enabled: Option<bool>,
    pub unity_analytics_settings: UnityAnalyticsSettings,
    pub m_test_mode: Option<bool>,
    pub m_test_event_url: Option<String>,
    pub m_event_old_url: Option<String>,
    pub m_test_config_url: Option<UnityConnectSettingsMTestConfigUrlVariant>,
    pub m_event_url: Option<String>,
    pub crash_reporting_settings: Option<CrashReportingSettings>,
    pub m_test_init_mode: Option<i32>,
    pub m_config_url: Option<String>,
    pub m_dashboard_url: Option<String>,
    pub unity_ads_settings: Option<UnityAdsSettings>,
    pub insights_settings: Option<InsightsSettings>,
    pub performance_reporting_settings: Option<PerformanceReportingSettings>,
}

impl crate::Parse for UnityConnectSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut unity_purchasing_settings = None;
        let mut m_enabled = None;
        let mut unity_analytics_settings = None;
        let mut m_test_mode = None;
        let mut m_test_event_url = None;
        let mut m_event_old_url = None;
        let mut m_test_config_url = None;
        let mut m_event_url = None;
        let mut crash_reporting_settings = None;
        let mut m_test_init_mode = None;
        let mut m_config_url = None;
        let mut m_dashboard_url = None;
        let mut unity_ads_settings = None;
        let mut insights_settings = None;
        let mut performance_reporting_settings = None;
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_event_old_url = Some(rdr.read_aligned_string()?);
            m_event_url = Some(rdr.read_aligned_string()?);
            m_config_url = Some(rdr.read_aligned_string()?);
            m_dashboard_url = Some(rdr.read_aligned_string()?);
            m_test_init_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            insights_settings = Some(InsightsSettings::parse(rdr, ver)?);
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_event_old_url = Some(rdr.read_aligned_string()?);
            m_event_url = Some(rdr.read_aligned_string()?);
            m_config_url = Some(rdr.read_aligned_string()?);
            m_dashboard_url = Some(rdr.read_aligned_string()?);
            m_test_init_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 70) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_event_old_url = Some(rdr.read_aligned_string()?);
            m_event_url = Some(rdr.read_aligned_string()?);
            m_config_url = Some(rdr.read_aligned_string()?);
            m_dashboard_url = Some(rdr.read_aligned_string()?);
            m_test_init_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            insights_settings = Some(InsightsSettings::parse(rdr, ver)?);
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_event_old_url = Some(rdr.read_aligned_string()?);
            m_event_url = Some(rdr.read_aligned_string()?);
            m_config_url = Some(rdr.read_aligned_string()?);
            m_dashboard_url = Some(rdr.read_aligned_string()?);
            m_test_init_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 74) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_event_old_url = Some(rdr.read_aligned_string()?);
            m_event_url = Some(rdr.read_aligned_string()?);
            m_config_url = Some(rdr.read_aligned_string()?);
            m_dashboard_url = Some(rdr.read_aligned_string()?);
            m_test_init_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            insights_settings = Some(InsightsSettings::parse(rdr, ver)?);
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 5) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_event_old_url = Some(rdr.read_aligned_string()?);
            m_event_url = Some(rdr.read_aligned_string()?);
            m_config_url = Some(rdr.read_aligned_string()?);
            m_dashboard_url = Some(rdr.read_aligned_string()?);
            m_test_init_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_event_old_url = Some(rdr.read_aligned_string()?);
            m_event_url = Some(rdr.read_aligned_string()?);
            m_config_url = Some(rdr.read_aligned_string()?);
            m_test_init_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_test_event_url = Some(rdr.read_aligned_string()?);
            m_test_config_url = Some(UnityConnectSettingsMTestConfigUrlVariant::parse(rdr, ver)?);
            m_test_init_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_enabled = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_test_event_url = Some(rdr.read_aligned_string()?);
            m_test_config_url = Some(UnityConnectSettingsMTestConfigUrlVariant::parse(rdr, ver)?);
            crash_reporting_settings = Some(CrashReportingSettings::parse(rdr, ver)?);
        } else {
        }
            unity_purchasing_settings = Some(UnityPurchasingSettings::parse(rdr, ver)?);
            unity_analytics_settings = Some(UnityAnalyticsSettings::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            unity_ads_settings = Some(UnityAdsSettings::parse(rdr, ver)?);
            performance_reporting_settings = Some(PerformanceReportingSettings::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            unity_ads_settings = Some(UnityAdsSettings::parse(rdr, ver)?);
        } else {
        }

        Ok(UnityConnectSettings {
            unity_purchasing_settings: unity_purchasing_settings.unwrap(),
            m_enabled,
            unity_analytics_settings: unity_analytics_settings.unwrap(),
            m_test_mode,
            m_test_event_url,
            m_event_old_url,
            m_test_config_url,
            m_event_url,
            crash_reporting_settings,
            m_test_init_mode,
            m_config_url,
            m_dashboard_url,
            unity_ads_settings,
            insights_settings,
            performance_reporting_settings,
        })
    }
}
