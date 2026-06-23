//! 自动生成。请勿手动修改。

use crate::subclasses::quality_setting::QualitySetting;
use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct QualitySettings {
    pub parent: GlobalGameManager,
    pub m_default_standalone_quality: Option<i32>,
    pub m_current_quality: Option<i32>,
    pub m_default_web_player_quality: Option<i32>,
    pub m_quality_settings: Option<Vec<QualitySetting>>,
    pub m_default_mobile_quality: Option<i32>,
    pub m_stripped_maximum_lod_level: Option<i32>,
    pub m_texture_mipmap_limit_group_names: Option<Vec<String>>,
    pub m_editor_quality: Option<i32>,
    pub fastest: Option<QualitySetting>,
    pub fast: Option<QualitySetting>,
    pub simple: Option<QualitySetting>,
    pub good: Option<QualitySetting>,
    pub beautiful: Option<QualitySetting>,
    pub fantastic: Option<QualitySetting>,
}

impl crate::Parse for QualitySettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_default_standalone_quality = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_current_quality = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_default_web_player_quality = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_quality_settings = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { QualitySetting::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_default_mobile_quality = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_stripped_maximum_lod_level = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_mipmap_limit_group_names = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_editor_quality = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let fastest = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = QualitySetting::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let fast = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = QualitySetting::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let simple = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = QualitySetting::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let good = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = QualitySetting::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let beautiful = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = QualitySetting::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let fantastic = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = QualitySetting::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(QualitySettings {
            parent,
            m_default_standalone_quality,
            m_current_quality,
            m_default_web_player_quality,
            m_quality_settings,
            m_default_mobile_quality,
            m_stripped_maximum_lod_level,
            m_texture_mipmap_limit_group_names,
            m_editor_quality,
            fastest,
            fast,
            simple,
            good,
            beautiful,
            fantastic,
        })
    }
}
