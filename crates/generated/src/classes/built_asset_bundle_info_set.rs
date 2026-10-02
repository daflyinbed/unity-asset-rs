//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::built_asset_bundle_info::BuiltAssetBundleInfo;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::video_clip_importer_output::VideoClipImporterOutput;
use crate::subclasses::video_clip_importer_target_settings::VideoClipImporterTargetSettings;

/// 版本枚举：BuiltAssetBundleInfoSetMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuiltAssetBundleInfoSetMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for BuiltAssetBundleInfoSetMExternalObjectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BuiltAssetBundleInfoSetMExternalObjectsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：BuiltAssetBundleInfoSetMFlipHorizontalVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuiltAssetBundleInfoSetMFlipHorizontalVariant {
    Default(bool),
    V2017_2(bool),
    V2017_2_1(bool),
}

impl crate::Parse for BuiltAssetBundleInfoSetMFlipHorizontalVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 1) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_2_1(v))
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BuiltAssetBundleInfoSetMFlipHorizontalVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_2(v) => v.to_value(),
            Self::V2017_2_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_2_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：BuiltAssetBundleInfoSetMTargetSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuiltAssetBundleInfoSetMTargetSettingsVariant {
    Default(Vec<(i32, VideoClipImporterTargetSettings)>),
    V2023_3(Vec<(String, VideoClipImporterTargetSettings)>),
}

impl crate::Parse for BuiltAssetBundleInfoSetMTargetSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = VideoClipImporterTargetSettings::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V2023_3(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_i32()?;
                            let b = VideoClipImporterTargetSettings::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BuiltAssetBundleInfoSetMTargetSettingsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfoSet {
    pub bundle_infos: Option<Vec<BuiltAssetBundleInfo>>,
    pub m_name: Option<String>,
    pub m_use_legacy_importer: Option<bool>,
    pub m_external_objects: Option<BuiltAssetBundleInfoSetMExternalObjectsVariant>,
    pub m_quality: Option<f32>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_is_color_linear: Option<bool>,
    pub m_frame_range: Option<i32>,
    pub m_original_width: Option<i32>,
    pub m_start_frame: Option<i32>,
    pub m_original_height: Option<i32>,
    pub m_end_frame: Option<i32>,
    pub m_color_space: Option<i32>,
    pub m_deinterlace: Option<i32>,
    pub m_encode_alpha: Option<bool>,
    pub m_frame_count: Option<i32>,
    pub m_flip_vertical: Option<bool>,
    pub m_frame_rate: Option<f64>,
    pub m_flip_horizontal: Option<BuiltAssetBundleInfoSetMFlipHorizontalVariant>,
    pub m_import_audio: Option<bool>,
    pub m_target_settings: Option<BuiltAssetBundleInfoSetMTargetSettingsVariant>,
    pub m_source_file_size: Option<u64>,
    pub m_output: Option<VideoClipImporterOutput>,
    pub m_user_data: Option<String>,
    pub m_source_has_alpha: Option<bool>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_source_audio_channel_count: Option<Vec<u16>>,
    pub m_pixel_aspect_ratio_numerator: Option<u32>,
    pub m_source_audio_sample_rate: Option<Vec<u32>>,
    pub m_pixel_aspect_ratio_denominator: Option<u32>,
}

impl crate::Parse for BuiltAssetBundleInfoSet {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut bundle_infos = None;
        let mut m_name = None;
        let mut m_use_legacy_importer = None;
        let mut m_external_objects = None;
        let mut m_quality = None;
        let mut m_used_file_i_ds = None;
        let mut m_is_color_linear = None;
        let mut m_frame_range = None;
        let mut m_original_width = None;
        let mut m_start_frame = None;
        let mut m_original_height = None;
        let mut m_end_frame = None;
        let mut m_color_space = None;
        let mut m_deinterlace = None;
        let mut m_encode_alpha = None;
        let mut m_frame_count = None;
        let mut m_flip_vertical = None;
        let mut m_frame_rate = None;
        let mut m_flip_horizontal = None;
        let mut m_import_audio = None;
        let mut m_target_settings = None;
        let mut m_source_file_size = None;
        let mut m_output = None;
        let mut m_user_data = None;
        let mut m_source_has_alpha = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
        let mut m_source_audio_channel_count = None;
        let mut m_pixel_aspect_ratio_numerator = None;
        let mut m_source_audio_sample_rate = None;
        let mut m_pixel_aspect_ratio_denominator = None;
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_name = Some(rdr.read_aligned_string()?);
            m_external_objects = Some(BuiltAssetBundleInfoSetMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_frame_range = Some(rdr.read_i32()?);
            m_start_frame = Some(rdr.read_i32()?);
            m_end_frame = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
            m_deinterlace = Some(rdr.read_i32()?);
            m_encode_alpha = Some(rdr.read_bool()?);
            m_flip_vertical = Some(rdr.read_bool()?);
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(
                rdr, ver,
            )?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(
                rdr, ver,
            )?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_name = Some(rdr.read_aligned_string()?);
            m_external_objects = Some(BuiltAssetBundleInfoSetMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_use_legacy_importer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_quality = Some(rdr.read_f32()?);
            m_is_color_linear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_frame_range = Some(rdr.read_i32()?);
            m_start_frame = Some(rdr.read_i32()?);
            m_end_frame = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
            m_deinterlace = Some(rdr.read_i32()?);
            m_encode_alpha = Some(rdr.read_bool()?);
            m_flip_vertical = Some(rdr.read_bool()?);
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(
                rdr, ver,
            )?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(
                rdr, ver,
            )?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 1) {
            m_name = Some(rdr.read_aligned_string()?);
            m_external_objects = Some(BuiltAssetBundleInfoSetMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_use_legacy_importer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_quality = Some(rdr.read_f32()?);
            m_is_color_linear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_frame_range = Some(rdr.read_i32()?);
            m_start_frame = Some(rdr.read_i32()?);
            m_end_frame = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
            m_deinterlace = Some(rdr.read_i32()?);
            m_encode_alpha = Some(rdr.read_bool()?);
            m_flip_vertical = Some(rdr.read_bool()?);
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(
                rdr, ver,
            )?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(
                rdr, ver,
            )?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_name = Some(rdr.read_aligned_string()?);
            m_external_objects = Some(BuiltAssetBundleInfoSetMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_use_legacy_importer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_quality = Some(rdr.read_f32()?);
            m_is_color_linear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_original_width = Some(rdr.read_i32()?);
            m_original_height = Some(rdr.read_i32()?);
            m_frame_range = Some(rdr.read_i32()?);
            m_start_frame = Some(rdr.read_i32()?);
            m_end_frame = Some(rdr.read_i32()?);
            m_frame_count = Some(rdr.read_i32()?);
            m_frame_rate = Some(rdr.read_f64()?);
            m_color_space = Some(rdr.read_i32()?);
            m_deinterlace = Some(rdr.read_i32()?);
            m_source_file_size = Some(rdr.read_u64()?);
            m_encode_alpha = Some(rdr.read_bool()?);
            m_source_has_alpha = Some(rdr.read_bool()?);
            m_flip_vertical = Some(rdr.read_bool()?);
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(
                rdr, ver,
            )?);
            m_pixel_aspect_ratio_numerator = Some(rdr.read_u32()?);
            m_pixel_aspect_ratio_denominator = Some(rdr.read_u32()?);
            rdr.align(4)?;
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_source_audio_channel_count = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_source_audio_sample_rate = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(
                rdr, ver,
            )?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 3) {
            m_name = Some(rdr.read_aligned_string()?);
            m_use_legacy_importer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_quality = Some(rdr.read_f32()?);
            m_is_color_linear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_frame_range = Some(rdr.read_i32()?);
            m_start_frame = Some(rdr.read_i32()?);
            m_end_frame = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
            m_deinterlace = Some(rdr.read_i32()?);
            m_encode_alpha = Some(rdr.read_bool()?);
            m_flip_vertical = Some(rdr.read_bool()?);
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(
                rdr, ver,
            )?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(
                rdr, ver,
            )?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_name = Some(rdr.read_aligned_string()?);
            m_use_legacy_importer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_quality = Some(rdr.read_f32()?);
            m_is_color_linear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_original_width = Some(rdr.read_i32()?);
            m_original_height = Some(rdr.read_i32()?);
            m_frame_range = Some(rdr.read_i32()?);
            m_start_frame = Some(rdr.read_i32()?);
            m_end_frame = Some(rdr.read_i32()?);
            m_frame_count = Some(rdr.read_i32()?);
            m_frame_rate = Some(rdr.read_f64()?);
            m_color_space = Some(rdr.read_i32()?);
            m_deinterlace = Some(rdr.read_i32()?);
            m_source_file_size = Some(rdr.read_u64()?);
            m_encode_alpha = Some(rdr.read_bool()?);
            m_source_has_alpha = Some(rdr.read_bool()?);
            m_flip_vertical = Some(rdr.read_bool()?);
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(
                rdr, ver,
            )?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_source_audio_channel_count = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_source_audio_sample_rate = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(
                rdr, ver,
            )?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else {
            bundle_infos = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| BuiltAssetBundleInfo::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }

        Ok(BuiltAssetBundleInfoSet {
            bundle_infos,
            m_name,
            m_use_legacy_importer,
            m_external_objects,
            m_quality,
            m_used_file_i_ds,
            m_is_color_linear,
            m_frame_range,
            m_original_width,
            m_start_frame,
            m_original_height,
            m_end_frame,
            m_color_space,
            m_deinterlace,
            m_encode_alpha,
            m_frame_count,
            m_flip_vertical,
            m_frame_rate,
            m_flip_horizontal,
            m_import_audio,
            m_target_settings,
            m_source_file_size,
            m_output,
            m_user_data,
            m_source_has_alpha,
            m_asset_bundle_name,
            m_asset_bundle_variant,
            m_source_audio_channel_count,
            m_pixel_aspect_ratio_numerator,
            m_source_audio_sample_rate,
            m_pixel_aspect_ratio_denominator,
        })
    }
}

impl crate::ToValue for BuiltAssetBundleInfoSet {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("bundleInfos", &self.bundle_infos),
            crate::opt_field("m_Name", &self.m_name),
            crate::opt_field("m_UseLegacyImporter", &self.m_use_legacy_importer),
            crate::opt_field("m_ExternalObjects", &self.m_external_objects),
            crate::opt_field("m_Quality", &self.m_quality),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            crate::opt_field("m_IsColorLinear", &self.m_is_color_linear),
            crate::opt_field("m_FrameRange", &self.m_frame_range),
            crate::opt_field("m_OriginalWidth", &self.m_original_width),
            crate::opt_field("m_StartFrame", &self.m_start_frame),
            crate::opt_field("m_OriginalHeight", &self.m_original_height),
            crate::opt_field("m_EndFrame", &self.m_end_frame),
            crate::opt_field("m_ColorSpace", &self.m_color_space),
            crate::opt_field("m_Deinterlace", &self.m_deinterlace),
            crate::opt_field("m_EncodeAlpha", &self.m_encode_alpha),
            crate::opt_field("m_FrameCount", &self.m_frame_count),
            crate::opt_field("m_FlipVertical", &self.m_flip_vertical),
            crate::opt_field("m_FrameRate", &self.m_frame_rate),
            crate::opt_field("m_FlipHorizontal", &self.m_flip_horizontal),
            crate::opt_field("m_ImportAudio", &self.m_import_audio),
            crate::opt_field("m_TargetSettings", &self.m_target_settings),
            crate::opt_field("m_SourceFileSize", &self.m_source_file_size),
            crate::opt_field("m_Output", &self.m_output),
            crate::opt_field("m_UserData", &self.m_user_data),
            crate::opt_field("m_SourceHasAlpha", &self.m_source_has_alpha),
            crate::opt_field("m_AssetBundleName", &self.m_asset_bundle_name),
            crate::opt_field("m_AssetBundleVariant", &self.m_asset_bundle_variant),
            crate::opt_field(
                "m_SourceAudioChannelCount",
                &self.m_source_audio_channel_count,
            ),
            crate::opt_field(
                "m_PixelAspectRatioNumerator",
                &self.m_pixel_aspect_ratio_numerator,
            ),
            crate::opt_field("m_SourceAudioSampleRate", &self.m_source_audio_sample_rate),
            crate::opt_field(
                "m_PixelAspectRatioDenominator",
                &self.m_pixel_aspect_ratio_denominator,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "bundleInfos" => (self.bundle_infos).inspect_value(path, offset, limit),
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_UseLegacyImporter" => {
                    (self.m_use_legacy_importer).inspect_value(path, offset, limit)
                }
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_Quality" => (self.m_quality).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_IsColorLinear" => (self.m_is_color_linear).inspect_value(path, offset, limit),
                "m_FrameRange" => (self.m_frame_range).inspect_value(path, offset, limit),
                "m_OriginalWidth" => (self.m_original_width).inspect_value(path, offset, limit),
                "m_StartFrame" => (self.m_start_frame).inspect_value(path, offset, limit),
                "m_OriginalHeight" => (self.m_original_height).inspect_value(path, offset, limit),
                "m_EndFrame" => (self.m_end_frame).inspect_value(path, offset, limit),
                "m_ColorSpace" => (self.m_color_space).inspect_value(path, offset, limit),
                "m_Deinterlace" => (self.m_deinterlace).inspect_value(path, offset, limit),
                "m_EncodeAlpha" => (self.m_encode_alpha).inspect_value(path, offset, limit),
                "m_FrameCount" => (self.m_frame_count).inspect_value(path, offset, limit),
                "m_FlipVertical" => (self.m_flip_vertical).inspect_value(path, offset, limit),
                "m_FrameRate" => (self.m_frame_rate).inspect_value(path, offset, limit),
                "m_FlipHorizontal" => (self.m_flip_horizontal).inspect_value(path, offset, limit),
                "m_ImportAudio" => (self.m_import_audio).inspect_value(path, offset, limit),
                "m_TargetSettings" => (self.m_target_settings).inspect_value(path, offset, limit),
                "m_SourceFileSize" => (self.m_source_file_size).inspect_value(path, offset, limit),
                "m_Output" => (self.m_output).inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_SourceHasAlpha" => (self.m_source_has_alpha).inspect_value(path, offset, limit),
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                "m_SourceAudioChannelCount" => {
                    (self.m_source_audio_channel_count).inspect_value(path, offset, limit)
                }
                "m_PixelAspectRatioNumerator" => {
                    (self.m_pixel_aspect_ratio_numerator).inspect_value(path, offset, limit)
                }
                "m_SourceAudioSampleRate" => {
                    (self.m_source_audio_sample_rate).inspect_value(path, offset, limit)
                }
                "m_PixelAspectRatioDenominator" => {
                    (self.m_pixel_aspect_ratio_denominator).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.bundle_infos).inspect_value(path, offset, limit))
                .map(|v| ("bundleInfos".to_string(), v)),
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_use_legacy_importer).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLegacyImporter".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_Quality".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_is_color_linear).inspect_value(path, offset, limit))
                .map(|v| ("m_IsColorLinear".to_string(), v)),
            ((self.m_frame_range).inspect_value(path, offset, limit))
                .map(|v| ("m_FrameRange".to_string(), v)),
            ((self.m_original_width).inspect_value(path, offset, limit))
                .map(|v| ("m_OriginalWidth".to_string(), v)),
            ((self.m_start_frame).inspect_value(path, offset, limit))
                .map(|v| ("m_StartFrame".to_string(), v)),
            ((self.m_original_height).inspect_value(path, offset, limit))
                .map(|v| ("m_OriginalHeight".to_string(), v)),
            ((self.m_end_frame).inspect_value(path, offset, limit))
                .map(|v| ("m_EndFrame".to_string(), v)),
            ((self.m_color_space).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorSpace".to_string(), v)),
            ((self.m_deinterlace).inspect_value(path, offset, limit))
                .map(|v| ("m_Deinterlace".to_string(), v)),
            ((self.m_encode_alpha).inspect_value(path, offset, limit))
                .map(|v| ("m_EncodeAlpha".to_string(), v)),
            ((self.m_frame_count).inspect_value(path, offset, limit))
                .map(|v| ("m_FrameCount".to_string(), v)),
            ((self.m_flip_vertical).inspect_value(path, offset, limit))
                .map(|v| ("m_FlipVertical".to_string(), v)),
            ((self.m_frame_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_FrameRate".to_string(), v)),
            ((self.m_flip_horizontal).inspect_value(path, offset, limit))
                .map(|v| ("m_FlipHorizontal".to_string(), v)),
            ((self.m_import_audio).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportAudio".to_string(), v)),
            ((self.m_target_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetSettings".to_string(), v)),
            ((self.m_source_file_size).inspect_value(path, offset, limit))
                .map(|v| ("m_SourceFileSize".to_string(), v)),
            ((self.m_output).inspect_value(path, offset, limit))
                .map(|v| ("m_Output".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_source_has_alpha).inspect_value(path, offset, limit))
                .map(|v| ("m_SourceHasAlpha".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
            ((self.m_source_audio_channel_count).inspect_value(path, offset, limit))
                .map(|v| ("m_SourceAudioChannelCount".to_string(), v)),
            ((self.m_pixel_aspect_ratio_numerator).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelAspectRatioNumerator".to_string(), v)),
            ((self.m_source_audio_sample_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_SourceAudioSampleRate".to_string(), v)),
            ((self.m_pixel_aspect_ratio_denominator).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelAspectRatioDenominator".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
