//! 自动生成。请勿手动修改。

use crate::PPtr;

/// 版本枚举：MovieTextureMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MovieTextureMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for MovieTextureMDownscaleFallbackVariant {
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

impl crate::ToValue for MovieTextureMDownscaleFallbackVariant {
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
#[derive(Debug, Clone)]
pub struct MovieTexture {
    pub m_name: String,
    pub m_loop: Option<bool>,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_audio_clip: Option<PPtr>,
    pub m_downscale_fallback: Option<MovieTextureMDownscaleFallbackVariant>,
    pub m_movie_data: Option<Vec<u8>>,
    pub m_color_space: Option<i32>,
}

impl crate::Parse for MovieTexture {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_loop = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_audio_clip = None;
        let mut m_downscale_fallback = None;
        let mut m_movie_data = None;
        let mut m_color_space = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(MovieTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(MovieTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(MovieTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_loop = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_audio_clip = Some(crate::PPtr::parse(rdr, ver)?);
            m_movie_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_color_space = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_loop = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_audio_clip = Some(crate::PPtr::parse(rdr, ver)?);
            m_movie_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_color_space = Some(rdr.read_i32()?);
        } else {
            m_loop = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_audio_clip = Some(crate::PPtr::parse(rdr, ver)?);
            m_movie_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }

        Ok(MovieTexture {
            m_name: m_name.unwrap(),
            m_loop,
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_audio_clip,
            m_downscale_fallback,
            m_movie_data,
            m_color_space,
        })
    }
}

impl crate::ToValue for MovieTexture {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_Loop", &self.m_loop),
            crate::opt_field("m_ForcedFallbackFormat", &self.m_forced_fallback_format),
            crate::opt_field(
                "m_IsAlphaChannelOptional",
                &self.m_is_alpha_channel_optional,
            ),
            crate::opt_field("m_AudioClip", &self.m_audio_clip),
            crate::opt_field("m_DownscaleFallback", &self.m_downscale_fallback),
            crate::opt_field("m_MovieData", &self.m_movie_data),
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
                "m_Loop" => (self.m_loop).inspect_value(path, offset, limit),
                "m_ForcedFallbackFormat" => {
                    (self.m_forced_fallback_format).inspect_value(path, offset, limit)
                }
                "m_IsAlphaChannelOptional" => {
                    (self.m_is_alpha_channel_optional).inspect_value(path, offset, limit)
                }
                "m_AudioClip" => (self.m_audio_clip).inspect_value(path, offset, limit),
                "m_DownscaleFallback" => {
                    (self.m_downscale_fallback).inspect_value(path, offset, limit)
                }
                "m_MovieData" => (self.m_movie_data).inspect_value(path, offset, limit),
                "m_ColorSpace" => (self.m_color_space).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_loop).inspect_value(path, offset, limit)).map(|v| ("m_Loop".to_string(), v)),
            ((self.m_forced_fallback_format).inspect_value(path, offset, limit))
                .map(|v| ("m_ForcedFallbackFormat".to_string(), v)),
            ((self.m_is_alpha_channel_optional).inspect_value(path, offset, limit))
                .map(|v| ("m_IsAlphaChannelOptional".to_string(), v)),
            ((self.m_audio_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioClip".to_string(), v)),
            ((self.m_downscale_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_DownscaleFallback".to_string(), v)),
            ((self.m_movie_data).inspect_value(path, offset, limit))
                .map(|v| ("m_MovieData".to_string(), v)),
            ((self.m_color_space).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorSpace".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
