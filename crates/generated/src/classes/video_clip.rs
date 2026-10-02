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

impl crate::ToValue for VideoClip {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_OriginalPath".to_string(),
                self.m_original_path.to_value(),
            )),
            Some(("m_ProxyWidth".to_string(), self.m_proxy_width.to_value())),
            Some(("m_ProxyHeight".to_string(), self.m_proxy_height.to_value())),
            Some(("Width".to_string(), self.width.to_value())),
            Some(("Height".to_string(), self.height.to_value())),
            Some(("m_FrameRate".to_string(), self.m_frame_rate.to_value())),
            crate::opt_field("m_PixelAspecRatioNum", &self.m_pixel_aspec_ratio_num),
            Some(("m_FrameCount".to_string(), self.m_frame_count.to_value())),
            crate::opt_field("m_PixelAspecRatioDen", &self.m_pixel_aspec_ratio_den),
            Some(("m_Format".to_string(), self.m_format.to_value())),
            Some((
                "m_AudioChannelCount".to_string(),
                self.m_audio_channel_count.to_value(),
            )),
            Some((
                "m_AudioSampleRate".to_string(),
                self.m_audio_sample_rate.to_value(),
            )),
            Some((
                "m_AudioLanguage".to_string(),
                self.m_audio_language.to_value(),
            )),
            Some((
                "m_ExternalResources".to_string(),
                self.m_external_resources.to_value(),
            )),
            Some((
                "m_HasSplitAlpha".to_string(),
                self.m_has_split_alpha.to_value(),
            )),
            crate::opt_field("m_VideoShaders", &self.m_video_shaders),
            crate::opt_field("m_sRGB", &self.m_s_rgb),
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
                "m_OriginalPath" => (self.m_original_path).inspect_value(path, offset, limit),
                "m_ProxyWidth" => (self.m_proxy_width).inspect_value(path, offset, limit),
                "m_ProxyHeight" => (self.m_proxy_height).inspect_value(path, offset, limit),
                "Width" => (self.width).inspect_value(path, offset, limit),
                "Height" => (self.height).inspect_value(path, offset, limit),
                "m_FrameRate" => (self.m_frame_rate).inspect_value(path, offset, limit),
                "m_PixelAspecRatioNum" => {
                    (self.m_pixel_aspec_ratio_num).inspect_value(path, offset, limit)
                }
                "m_FrameCount" => (self.m_frame_count).inspect_value(path, offset, limit),
                "m_PixelAspecRatioDen" => {
                    (self.m_pixel_aspec_ratio_den).inspect_value(path, offset, limit)
                }
                "m_Format" => (self.m_format).inspect_value(path, offset, limit),
                "m_AudioChannelCount" => {
                    (self.m_audio_channel_count).inspect_value(path, offset, limit)
                }
                "m_AudioSampleRate" => {
                    (self.m_audio_sample_rate).inspect_value(path, offset, limit)
                }
                "m_AudioLanguage" => (self.m_audio_language).inspect_value(path, offset, limit),
                "m_ExternalResources" => {
                    (self.m_external_resources).inspect_value(path, offset, limit)
                }
                "m_HasSplitAlpha" => (self.m_has_split_alpha).inspect_value(path, offset, limit),
                "m_VideoShaders" => (self.m_video_shaders).inspect_value(path, offset, limit),
                "m_sRGB" => (self.m_s_rgb).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_original_path).inspect_value(path, offset, limit))
                .map(|v| ("m_OriginalPath".to_string(), v)),
            ((self.m_proxy_width).inspect_value(path, offset, limit))
                .map(|v| ("m_ProxyWidth".to_string(), v)),
            ((self.m_proxy_height).inspect_value(path, offset, limit))
                .map(|v| ("m_ProxyHeight".to_string(), v)),
            ((self.width).inspect_value(path, offset, limit)).map(|v| ("Width".to_string(), v)),
            ((self.height).inspect_value(path, offset, limit)).map(|v| ("Height".to_string(), v)),
            ((self.m_frame_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_FrameRate".to_string(), v)),
            ((self.m_pixel_aspec_ratio_num).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelAspecRatioNum".to_string(), v)),
            ((self.m_frame_count).inspect_value(path, offset, limit))
                .map(|v| ("m_FrameCount".to_string(), v)),
            ((self.m_pixel_aspec_ratio_den).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelAspecRatioDen".to_string(), v)),
            ((self.m_format).inspect_value(path, offset, limit))
                .map(|v| ("m_Format".to_string(), v)),
            ((self.m_audio_channel_count).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioChannelCount".to_string(), v)),
            ((self.m_audio_sample_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioSampleRate".to_string(), v)),
            ((self.m_audio_language).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioLanguage".to_string(), v)),
            ((self.m_external_resources).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalResources".to_string(), v)),
            ((self.m_has_split_alpha).inspect_value(path, offset, limit))
                .map(|v| ("m_HasSplitAlpha".to_string(), v)),
            ((self.m_video_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_VideoShaders".to_string(), v)),
            ((self.m_s_rgb).inspect_value(path, offset, limit)).map(|v| ("m_sRGB".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
