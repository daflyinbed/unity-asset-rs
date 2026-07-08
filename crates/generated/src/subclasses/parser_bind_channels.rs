//! 自动生成。请勿手动修改。

use crate::subclasses::shader_bind_channel::ShaderBindChannel;


/// 版本枚举：ParserBindChannelsMChannelsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParserBindChannelsMChannelsVariant {
    Default(Vec<ShaderBindChannel>),
    V2017_1(Vec<ShaderBindChannel>),
}

impl crate::Parse for ParserBindChannelsMChannelsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ShaderBindChannel::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ShaderBindChannel::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

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
#[derive(Debug, Clone)]
pub struct ParserBindChannels {
    pub m_channels: ParserBindChannelsMChannelsVariant,
    pub m_source_map: ParserBindChannelsMSourceMapVariant,
}

impl crate::Parse for ParserBindChannels {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_channels = ParserBindChannelsMChannelsVariant::parse(rdr, ver)?;
        let m_source_map = ParserBindChannelsMSourceMapVariant::parse(rdr, ver)?;

        Ok(ParserBindChannels {
            m_channels,
            m_source_map,
        })
    }
}
