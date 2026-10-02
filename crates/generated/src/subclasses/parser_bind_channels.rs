//! 自动生成。请勿手动修改。

use crate::subclasses::shader_bind_channel::ShaderBindChannel;

/// 版本枚举：ParserBindChannelsMSourceMapVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParserBindChannelsMSourceMapVariant {
    Default(u32),
    V5_6(i32),
}

impl crate::Parse for ParserBindChannelsMSourceMapVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ParserBindChannelsMSourceMapVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ParserBindChannels {
    pub m_channels: Vec<ShaderBindChannel>,
    pub m_source_map: ParserBindChannelsMSourceMapVariant,
}

impl crate::Parse for ParserBindChannels {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_channels = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ShaderBindChannel::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_source_map = ParserBindChannelsMSourceMapVariant::parse(rdr, ver)?;

        Ok(ParserBindChannels {
            m_channels,
            m_source_map,
        })
    }
}

impl crate::ToValue for ParserBindChannels {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Channels".to_string(), self.m_channels.to_value())),
            Some(("m_SourceMap".to_string(), self.m_source_map.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Channels" => (self.m_channels).inspect_value(path, offset, limit),
                "m_SourceMap" => (self.m_source_map).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_channels).inspect_value(path, offset, limit))
                .map(|v| ("m_Channels".to_string(), v)),
            ((self.m_source_map).inspect_value(path, offset, limit))
                .map(|v| ("m_SourceMap".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
