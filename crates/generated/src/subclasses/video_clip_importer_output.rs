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
