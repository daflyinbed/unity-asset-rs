//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UnityAdsSettings {
    pub m_enabled: Option<bool>,
    pub m_initialize_on_startup: Option<bool>,
    pub m_test_mode: Option<bool>,
    pub m_enabled_platforms: Option<u32>,
    pub m_ios_game_id: Option<String>,
    pub m_android_game_id: Option<String>,
}

impl crate::Parse for UnityAdsSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_enabled = None;
        let mut m_initialize_on_startup = None;
        let mut m_test_mode = None;
        let mut m_enabled_platforms = None;
        let mut m_ios_game_id = None;
        let mut m_android_game_id = None;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_enabled = Some(rdr.read_bool()?);
            m_initialize_on_startup = Some(rdr.read_bool()?);
            m_test_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enabled_platforms = Some(rdr.read_u32()?);
            m_ios_game_id = Some(rdr.read_aligned_string()?);
            m_android_game_id = Some(rdr.read_aligned_string()?);
        }

        Ok(UnityAdsSettings {
            m_enabled,
            m_initialize_on_startup,
            m_test_mode,
            m_enabled_platforms,
            m_ios_game_id,
            m_android_game_id,
        })
    }
}

impl crate::ToValue for UnityAdsSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_Enabled", &self.m_enabled),
            crate::opt_field("m_InitializeOnStartup", &self.m_initialize_on_startup),
            crate::opt_field("m_TestMode", &self.m_test_mode),
            crate::opt_field("m_EnabledPlatforms", &self.m_enabled_platforms),
            crate::opt_field("m_IosGameId", &self.m_ios_game_id),
            crate::opt_field("m_AndroidGameId", &self.m_android_game_id),
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
                "m_EnabledPlatforms" => {
                    (self.m_enabled_platforms).inspect_value(path, offset, limit)
                }
                "m_IosGameId" => (self.m_ios_game_id).inspect_value(path, offset, limit),
                "m_AndroidGameId" => (self.m_android_game_id).inspect_value(path, offset, limit),
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
            ((self.m_enabled_platforms).inspect_value(path, offset, limit))
                .map(|v| ("m_EnabledPlatforms".to_string(), v)),
            ((self.m_ios_game_id).inspect_value(path, offset, limit))
                .map(|v| ("m_IosGameId".to_string(), v)),
            ((self.m_android_game_id).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidGameId".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
