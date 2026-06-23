//! 自动生成。请勿手动修改。

use crate::subclasses::shader_bind_channel::ShaderBindChannel;

#[derive(Debug, Clone)]
pub struct ParserBindChannels {
    pub m_channels: Vec<ShaderBindChannel>,
    pub m_source_map: u32,
}

impl crate::Parse for ParserBindChannels {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_channels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ShaderBindChannel::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_source_map = r.read_u32()?;

        Ok(ParserBindChannels {
            m_channels,
            m_source_map,
        })
    }
}
