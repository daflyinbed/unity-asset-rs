//! 自动生成。请勿手动修改。

/// 版本枚举：WebCamTextureMDownscaleFallbackVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum WebCamTextureMDownscaleFallbackVariant {
    Default(bool),
    V2020_2(bool),
}

impl crate::Parse for WebCamTextureMDownscaleFallbackVariant {
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

impl crate::ToValue for WebCamTextureMDownscaleFallbackVariant {
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
pub struct WebCamTexture {
    pub m_name: String,
    pub m_forced_fallback_format: Option<i32>,
    pub m_is_alpha_channel_optional: Option<bool>,
    pub m_downscale_fallback: Option<WebCamTextureMDownscaleFallbackVariant>,
}

impl crate::Parse for WebCamTexture {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_forced_fallback_format = None;
        let mut m_is_alpha_channel_optional = None;
        let mut m_downscale_fallback = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(WebCamTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
            m_is_alpha_channel_optional = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_forced_fallback_format = Some(rdr.read_i32()?);
            m_downscale_fallback = Some(WebCamTextureMDownscaleFallbackVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(WebCamTexture {
            m_name: m_name.unwrap(),
            m_forced_fallback_format,
            m_is_alpha_channel_optional,
            m_downscale_fallback,
        })
    }
}

impl crate::ToValue for WebCamTexture {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_ForcedFallbackFormat", &self.m_forced_fallback_format),
            crate::opt_field(
                "m_IsAlphaChannelOptional",
                &self.m_is_alpha_channel_optional,
            ),
            crate::opt_field("m_DownscaleFallback", &self.m_downscale_fallback),
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
                "m_ForcedFallbackFormat" => {
                    (self.m_forced_fallback_format).inspect_value(path, offset, limit)
                }
                "m_IsAlphaChannelOptional" => {
                    (self.m_is_alpha_channel_optional).inspect_value(path, offset, limit)
                }
                "m_DownscaleFallback" => {
                    (self.m_downscale_fallback).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_forced_fallback_format).inspect_value(path, offset, limit))
                .map(|v| ("m_ForcedFallbackFormat".to_string(), v)),
            ((self.m_is_alpha_channel_optional).inspect_value(path, offset, limit))
                .map(|v| ("m_IsAlphaChannelOptional".to_string(), v)),
            ((self.m_downscale_fallback).inspect_value(path, offset, limit))
                .map(|v| ("m_DownscaleFallback".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
