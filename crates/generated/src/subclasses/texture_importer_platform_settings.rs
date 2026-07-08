//! 自动生成。请勿手动修改。


/// 版本枚举：TextureImporterPlatformSettingsMOverriddenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterPlatformSettingsMOverriddenVariant {
    Default(bool),
    V2022_2_20(bool),
    V2023_1(bool),
    V2023_2(bool),
}

impl crate::Parse for TextureImporterPlatformSettingsMOverriddenVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_2(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(2022, 2, 20) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_2_20(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TextureImporterPlatformSettings {
    pub m_build_target: String,
    pub m_max_texture_size: i32,
    pub m_resize_algorithm: i32,
    pub m_texture_format: i32,
    pub m_texture_compression: i32,
    pub m_compression_quality: i32,
    pub m_crunched_compression: bool,
    pub m_allows_alpha_splitting: bool,
    pub m_overridden: TextureImporterPlatformSettingsMOverriddenVariant,
    pub m_android_etc2_fallback_override: i32,
    pub m_ignore_platform_support: Option<bool>,
    pub m_force_maximum_compression_quality_bc6h_bc7: Option<bool>,
}

impl crate::Parse for TextureImporterPlatformSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_build_target = None;
        let mut m_max_texture_size = None;
        let mut m_resize_algorithm = None;
        let mut m_texture_format = None;
        let mut m_texture_compression = None;
        let mut m_compression_quality = None;
        let mut m_crunched_compression = None;
        let mut m_allows_alpha_splitting = None;
        let mut m_overridden = None;
        let mut m_android_etc2_fallback_override = None;
        let mut m_ignore_platform_support = None;
        let mut m_force_maximum_compression_quality_bc6h_bc7 = None;
            m_build_target = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_max_texture_size = Some(rdr.read_i32()?);
            m_resize_algorithm = Some(rdr.read_i32()?);
            m_texture_format = Some(rdr.read_i32()?);
            m_texture_compression = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_crunched_compression = Some(rdr.read_bool()?);
            m_allows_alpha_splitting = Some(rdr.read_bool()?);
            m_overridden = Some(TextureImporterPlatformSettingsMOverriddenVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_ignore_platform_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2022, 2, 20) {
            m_ignore_platform_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_android_etc2_fallback_override = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_force_maximum_compression_quality_bc6h_bc7 = Some(rdr.read_bool()?);
        } else {
        }

        Ok(TextureImporterPlatformSettings {
            m_build_target: m_build_target.unwrap(),
            m_max_texture_size: m_max_texture_size.unwrap(),
            m_resize_algorithm: m_resize_algorithm.unwrap(),
            m_texture_format: m_texture_format.unwrap(),
            m_texture_compression: m_texture_compression.unwrap(),
            m_compression_quality: m_compression_quality.unwrap(),
            m_crunched_compression: m_crunched_compression.unwrap(),
            m_allows_alpha_splitting: m_allows_alpha_splitting.unwrap(),
            m_overridden: m_overridden.unwrap(),
            m_android_etc2_fallback_override: m_android_etc2_fallback_override.unwrap(),
            m_ignore_platform_support,
            m_force_maximum_compression_quality_bc6h_bc7,
        })
    }
}
