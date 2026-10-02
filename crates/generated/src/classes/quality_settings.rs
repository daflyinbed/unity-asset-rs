//! 自动生成。请勿手动修改。

use crate::subclasses::quality_setting::QualitySetting;

/// 版本枚举：QualitySettingsMQualitySettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingsMQualitySettingsVariant {
    Default(Vec<QualitySetting>),
    V2017_1(Vec<QualitySetting>),
}

impl crate::Parse for QualitySettingsMQualitySettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| QualitySetting::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| QualitySetting::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingsMQualitySettingsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QualitySettings {
    pub m_default_standalone_quality: Option<i32>,
    pub m_current_quality: Option<i32>,
    pub m_default_web_player_quality: Option<i32>,
    pub m_quality_settings: Option<QualitySettingsMQualitySettingsVariant>,
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
            m_quality_settings = Some(QualitySettingsMQualitySettingsVariant::parse(rdr, ver)?);
            m_texture_mipmap_limit_group_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_stripped_maximum_lod_level = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_current_quality = Some(rdr.read_i32()?);
            m_quality_settings = Some(QualitySettingsMQualitySettingsVariant::parse(rdr, ver)?);
            m_stripped_maximum_lod_level = Some(rdr.read_i32()?);
        } else {
            m_default_standalone_quality = Some(rdr.read_i32()?);
            m_default_web_player_quality = Some(rdr.read_i32()?);
            m_default_mobile_quality = Some(rdr.read_i32()?);
            m_editor_quality = Some(rdr.read_i32()?);
            fastest = Some(QualitySetting::parse(rdr, ver)?);
            fast = Some(QualitySetting::parse(rdr, ver)?);
            simple = Some(QualitySetting::parse(rdr, ver)?);
            good = Some(QualitySetting::parse(rdr, ver)?);
            beautiful = Some(QualitySetting::parse(rdr, ver)?);
            fantastic = Some(QualitySetting::parse(rdr, ver)?);
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

impl crate::ToValue for QualitySettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field(
                "m_DefaultStandaloneQuality",
                &self.m_default_standalone_quality,
            ),
            crate::opt_field("m_CurrentQuality", &self.m_current_quality),
            crate::opt_field(
                "m_DefaultWebPlayerQuality",
                &self.m_default_web_player_quality,
            ),
            crate::opt_field("m_QualitySettings", &self.m_quality_settings),
            crate::opt_field("m_DefaultMobileQuality", &self.m_default_mobile_quality),
            crate::opt_field(
                "m_StrippedMaximumLODLevel",
                &self.m_stripped_maximum_lod_level,
            ),
            crate::opt_field(
                "m_TextureMipmapLimitGroupNames",
                &self.m_texture_mipmap_limit_group_names,
            ),
            crate::opt_field("m_EditorQuality", &self.m_editor_quality),
            crate::opt_field("Fastest", &self.fastest),
            crate::opt_field("Fast", &self.fast),
            crate::opt_field("Simple", &self.simple),
            crate::opt_field("Good", &self.good),
            crate::opt_field("Beautiful", &self.beautiful),
            crate::opt_field("Fantastic", &self.fantastic),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_DefaultStandaloneQuality" => {
                    (self.m_default_standalone_quality).inspect_value(path, offset, limit)
                }
                "m_CurrentQuality" => (self.m_current_quality).inspect_value(path, offset, limit),
                "m_DefaultWebPlayerQuality" => {
                    (self.m_default_web_player_quality).inspect_value(path, offset, limit)
                }
                "m_QualitySettings" => (self.m_quality_settings).inspect_value(path, offset, limit),
                "m_DefaultMobileQuality" => {
                    (self.m_default_mobile_quality).inspect_value(path, offset, limit)
                }
                "m_StrippedMaximumLODLevel" => {
                    (self.m_stripped_maximum_lod_level).inspect_value(path, offset, limit)
                }
                "m_TextureMipmapLimitGroupNames" => {
                    (self.m_texture_mipmap_limit_group_names).inspect_value(path, offset, limit)
                }
                "m_EditorQuality" => (self.m_editor_quality).inspect_value(path, offset, limit),
                "Fastest" => (self.fastest).inspect_value(path, offset, limit),
                "Fast" => (self.fast).inspect_value(path, offset, limit),
                "Simple" => (self.simple).inspect_value(path, offset, limit),
                "Good" => (self.good).inspect_value(path, offset, limit),
                "Beautiful" => (self.beautiful).inspect_value(path, offset, limit),
                "Fantastic" => (self.fantastic).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_default_standalone_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultStandaloneQuality".to_string(), v)),
            ((self.m_current_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_CurrentQuality".to_string(), v)),
            ((self.m_default_web_player_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultWebPlayerQuality".to_string(), v)),
            ((self.m_quality_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_QualitySettings".to_string(), v)),
            ((self.m_default_mobile_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultMobileQuality".to_string(), v)),
            ((self.m_stripped_maximum_lod_level).inspect_value(path, offset, limit))
                .map(|v| ("m_StrippedMaximumLODLevel".to_string(), v)),
            ((self.m_texture_mipmap_limit_group_names).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureMipmapLimitGroupNames".to_string(), v)),
            ((self.m_editor_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_EditorQuality".to_string(), v)),
            ((self.fastest).inspect_value(path, offset, limit)).map(|v| ("Fastest".to_string(), v)),
            ((self.fast).inspect_value(path, offset, limit)).map(|v| ("Fast".to_string(), v)),
            ((self.simple).inspect_value(path, offset, limit)).map(|v| ("Simple".to_string(), v)),
            ((self.good).inspect_value(path, offset, limit)).map(|v| ("Good".to_string(), v)),
            ((self.beautiful).inspect_value(path, offset, limit))
                .map(|v| ("Beautiful".to_string(), v)),
            ((self.fantastic).inspect_value(path, offset, limit))
                .map(|v| ("Fantastic".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
