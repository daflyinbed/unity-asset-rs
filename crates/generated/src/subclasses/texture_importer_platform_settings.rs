//! 自动生成。请勿手动修改。

/// 版本枚举：TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

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

impl crate::ToValue for TextureImporterPlatformSettingsMOverriddenVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_2_20(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
            Self::V2023_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2_20(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TextureImporterPlatformSettings {
    pub m_build_target: String,
    pub m_max_texture_size: i32,
    pub m_format: i32,
    pub m_resize_algorithm: Option<i32>,
    pub m_compression_quality: Option<i32>,
    pub m_texture_compression: Option<i32>,
    pub m_allows_alpha_splitting:
        Option<TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant>,
    pub m_crunched_compression: Option<bool>,
    pub m_overridden: Option<TextureImporterPlatformSettingsMOverriddenVariant>,
    pub m_android_etc2_fallback_override: Option<i32>,
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
        let mut m_format = None;
        let mut m_resize_algorithm = None;
        let mut m_compression_quality = None;
        let mut m_texture_compression = None;
        let mut m_allows_alpha_splitting = None;
        let mut m_crunched_compression = None;
        let mut m_overridden = None;
        let mut m_android_etc2_fallback_override = None;
        let mut m_ignore_platform_support = None;
        let mut m_force_maximum_compression_quality_bc6h_bc7 = None;
        m_build_target = Some(rdr.read_aligned_string()?);
        m_max_texture_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_resize_algorithm = Some(rdr.read_i32()?);
        } else {
        }
        m_format = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_texture_compression = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_crunched_compression = Some(rdr.read_bool()?);
            m_allows_alpha_splitting =
                Some(TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant::parse(rdr, ver)?);
            m_overridden = Some(TextureImporterPlatformSettingsMOverriddenVariant::parse(
                rdr, ver,
            )?);
            m_ignore_platform_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_android_etc2_fallback_override = Some(rdr.read_i32()?);
            m_force_maximum_compression_quality_bc6h_bc7 = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_texture_compression = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_crunched_compression = Some(rdr.read_bool()?);
            m_allows_alpha_splitting =
                Some(TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant::parse(rdr, ver)?);
            m_overridden = Some(TextureImporterPlatformSettingsMOverriddenVariant::parse(
                rdr, ver,
            )?);
            m_android_etc2_fallback_override = Some(rdr.read_i32()?);
            m_force_maximum_compression_quality_bc6h_bc7 = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 20) {
            m_texture_compression = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_crunched_compression = Some(rdr.read_bool()?);
            m_allows_alpha_splitting =
                Some(TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant::parse(rdr, ver)?);
            m_overridden = Some(TextureImporterPlatformSettingsMOverriddenVariant::parse(
                rdr, ver,
            )?);
            m_ignore_platform_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_android_etc2_fallback_override = Some(rdr.read_i32()?);
            m_force_maximum_compression_quality_bc6h_bc7 = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_texture_compression = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_crunched_compression = Some(rdr.read_bool()?);
            m_allows_alpha_splitting =
                Some(TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant::parse(rdr, ver)?);
            m_overridden = Some(TextureImporterPlatformSettingsMOverriddenVariant::parse(
                rdr, ver,
            )?);
            m_android_etc2_fallback_override = Some(rdr.read_i32()?);
            m_force_maximum_compression_quality_bc6h_bc7 = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_texture_compression = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_crunched_compression = Some(rdr.read_bool()?);
            m_allows_alpha_splitting =
                Some(TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant::parse(rdr, ver)?);
            m_overridden = Some(TextureImporterPlatformSettingsMOverriddenVariant::parse(
                rdr, ver,
            )?);
            m_android_etc2_fallback_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_texture_compression = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_crunched_compression = Some(rdr.read_bool()?);
            m_allows_alpha_splitting =
                Some(TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant::parse(rdr, ver)?);
            m_overridden = Some(TextureImporterPlatformSettingsMOverriddenVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_compression_quality = Some(rdr.read_i32()?);
            m_allows_alpha_splitting =
                Some(TextureImporterPlatformSettingsMAllowsAlphaSplittingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_compression_quality = Some(rdr.read_i32()?);
        } else {
        }

        Ok(TextureImporterPlatformSettings {
            m_build_target: m_build_target.unwrap(),
            m_max_texture_size: m_max_texture_size.unwrap(),
            m_format: m_format.unwrap(),
            m_resize_algorithm,
            m_compression_quality,
            m_texture_compression,
            m_allows_alpha_splitting,
            m_crunched_compression,
            m_overridden,
            m_android_etc2_fallback_override,
            m_ignore_platform_support,
            m_force_maximum_compression_quality_bc6h_bc7,
        })
    }
}

impl crate::ToValue for TextureImporterPlatformSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some((
                "m_MaxTextureSize".to_string(),
                self.m_max_texture_size.to_value(),
            )),
            Some(("m_Format".to_string(), self.m_format.to_value())),
            crate::opt_field("m_ResizeAlgorithm", &self.m_resize_algorithm),
            crate::opt_field("m_CompressionQuality", &self.m_compression_quality),
            crate::opt_field("m_TextureCompression", &self.m_texture_compression),
            crate::opt_field("m_AllowsAlphaSplitting", &self.m_allows_alpha_splitting),
            crate::opt_field("m_CrunchedCompression", &self.m_crunched_compression),
            crate::opt_field("m_Overridden", &self.m_overridden),
            crate::opt_field(
                "m_AndroidETC2FallbackOverride",
                &self.m_android_etc2_fallback_override,
            ),
            crate::opt_field("m_IgnorePlatformSupport", &self.m_ignore_platform_support),
            crate::opt_field(
                "m_ForceMaximumCompressionQuality_BC6H_BC7",
                &self.m_force_maximum_compression_quality_bc6h_bc7,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BuildTarget" => (self.m_build_target).inspect_value(path, offset, limit),
                "m_MaxTextureSize" => (self.m_max_texture_size).inspect_value(path, offset, limit),
                "m_Format" => (self.m_format).inspect_value(path, offset, limit),
                "m_ResizeAlgorithm" => (self.m_resize_algorithm).inspect_value(path, offset, limit),
                "m_CompressionQuality" => {
                    (self.m_compression_quality).inspect_value(path, offset, limit)
                }
                "m_TextureCompression" => {
                    (self.m_texture_compression).inspect_value(path, offset, limit)
                }
                "m_AllowsAlphaSplitting" => {
                    (self.m_allows_alpha_splitting).inspect_value(path, offset, limit)
                }
                "m_CrunchedCompression" => {
                    (self.m_crunched_compression).inspect_value(path, offset, limit)
                }
                "m_Overridden" => (self.m_overridden).inspect_value(path, offset, limit),
                "m_AndroidETC2FallbackOverride" => {
                    (self.m_android_etc2_fallback_override).inspect_value(path, offset, limit)
                }
                "m_IgnorePlatformSupport" => {
                    (self.m_ignore_platform_support).inspect_value(path, offset, limit)
                }
                "m_ForceMaximumCompressionQuality_BC6H_BC7" => (self
                    .m_force_maximum_compression_quality_bc6h_bc7)
                    .inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_max_texture_size).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxTextureSize".to_string(), v)),
            ((self.m_format).inspect_value(path, offset, limit))
                .map(|v| ("m_Format".to_string(), v)),
            ((self.m_resize_algorithm).inspect_value(path, offset, limit))
                .map(|v| ("m_ResizeAlgorithm".to_string(), v)),
            ((self.m_compression_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_CompressionQuality".to_string(), v)),
            ((self.m_texture_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureCompression".to_string(), v)),
            ((self.m_allows_alpha_splitting).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowsAlphaSplitting".to_string(), v)),
            ((self.m_crunched_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_CrunchedCompression".to_string(), v)),
            ((self.m_overridden).inspect_value(path, offset, limit))
                .map(|v| ("m_Overridden".to_string(), v)),
            ((self.m_android_etc2_fallback_override).inspect_value(path, offset, limit))
                .map(|v| ("m_AndroidETC2FallbackOverride".to_string(), v)),
            ((self.m_ignore_platform_support).inspect_value(path, offset, limit))
                .map(|v| ("m_IgnorePlatformSupport".to_string(), v)),
            ((self.m_force_maximum_compression_quality_bc6h_bc7)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_ForceMaximumCompressionQuality_BC6H_BC7".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
