//! 自动生成。请勿手动修改。

use crate::subclasses::stream_info::StreamInfo;

#[derive(Debug, Clone)]
pub struct VertexData {
    pub m_current_channels: u32,
    pub m_vertex_count: u32,
    pub m_streams_0: StreamInfo,
    pub m_streams_1: StreamInfo,
    pub m_streams_2: StreamInfo,
    pub m_streams_3: StreamInfo,
    pub m_data_size: Vec<u8>,
}

impl crate::Parse for VertexData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_current_channels = r.read_u32()?;
        let m_vertex_count = r.read_u32()?;
        let m_streams_0 = StreamInfo::parse(r, ver)?;
        let m_streams_1 = StreamInfo::parse(r, ver)?;
        let m_streams_2 = StreamInfo::parse(r, ver)?;
        let m_streams_3 = StreamInfo::parse(r, ver)?;
        let m_data_size = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;

        Ok(VertexData {
            m_current_channels,
            m_vertex_count,
            m_streams_0,
            m_streams_1,
            m_streams_2,
            m_streams_3,
            m_data_size,
        })
    }
}
