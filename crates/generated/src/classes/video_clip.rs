//! 自动生成。请勿手动修改。

use crate::subclasses::streamed_resource::StreamedResource;

#[derive(Debug, Clone)]
pub struct VideoClip {
    pub m_name: String,
    pub m_original_path: String,
    pub m_proxy_width: u32,
    pub m_proxy_height: u32,
    pub width: u32,
    pub height: u32,
    pub m_frame_rate: f64,
    pub m_frame_count: u64,
    pub m_format: i32,
    pub m_audio_channel_count: Vec<u16>,
    pub m_audio_sample_rate: Vec<u32>,
    pub m_audio_language: Vec<String>,
    pub m_external_resources: StreamedResource,
    pub m_has_split_alpha: bool,
}

impl crate::Parse for VideoClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_original_path = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_proxy_width = rdr.read_u32()?;
        let m_proxy_height = rdr.read_u32()?;
        let width = rdr.read_u32()?;
        let height = rdr.read_u32()?;
        let m_frame_rate = rdr.read_f64()?;
        let m_frame_count = rdr.read_u64()?;
        let m_format = rdr.read_i32()?;
        let m_audio_channel_count = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_audio_sample_rate = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_audio_language = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_external_resources = StreamedResource::parse(rdr, ver)?;
        let m_has_split_alpha = rdr.read_bool()?;

        Ok(VideoClip {
            m_name,
            m_original_path,
            m_proxy_width,
            m_proxy_height,
            width,
            height,
            m_frame_rate,
            m_frame_count,
            m_format,
            m_audio_channel_count,
            m_audio_sample_rate,
            m_audio_language,
            m_external_resources,
            m_has_split_alpha,
        })
    }
}
