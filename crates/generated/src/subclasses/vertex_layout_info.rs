//! 自动生成。请勿手动修改。

use crate::subclasses::channel_info::ChannelInfo;

#[derive(Debug, Clone)]
pub struct VertexLayoutInfo {
    pub vertex_channels_info: Vec<ChannelInfo>,
    pub vertex_strides: Vec<u8>,
    pub vertex_stream_count: i32,
}

impl crate::Parse for VertexLayoutInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vertex_channels_info = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ChannelInfo::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let vertex_strides = ({
            let n = rdr.read_i32()? as usize;
            (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let vertex_stream_count = rdr.read_i32()?;

        Ok(VertexLayoutInfo {
            vertex_channels_info,
            vertex_strides,
            vertex_stream_count,
        })
    }
}
