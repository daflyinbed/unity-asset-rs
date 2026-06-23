//! 自动生成。请勿手动修改。

use crate::subclasses::streamed_resource::StreamedResource;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct VideoClip {
    pub parent: NamedObject,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_original_path = r.read_aligned_string()?;
        r.align(4)?;
        let m_proxy_width = r.read_u32()?;
        let m_proxy_height = r.read_u32()?;
        let width = r.read_u32()?;
        let height = r.read_u32()?;
        let m_frame_rate = r.read_f64()?;
        let m_frame_count = r.read_u64()?;
        let m_format = r.read_i32()?;
        let m_audio_channel_count = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_audio_sample_rate = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_audio_language = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_external_resources = StreamedResource::parse(r, ver)?;
        r.align(4)?;
        let m_has_split_alpha = r.read_bool()?;

        Ok(VideoClip {
            parent,
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
