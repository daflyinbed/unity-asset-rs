//! 自动生成。请勿手动修改。

use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct UnityAdsSettings {
    pub parent: GlobalGameManager,
    pub m_enabled: Option<bool>,
    pub m_initialize_on_startup: Option<bool>,
    pub m_test_mode: Option<bool>,
    pub m_enabled_platforms: Option<u32>,
    pub m_ios_game_id: Option<String>,
    pub m_android_game_id: Option<String>,
}

impl crate::Parse for UnityAdsSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_enabled = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_initialize_on_startup = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_test_mode = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enabled_platforms = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_ios_game_id = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_android_game_id = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(UnityAdsSettings {
            parent,
            m_enabled,
            m_initialize_on_startup,
            m_test_mode,
            m_enabled_platforms,
            m_ios_game_id,
            m_android_game_id,
        })
    }
}
