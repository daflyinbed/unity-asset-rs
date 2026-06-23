//! 自动生成。请勿手动修改。

use crate::subclasses::unity_analytics_settings::UnityAnalyticsSettings;
use crate::subclasses::unity_purchasing_settings::UnityPurchasingSettings;

#[derive(Debug, Clone)]
pub struct UnityConnectSettings {
    pub m_object_hide_flags: u32,
    pub unity_purchasing_settings: UnityPurchasingSettings,
    pub unity_analytics_settings: UnityAnalyticsSettings,
}

impl crate::Parse for UnityConnectSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let unity_purchasing_settings = UnityPurchasingSettings::parse(r, ver)?;
        r.align(4)?;
        let unity_analytics_settings = UnityAnalyticsSettings::parse(r, ver)?;
        r.align(4)?;

        Ok(UnityConnectSettings {
            m_object_hide_flags,
            unity_purchasing_settings,
            unity_analytics_settings,
        })
    }
}
