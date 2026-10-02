//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::texture_parameters::TextureParameters;

/// 版本枚举：ProceduralTextureMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProceduralTextureMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for ProceduralTextureMDownscaleFallbackVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ProceduralTextureMDownscaleFallbackVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ProceduralTextureAlphaSourceIsGrayscaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProceduralTextureAlphaSourceIsGrayscaleVariant {
    Default(bool),
    V5_2(bool),
}

impl crate::Parse for ProceduralTextureAlphaSourceIsGrayscaleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ProceduralTextureAlphaSourceIsGrayscaleVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProceduralTexture {
    pub m_name: String,
    pub m_substance_material: Option<PPtr>,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_substance_texture_uid: Option<u64>,
    pub m_downscale_fallback: Option<ProceduralTextureMDownscaleFallbackVariant>,
    pub r#type: Option<i32>,
    pub alpha_source: Option<i32>,
    pub format: Option<i32>,
    pub m_alpha_source_uid: Option<u64>,
    pub m_texture_parameters: Option<TextureParameters>,
    pub m_texture_settings: Option<GlTextureSettings>,
    pub alpha_source_is_grayscale: Option<ProceduralTextureAlphaSourceIsGrayscaleVariant>,
    pub m_baked_data: Option<Vec<u8>>,
    pub m_mipmaps: Option<i32>,
    pub m_alpha_source_is_inverted: Option<bool>,
    pub m_baked_parameters: Option<TextureParameters>,
    pub m_lightmap_format: Option<i32>,
    pub m_color_space: Option<i32>,
}

impl crate::Parse for ProceduralTexture {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_substance_material = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_substance_texture_uid = None;
        let mut m_downscale_fallback = None;
        let mut r#type = None;
        let mut alpha_source = None;
        let mut format = None;
        let mut m_alpha_source_uid = None;
        let mut m_texture_parameters = None;
        let mut m_texture_settings = None;
        let mut alpha_source_is_grayscale = None;
        let mut m_baked_data = None;
        let mut m_mipmaps = None;
        let mut m_alpha_source_is_inverted = None;
        let mut m_baked_parameters = None;
        let mut m_lightmap_format = None;
        let mut m_color_space = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback =
                Some(ProceduralTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback =
                Some(ProceduralTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback =
                Some(ProceduralTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_substance_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_substance_texture_uid = Some(rdr.read_u64()?);
            r#type = Some(rdr.read_i32()?);
            alpha_source = Some(rdr.read_i32()?);
            m_alpha_source_uid = Some(rdr.read_u64()?);
            alpha_source_is_grayscale = Some(
                ProceduralTextureAlphaSourceIsGrayscaleVariant::parse(rdr, ver)?,
            );
            m_alpha_source_is_inverted = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mipmaps = Some(rdr.read_i32()?);
            format = Some(rdr.read_i32()?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
            m_baked_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_texture_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_lightmap_format = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 1) {
            m_substance_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_substance_texture_uid = Some(rdr.read_u64()?);
            r#type = Some(rdr.read_i32()?);
            alpha_source = Some(rdr.read_i32()?);
            m_alpha_source_uid = Some(rdr.read_u64()?);
            alpha_source_is_grayscale = Some(
                ProceduralTextureAlphaSourceIsGrayscaleVariant::parse(rdr, ver)?,
            );
            m_alpha_source_is_inverted = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mipmaps = Some(rdr.read_i32()?);
            format = Some(rdr.read_i32()?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
            m_baked_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_texture_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_lightmap_format = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_substance_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_substance_texture_uid = Some(rdr.read_u64()?);
            r#type = Some(rdr.read_i32()?);
            alpha_source = Some(rdr.read_i32()?);
            m_alpha_source_uid = Some(rdr.read_u64()?);
            alpha_source_is_grayscale = Some(
                ProceduralTextureAlphaSourceIsGrayscaleVariant::parse(rdr, ver)?,
            );
            m_alpha_source_is_inverted = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mipmaps = Some(rdr.read_i32()?);
            format = Some(rdr.read_i32()?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
            m_baked_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_lightmap_format = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 4) {
            m_substance_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_substance_texture_uid = Some(rdr.read_u64()?);
            r#type = Some(rdr.read_i32()?);
            alpha_source = Some(rdr.read_i32()?);
            m_alpha_source_uid = Some(rdr.read_u64()?);
            alpha_source_is_grayscale = Some(
                ProceduralTextureAlphaSourceIsGrayscaleVariant::parse(rdr, ver)?,
            );
            m_alpha_source_is_inverted = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mipmaps = Some(rdr.read_i32()?);
            format = Some(rdr.read_i32()?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
            m_baked_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_texture_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_lightmap_format = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_substance_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_substance_texture_uid = Some(rdr.read_u64()?);
            r#type = Some(rdr.read_i32()?);
            alpha_source = Some(rdr.read_i32()?);
            m_alpha_source_uid = Some(rdr.read_u64()?);
            alpha_source_is_grayscale = Some(
                ProceduralTextureAlphaSourceIsGrayscaleVariant::parse(rdr, ver)?,
            );
            m_alpha_source_is_inverted = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mipmaps = Some(rdr.read_i32()?);
            format = Some(rdr.read_i32()?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
            m_baked_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_lightmap_format = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_substance_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_substance_texture_uid = Some(rdr.read_u64()?);
            r#type = Some(rdr.read_i32()?);
            alpha_source = Some(rdr.read_i32()?);
            m_alpha_source_uid = Some(rdr.read_u64()?);
            alpha_source_is_grayscale = Some(
                ProceduralTextureAlphaSourceIsGrayscaleVariant::parse(rdr, ver)?,
            );
            m_mipmaps = Some(rdr.read_i32()?);
            format = Some(rdr.read_i32()?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
            m_baked_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_lightmap_format = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_substance_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_substance_texture_uid = Some(rdr.read_u64()?);
            r#type = Some(rdr.read_i32()?);
            alpha_source = Some(rdr.read_i32()?);
            format = Some(rdr.read_i32()?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
            m_baked_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_lightmap_format = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
        } else {
            m_substance_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_substance_texture_uid = Some(rdr.read_u64()?);
            r#type = Some(rdr.read_i32()?);
            alpha_source = Some(rdr.read_i32()?);
            format = Some(rdr.read_i32()?);
            m_texture_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
            m_baked_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_baked_parameters = Some(TextureParameters::parse(rdr, ver)?);
            m_lightmap_format = Some(rdr.read_i32()?);
        }

        Ok(ProceduralTexture {
            m_name: m_name.unwrap(),
            m_substance_material,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_substance_texture_uid,
            m_downscale_fallback,
            r#type,
            alpha_source,
            format,
            m_alpha_source_uid,
            m_texture_parameters,
            m_texture_settings,
            alpha_source_is_grayscale,
            m_baked_data,
            m_mipmaps,
            m_alpha_source_is_inverted,
            m_baked_parameters,
            m_lightmap_format,
            m_color_space,
        })
    }
}

impl crate::ToValue for ProceduralTexture {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_SubstanceMaterial", &self.m_substance_material),
            crate::opt_field("m_ForcedFallbackFormat", &self.m_forced_fallback_format),
            crate::opt_field(
                "m_IsAlphaChannelOptional",
                &self.m_is_alpha_channel_optional,
            ),
            crate::opt_field("m_SubstanceTextureUID", &self.m_substance_texture_uid),
            crate::opt_field("m_DownscaleFallback", &self.m_downscale_fallback),
            crate::opt_field("Type", &self.r#type),
            crate::opt_field("AlphaSource", &self.alpha_source),
            crate::opt_field("Format", &self.format),
            crate::opt_field("m_AlphaSourceUID", &self.m_alpha_source_uid),
            crate::opt_field("m_TextureParameters", &self.m_texture_parameters),
            crate::opt_field("m_TextureSettings", &self.m_texture_settings),
            crate::opt_field("AlphaSourceIsGrayscale", &self.alpha_source_is_grayscale),
            crate::opt_field("m_BakedData", &self.m_baked_data),
            crate::opt_field("m_Mipmaps", &self.m_mipmaps),
            crate::opt_field("m_AlphaSourceIsInverted", &self.m_alpha_source_is_inverted),
            crate::opt_field("m_BakedParameters", &self.m_baked_parameters),
            crate::opt_field("m_LightmapFormat", &self.m_lightmap_format),
            crate::opt_field("m_ColorSpace", &self.m_color_space),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_SubstanceMaterial" => {
                    (self.m_substance_material).inspect_value(path, offset, limit)
                }
                "m_ForcedFallbackFormat" => {
                    (self.m_forced_fallback_format).inspect_value(path, offset, limit)
                }
                "m_IsAlphaChannelOptional" => {
                    (self.m_is_alpha_channel_optional).inspect_value(path, offset, limit)
                }
                "m_SubstanceTextureUID" => {
                    (self.m_substance_texture_uid).inspect_value(path, offset, limit)
                }
                "m_DownscaleFallback" => {
                    (self.m_downscale_fallback).inspect_value(path, offset, limit)
                }
                "Type" => (self.r#type).inspect_value(path, offset, limit),
                "AlphaSource" => (self.alpha_source).inspect_value(path, offset, limit),
                "Format" => (self.format).inspect_value(path, offset, limit),
                "m_AlphaSourceUID" => (self.m_alpha_source_uid).inspect_value(path, offset, limit),
                "m_TextureParameters" => {
                    (self.m_texture_parameters).inspect_value(path, offset, limit)
                }
                "m_TextureSettings" => (self.m_texture_settings).inspect_value(path, offset, limit),
                "AlphaSourceIsGrayscale" => {
                    (self.alpha_source_is_grayscale).inspect_value(path, offset, limit)
                }
                "m_BakedData" => (self.m_baked_data).inspect_value(path, offset, limit),
                "m_Mipmaps" => (self.m_mipmaps).inspect_value(path, offset, limit),
                "m_AlphaSourceIsInverted" => {
                    (self.m_alpha_source_is_inverted).inspect_value(path, offset, limit)
                }
                "m_BakedParameters" => (self.m_baked_parameters).inspect_value(path, offset, limit),
                "m_LightmapFormat" => (self.m_lightmap_format).inspect_value(path, offset, limit),
                "m_ColorSpace" => (self.m_color_space).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_substance_material).inspect_value(path, offset, limit))
                .map(|v| ("m_SubstanceMaterial".to_string(), v)),
            ((self.m_forced_fallback_format).inspect_value(path, offset, limit))
                .map(|v| ("m_ForcedFallbackFormat".to_string(), v)),
            ((self.m_is_alpha_channel_optional).inspect_value(path, offset, limit))
                .map(|v| ("m_IsAlphaChannelOptional".to_string(), v)),
            ((self.m_substance_texture_uid).inspect_value(path, offset, limit))
                .map(|v| ("m_SubstanceTextureUID".to_string(), v)),
            ((self.m_downscale_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_DownscaleFallback".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("Type".to_string(), v)),
            ((self.alpha_source).inspect_value(path, offset, limit))
                .map(|v| ("AlphaSource".to_string(), v)),
            ((self.format).inspect_value(path, offset, limit)).map(|v| ("Format".to_string(), v)),
            ((self.m_alpha_source_uid).inspect_value(path, offset, limit))
                .map(|v| ("m_AlphaSourceUID".to_string(), v)),
            ((self.m_texture_parameters).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureParameters".to_string(), v)),
            ((self.m_texture_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureSettings".to_string(), v)),
            ((self.alpha_source_is_grayscale).inspect_value(path, offset, limit))
                .map(|v| ("AlphaSourceIsGrayscale".to_string(), v)),
            ((self.m_baked_data).inspect_value(path, offset, limit))
                .map(|v| ("m_BakedData".to_string(), v)),
            ((self.m_mipmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_Mipmaps".to_string(), v)),
            ((self.m_alpha_source_is_inverted).inspect_value(path, offset, limit))
                .map(|v| ("m_AlphaSourceIsInverted".to_string(), v)),
            ((self.m_baked_parameters).inspect_value(path, offset, limit))
                .map(|v| ("m_BakedParameters".to_string(), v)),
            ((self.m_lightmap_format).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapFormat".to_string(), v)),
            ((self.m_color_space).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorSpace".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
