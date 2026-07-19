//! 自动生成。请勿手动修改。

use crate::PPtr;
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

impl crate::Parse for VideoClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_original_path = None;
        let mut m_proxy_width = None;
        let mut m_proxy_height = None;
        let mut width = None;
        let mut height = None;
        let mut m_frame_rate = None;
        let mut m_pixel_aspec_ratio_num = None;
        let mut m_frame_count = None;
        let mut m_pixel_aspec_ratio_den = None;
        let mut m_format = None;
        let mut m_audio_channel_count = None;
        let mut m_audio_sample_rate = None;
        let mut m_audio_language = None;
        let mut m_external_resources = None;
        let mut m_has_split_alpha = None;
        let mut m_video_shaders = None;
        let mut m_s_rgb = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_original_path = Some(rdr.read_aligned_string()?);
        rdr.align(4)?;
        m_proxy_width = Some(rdr.read_u32()?);
        m_proxy_height = Some(rdr.read_u32()?);
        width = Some(rdr.read_u32()?);
        height = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_pixel_aspec_ratio_num = Some(rdr.read_u32()?);
            m_pixel_aspec_ratio_den = Some(rdr.read_u32()?);
        } else {
        }
        m_frame_rate = Some(rdr.read_f64()?);
        m_frame_count = Some(rdr.read_u64()?);
        m_format = Some(rdr.read_i32()?);
        m_audio_channel_count = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u16())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_audio_sample_rate = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_audio_language = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_video_shaders = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        m_external_resources = Some(StreamedResource::parse(rdr, ver)?);
        m_has_split_alpha = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_s_rgb = Some(rdr.read_bool()?);
        } else {
        }

        Ok(VideoClip {
            m_name: m_name.unwrap(),
            m_original_path: m_original_path.unwrap(),
            m_proxy_width: m_proxy_width.unwrap(),
            m_proxy_height: m_proxy_height.unwrap(),
            width: width.unwrap(),
            height: height.unwrap(),
            m_frame_rate: m_frame_rate.unwrap(),
            m_pixel_aspec_ratio_num,
            m_frame_count: m_frame_count.unwrap(),
            m_pixel_aspec_ratio_den,
            m_format: m_format.unwrap(),
            m_audio_channel_count: m_audio_channel_count.unwrap(),
            m_audio_sample_rate: m_audio_sample_rate.unwrap(),
            m_audio_language: m_audio_language.unwrap(),
            m_external_resources: m_external_resources.unwrap(),
            m_has_split_alpha: m_has_split_alpha.unwrap(),
            m_video_shaders,
            m_s_rgb,
        })
    }
}
