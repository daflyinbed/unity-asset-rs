//! 自动生成。请勿手动修改。

use crate::subclasses::quality_setting::QualitySetting;

#[derive(Debug, Clone)]
pub struct QualitySettings {
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_default_standalone_quality = None;
        let mut m_current_quality = None;
        let mut m_default_web_player_quality = None;
        let mut m_quality_settings = None;
        let mut m_default_mobile_quality = None;
        let mut m_stripped_maximum_lod_level = None;
        let mut m_texture_mipmap_limit_group_names = None;
        let mut m_editor_quality = None;
        let mut fastest = None;
        let mut fast = None;
        let mut simple = None;
        let mut good = None;
        let mut beautiful = None;
        let mut fantastic = None;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_current_quality = Some(rdr.read_i32()?);
            m_quality_settings = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { QualitySetting::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_texture_mipmap_limit_group_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_stripped_maximum_lod_level = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_current_quality = Some(rdr.read_i32()?);
            m_quality_settings = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { QualitySetting::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_stripped_maximum_lod_level = Some(rdr.read_i32()?);
        } else {
            m_default_standalone_quality = Some(rdr.read_i32()?);
            m_default_web_player_quality = Some(rdr.read_i32()?);
            m_default_mobile_quality = Some(rdr.read_i32()?);
            m_editor_quality = Some(rdr.read_i32()?);
            fastest = Some(QualitySetting::parse(rdr, ver)?);
            rdr.align(4)?;
            fast = Some(QualitySetting::parse(rdr, ver)?);
            rdr.align(4)?;
            simple = Some(QualitySetting::parse(rdr, ver)?);
            rdr.align(4)?;
            good = Some(QualitySetting::parse(rdr, ver)?);
            rdr.align(4)?;
            beautiful = Some(QualitySetting::parse(rdr, ver)?);
            rdr.align(4)?;
            fantastic = Some(QualitySetting::parse(rdr, ver)?);
            rdr.align(4)?;
        }

        Ok(QualitySettings {
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
