//! 自动生成。请勿手动修改。

use crate::subclasses::quality_setting::QualitySetting;

#[derive(Debug, Clone)]
pub struct QualitySettings {
    pub m_object_hide_flags: u32,
    pub m_default_standalone_quality: i32,
    pub m_default_web_player_quality: i32,
    pub m_default_mobile_quality: i32,
    pub m_editor_quality: i32,
    pub fastest: QualitySetting,
    pub fast: QualitySetting,
    pub simple: QualitySetting,
    pub good: QualitySetting,
    pub beautiful: QualitySetting,
    pub fantastic: QualitySetting,
}

impl crate::Parse for QualitySettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_default_standalone_quality = r.read_i32()?;
        let m_default_web_player_quality = r.read_i32()?;
        let m_default_mobile_quality = r.read_i32()?;
        let m_editor_quality = r.read_i32()?;
        let fastest = QualitySetting::parse(r, ver)?;
        r.align(4)?;
        let fast = QualitySetting::parse(r, ver)?;
        r.align(4)?;
        let simple = QualitySetting::parse(r, ver)?;
        r.align(4)?;
        let good = QualitySetting::parse(r, ver)?;
        r.align(4)?;
        let beautiful = QualitySetting::parse(r, ver)?;
        r.align(4)?;
        let fantastic = QualitySetting::parse(r, ver)?;
        r.align(4)?;

        Ok(QualitySettings {
            m_object_hide_flags,
            m_default_standalone_quality,
            m_default_web_player_quality,
            m_default_mobile_quality,
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
