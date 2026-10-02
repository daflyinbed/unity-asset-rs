//! 自动生成。请勿手动修改。

/// 版本枚举：UnityAnalyticsSettingsMInitializeOnStartupVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum UnityAnalyticsSettingsMInitializeOnStartupVariant {
    Default(bool),
    V2018_3(bool),
    V2020_3_41(bool),
    V2021_1(bool),
    V2021_3_12(bool),
    V2022_1(bool),
    V2022_1_21(bool),
}

impl crate::Parse for UnityAnalyticsSettingsMInitializeOnStartupVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 21) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_1_21(v))
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_1(v))
        } else if *ver >= crate::UnityVersion::new(2021, 3, 12) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_3_12(v))
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2020, 3, 41) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_3_41(v))
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for UnityAnalyticsSettingsMInitializeOnStartupVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
            Self::V2020_3_41(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
            Self::V2021_3_12(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
            Self::V2022_1_21(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_3_41(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_3_12(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1_21(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：UnityAnalyticsSettingsMTestModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum UnityAnalyticsSettingsMTestModeVariant {
    Default(bool),
    V2018_3(bool),
}

impl crate::Parse for UnityAnalyticsSettingsMTestModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for UnityAnalyticsSettingsMTestModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct UnityAnalyticsSettings {
    pub m_enabled: bool,
    pub m_initialize_on_startup: UnityAnalyticsSettingsMInitializeOnStartupVariant,
    pub m_test_mode: UnityAnalyticsSettingsMTestModeVariant,
    pub m_test_event_url: Option<String>,
    pub m_package_requiring_core_stats_present: Option<bool>,
    pub m_test_config_url: Option<String>,
}

impl crate::Parse for UnityAnalyticsSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_enabled = None;
        let mut m_initialize_on_startup = None;
        let mut m_test_mode = None;
        let mut m_test_event_url = None;
        let mut m_package_requiring_core_stats_present = None;
        let mut m_test_config_url = None;
        m_enabled = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
        } else {
            m_initialize_on_startup = Some(
                UnityAnalyticsSettingsMInitializeOnStartupVariant::parse(rdr, ver)?,
            );
        }
        m_test_mode = Some(UnityAnalyticsSettingsMTestModeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 1, 21) {
            m_initialize_on_startup = Some(
                UnityAnalyticsSettingsMInitializeOnStartupVariant::parse(rdr, ver)?,
            );
            m_package_requiring_core_stats_present = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_initialize_on_startup = Some(
                UnityAnalyticsSettingsMInitializeOnStartupVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 3, 12) {
            m_initialize_on_startup = Some(
                UnityAnalyticsSettingsMInitializeOnStartupVariant::parse(rdr, ver)?,
            );
            m_package_requiring_core_stats_present = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_initialize_on_startup = Some(
                UnityAnalyticsSettingsMInitializeOnStartupVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 3, 41) {
            m_initialize_on_startup = Some(
                UnityAnalyticsSettingsMInitializeOnStartupVariant::parse(rdr, ver)?,
            );
            m_package_requiring_core_stats_present = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_initialize_on_startup = Some(
                UnityAnalyticsSettingsMInitializeOnStartupVariant::parse(rdr, ver)?,
            );
        } else {
            m_test_event_url = Some(rdr.read_aligned_string()?);
            m_test_config_url = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        }

        Ok(UnityAnalyticsSettings {
            m_enabled: m_enabled.unwrap(),
            m_initialize_on_startup: m_initialize_on_startup.unwrap(),
            m_test_mode: m_test_mode.unwrap(),
            m_test_event_url,
            m_package_requiring_core_stats_present,
            m_test_config_url,
        })
    }
}

impl crate::ToValue for UnityAnalyticsSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_InitializeOnStartup".to_string(),
                self.m_initialize_on_startup.to_value(),
            )),
            Some(("m_TestMode".to_string(), self.m_test_mode.to_value())),
            crate::opt_field("m_TestEventUrl", &self.m_test_event_url),
            crate::opt_field(
                "m_PackageRequiringCoreStatsPresent",
                &self.m_package_requiring_core_stats_present,
            ),
            crate::opt_field("m_TestConfigUrl", &self.m_test_config_url),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_InitializeOnStartup" => {
                    (self.m_initialize_on_startup).inspect_value(path, offset, limit)
                }
                "m_TestMode" => (self.m_test_mode).inspect_value(path, offset, limit),
                "m_TestEventUrl" => (self.m_test_event_url).inspect_value(path, offset, limit),
                "m_PackageRequiringCoreStatsPresent" => {
                    (self.m_package_requiring_core_stats_present).inspect_value(path, offset, limit)
                }
                "m_TestConfigUrl" => (self.m_test_config_url).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_initialize_on_startup).inspect_value(path, offset, limit))
                .map(|v| ("m_InitializeOnStartup".to_string(), v)),
            ((self.m_test_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_TestMode".to_string(), v)),
            ((self.m_test_event_url).inspect_value(path, offset, limit))
                .map(|v| ("m_TestEventUrl".to_string(), v)),
            ((self.m_package_requiring_core_stats_present).inspect_value(path, offset, limit))
                .map(|v| ("m_PackageRequiringCoreStatsPresent".to_string(), v)),
            ((self.m_test_config_url).inspect_value(path, offset, limit))
                .map(|v| ("m_TestConfigUrl".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
