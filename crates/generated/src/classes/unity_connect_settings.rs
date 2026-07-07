//! 自动生成。请勿手动修改。

use crate::subclasses::crash_reporting_settings::CrashReportingSettings;
use crate::subclasses::insights_settings::InsightsSettings;
use crate::subclasses::performance_reporting_settings::PerformanceReportingSettings;
use crate::subclasses::unity_ads_settings::UnityAdsSettings;
use crate::subclasses::unity_analytics_settings::UnityAnalyticsSettings;
use crate::subclasses::unity_purchasing_settings::UnityPurchasingSettings;

#[derive(Debug, Clone)]
pub struct UnityConnectSettings {
    pub unity_purchasing_settings: UnityPurchasingSettings,
    pub m_enabled: Option<bool>,
    pub unity_analytics_settings: UnityAnalyticsSettings,
    pub m_test_mode: Option<bool>,
    pub m_test_event_url: Option<String>,
    pub m_event_old_url: Option<String>,
    pub m_test_config_url: Option<String>,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let unity_purchasing_settings = UnityPurchasingSettings::parse(r, ver)?;
        r.align(4)?;
        let m_enabled = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let unity_analytics_settings = UnityAnalyticsSettings::parse(r, ver)?;
        r.align(4)?;
        let m_test_mode = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_test_event_url = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_event_old_url = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_test_config_url = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_event_url = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let crash_reporting_settings = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = CrashReportingSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_test_init_mode = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_config_url = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_dashboard_url = if *ver >= crate::UnityVersion::new(2020, 3, 5) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let unity_ads_settings = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = UnityAdsSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let insights_settings = if *ver >= crate::UnityVersion::new(2022, 3, 74) {
            let v = InsightsSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let performance_reporting_settings = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = PerformanceReportingSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(UnityConnectSettings {
            unity_purchasing_settings,
            m_enabled,
            unity_analytics_settings,
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
