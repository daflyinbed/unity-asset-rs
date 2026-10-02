//! 自动生成。请勿手动修改。

use crate::subclasses::streamed_resource::StreamedResource;
use crate::subclasses::video_clip_importer_target_settings::VideoClipImporterTargetSettings;

/// 版本枚举：VideoClipImporterOutputSourceHasAlphaVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VideoClipImporterOutputSourceHasAlphaVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for VideoClipImporterOutputSourceHasAlphaVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VideoClipImporterOutputSourceHasAlphaVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VideoClipImporterOutput {
    pub format: Option<i32>,
    pub source_frame_rate: Option<f64>,
    pub source_pixel_aspect_ratio_numerator: Option<u32>,
    pub settings: Option<VideoClipImporterTargetSettings>,
    pub source_file_size: Option<u64>,
    pub source_pixel_aspect_ratio_denominator: Option<u32>,
    pub encoded_width: Option<i32>,
    pub source_audio_channel_count: Option<Vec<u16>>,
    pub encoded_height: Option<i32>,
    pub source_has_alpha: Option<VideoClipImporterOutputSourceHasAlphaVariant>,
    pub encoded_start_frame: Option<i32>,
    pub source_audio_sample_rate: Option<Vec<u32>>,
    pub encoded_end_frame: Option<i32>,
    pub original_width: Option<i32>,
    pub streamed_resource: Option<StreamedResource>,
    pub original_height: Option<i32>,
    pub transcode_skipped: Option<bool>,
    pub original_frame_count: Option<i32>,
    pub encoded_settings: Option<VideoClipImporterTargetSettings>,
}

impl crate::Parse for VideoClipImporterOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut format = None;
        let mut source_frame_rate = None;
        let mut source_pixel_aspect_ratio_numerator = None;
        let mut settings = None;
        let mut source_file_size = None;
        let mut source_pixel_aspect_ratio_denominator = None;
        let mut encoded_width = None;
        let mut source_audio_channel_count = None;
        let mut encoded_height = None;
        let mut source_has_alpha = None;
        let mut encoded_start_frame = None;
        let mut source_audio_sample_rate = None;
        let mut encoded_end_frame = None;
        let mut original_width = None;
        let mut streamed_resource = None;
        let mut original_height = None;
        let mut transcode_skipped = None;
        let mut original_frame_count = None;
        let mut encoded_settings = None;
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            source_pixel_aspect_ratio_numerator = Some(rdr.read_u32()?);
            source_pixel_aspect_ratio_denominator = Some(rdr.read_u32()?);
            source_frame_rate = Some(rdr.read_f64()?);
            source_file_size = Some(rdr.read_u64()?);
            source_audio_channel_count = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            source_has_alpha = Some(VideoClipImporterOutputSourceHasAlphaVariant::parse(
                rdr, ver,
            )?);
            transcode_skipped = Some(rdr.read_bool()?);
            rdr.align(4)?;
            source_audio_sample_rate = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            original_width = Some(rdr.read_i32()?);
            original_height = Some(rdr.read_i32()?);
            original_frame_count = Some(rdr.read_i32()?);
            encoded_settings = Some(VideoClipImporterTargetSettings::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 1) {
            source_pixel_aspect_ratio_numerator = Some(rdr.read_u32()?);
            source_pixel_aspect_ratio_denominator = Some(rdr.read_u32()?);
            source_frame_rate = Some(rdr.read_f64()?);
            source_file_size = Some(rdr.read_u64()?);
            source_audio_channel_count = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            source_has_alpha = Some(VideoClipImporterOutputSourceHasAlphaVariant::parse(
                rdr, ver,
            )?);
            source_audio_sample_rate = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            original_width = Some(rdr.read_i32()?);
            original_height = Some(rdr.read_i32()?);
            original_frame_count = Some(rdr.read_i32()?);
            encoded_settings = Some(VideoClipImporterTargetSettings::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            format = Some(rdr.read_i32()?);
            settings = Some(VideoClipImporterTargetSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            encoded_width = Some(rdr.read_i32()?);
            encoded_height = Some(rdr.read_i32()?);
            encoded_start_frame = Some(rdr.read_i32()?);
            encoded_end_frame = Some(rdr.read_i32()?);
            streamed_resource = Some(StreamedResource::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 3) {
            source_frame_rate = Some(rdr.read_f64()?);
            source_file_size = Some(rdr.read_u64()?);
            source_audio_channel_count = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            source_has_alpha = Some(VideoClipImporterOutputSourceHasAlphaVariant::parse(
                rdr, ver,
            )?);
            source_audio_sample_rate = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            original_width = Some(rdr.read_i32()?);
            original_height = Some(rdr.read_i32()?);
            original_frame_count = Some(rdr.read_i32()?);
            encoded_settings = Some(VideoClipImporterTargetSettings::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
            format = Some(rdr.read_i32()?);
            settings = Some(VideoClipImporterTargetSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            encoded_width = Some(rdr.read_i32()?);
            encoded_height = Some(rdr.read_i32()?);
            encoded_start_frame = Some(rdr.read_i32()?);
            encoded_end_frame = Some(rdr.read_i32()?);
            streamed_resource = Some(StreamedResource::parse(rdr, ver)?);
        }

        Ok(VideoClipImporterOutput {
            format,
            source_frame_rate,
            source_pixel_aspect_ratio_numerator,
            settings,
            source_file_size,
            source_pixel_aspect_ratio_denominator,
            encoded_width,
            source_audio_channel_count,
            encoded_height,
            source_has_alpha,
            encoded_start_frame,
            source_audio_sample_rate,
            encoded_end_frame,
            original_width,
            streamed_resource,
            original_height,
            transcode_skipped,
            original_frame_count,
            encoded_settings,
        })
    }
}

impl crate::ToValue for VideoClipImporterOutput {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("format", &self.format),
            crate::opt_field("sourceFrameRate", &self.source_frame_rate),
            crate::opt_field(
                "sourcePixelAspectRatioNumerator",
                &self.source_pixel_aspect_ratio_numerator,
            ),
            crate::opt_field("settings", &self.settings),
            crate::opt_field("sourceFileSize", &self.source_file_size),
            crate::opt_field(
                "sourcePixelAspectRatioDenominator",
                &self.source_pixel_aspect_ratio_denominator,
            ),
            crate::opt_field("encodedWidth", &self.encoded_width),
            crate::opt_field("sourceAudioChannelCount", &self.source_audio_channel_count),
            crate::opt_field("encodedHeight", &self.encoded_height),
            crate::opt_field("sourceHasAlpha", &self.source_has_alpha),
            crate::opt_field("encodedStartFrame", &self.encoded_start_frame),
            crate::opt_field("sourceAudioSampleRate", &self.source_audio_sample_rate),
            crate::opt_field("encodedEndFrame", &self.encoded_end_frame),
            crate::opt_field("originalWidth", &self.original_width),
            crate::opt_field("streamedResource", &self.streamed_resource),
            crate::opt_field("originalHeight", &self.original_height),
            crate::opt_field("transcodeSkipped", &self.transcode_skipped),
            crate::opt_field("originalFrameCount", &self.original_frame_count),
            crate::opt_field("encodedSettings", &self.encoded_settings),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "format" => (self.format).inspect_value(path, offset, limit),
                "sourceFrameRate" => (self.source_frame_rate).inspect_value(path, offset, limit),
                "sourcePixelAspectRatioNumerator" => {
                    (self.source_pixel_aspect_ratio_numerator).inspect_value(path, offset, limit)
                }
                "settings" => (self.settings).inspect_value(path, offset, limit),
                "sourceFileSize" => (self.source_file_size).inspect_value(path, offset, limit),
                "sourcePixelAspectRatioDenominator" => {
                    (self.source_pixel_aspect_ratio_denominator).inspect_value(path, offset, limit)
                }
                "encodedWidth" => (self.encoded_width).inspect_value(path, offset, limit),
                "sourceAudioChannelCount" => {
                    (self.source_audio_channel_count).inspect_value(path, offset, limit)
                }
                "encodedHeight" => (self.encoded_height).inspect_value(path, offset, limit),
                "sourceHasAlpha" => (self.source_has_alpha).inspect_value(path, offset, limit),
                "encodedStartFrame" => {
                    (self.encoded_start_frame).inspect_value(path, offset, limit)
                }
                "sourceAudioSampleRate" => {
                    (self.source_audio_sample_rate).inspect_value(path, offset, limit)
                }
                "encodedEndFrame" => (self.encoded_end_frame).inspect_value(path, offset, limit),
                "originalWidth" => (self.original_width).inspect_value(path, offset, limit),
                "streamedResource" => (self.streamed_resource).inspect_value(path, offset, limit),
                "originalHeight" => (self.original_height).inspect_value(path, offset, limit),
                "transcodeSkipped" => (self.transcode_skipped).inspect_value(path, offset, limit),
                "originalFrameCount" => {
                    (self.original_frame_count).inspect_value(path, offset, limit)
                }
                "encodedSettings" => (self.encoded_settings).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.format).inspect_value(path, offset, limit)).map(|v| ("format".to_string(), v)),
            ((self.source_frame_rate).inspect_value(path, offset, limit))
                .map(|v| ("sourceFrameRate".to_string(), v)),
            ((self.source_pixel_aspect_ratio_numerator).inspect_value(path, offset, limit))
                .map(|v| ("sourcePixelAspectRatioNumerator".to_string(), v)),
            ((self.settings).inspect_value(path, offset, limit))
                .map(|v| ("settings".to_string(), v)),
            ((self.source_file_size).inspect_value(path, offset, limit))
                .map(|v| ("sourceFileSize".to_string(), v)),
            ((self.source_pixel_aspect_ratio_denominator).inspect_value(path, offset, limit))
                .map(|v| ("sourcePixelAspectRatioDenominator".to_string(), v)),
            ((self.encoded_width).inspect_value(path, offset, limit))
                .map(|v| ("encodedWidth".to_string(), v)),
            ((self.source_audio_channel_count).inspect_value(path, offset, limit))
                .map(|v| ("sourceAudioChannelCount".to_string(), v)),
            ((self.encoded_height).inspect_value(path, offset, limit))
                .map(|v| ("encodedHeight".to_string(), v)),
            ((self.source_has_alpha).inspect_value(path, offset, limit))
                .map(|v| ("sourceHasAlpha".to_string(), v)),
            ((self.encoded_start_frame).inspect_value(path, offset, limit))
                .map(|v| ("encodedStartFrame".to_string(), v)),
            ((self.source_audio_sample_rate).inspect_value(path, offset, limit))
                .map(|v| ("sourceAudioSampleRate".to_string(), v)),
            ((self.encoded_end_frame).inspect_value(path, offset, limit))
                .map(|v| ("encodedEndFrame".to_string(), v)),
            ((self.original_width).inspect_value(path, offset, limit))
                .map(|v| ("originalWidth".to_string(), v)),
            ((self.streamed_resource).inspect_value(path, offset, limit))
                .map(|v| ("streamedResource".to_string(), v)),
            ((self.original_height).inspect_value(path, offset, limit))
                .map(|v| ("originalHeight".to_string(), v)),
            ((self.transcode_skipped).inspect_value(path, offset, limit))
                .map(|v| ("transcodeSkipped".to_string(), v)),
            ((self.original_frame_count).inspect_value(path, offset, limit))
                .map(|v| ("originalFrameCount".to_string(), v)),
            ((self.encoded_settings).inspect_value(path, offset, limit))
                .map(|v| ("encodedSettings".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
