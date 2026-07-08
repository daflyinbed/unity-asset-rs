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
            rdr.align(4)?;
            m_android_game_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
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
