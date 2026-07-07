//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::streamed_resource::StreamedResource;

#[derive(Debug, Clone)]
pub struct VideoClip_329 {
    pub m_name: String,
    pub m_original_path: String,
    pub m_proxy_width: u32,
    pub m_proxy_height: u32,
    pub width: u32,
    pub height: u32,
    pub m_frame_rate: f64,
    pub m_pixel_aspec_ratio_num: Option<u32>,
    pub m_frame_count: u64,
    pub m_pixel_aspec_ratio_den: Option<u32>,
    pub m_format: i32,
    pub m_audio_channel_count: Vec<u16>,
    pub m_audio_sample_rate: Vec<u32>,
    pub m_audio_language: Vec<String>,
    pub m_external_resources: StreamedResource,
    pub m_has_split_alpha: bool,
    pub m_video_shaders: Option<Vec<PPtr>>,
    pub m_s_rgb: Option<bool>,
}

impl crate::Parse for VideoClip_329 {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_original_path = r.read_aligned_string()?;
        r.align(4)?;
        let m_proxy_width = r.read_u32()?;
        let m_proxy_height = r.read_u32()?;
        let width = r.read_u32()?;
        let height = r.read_u32()?;
        let m_frame_rate = r.read_f64()?;
        let m_pixel_aspec_ratio_num = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_frame_count = r.read_u64()?;
        let m_pixel_aspec_ratio_den = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
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
        let m_video_shaders = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_s_rgb = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(VideoClip_329 {
            m_name,
            m_original_path,
            m_proxy_width,
            m_proxy_height,
            width,
            height,
            m_frame_rate,
            m_pixel_aspec_ratio_num,
            m_frame_count,
            m_pixel_aspec_ratio_den,
            m_format,
            m_audio_channel_count,
            m_audio_sample_rate,
            m_audio_language,
            m_external_resources,
            m_has_split_alpha,
            m_video_shaders,
            m_s_rgb,
        })
    }
}
