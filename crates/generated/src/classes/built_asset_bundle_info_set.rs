//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::built_asset_bundle_info::BuiltAssetBundleInfo;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::video_clip_importer_output::VideoClipImporterOutput;
use crate::subclasses::video_clip_importer_target_settings::VideoClipImporterTargetSettings;


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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = VideoClipImporterTargetSettings::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2023_3(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = VideoClipImporterTargetSettings::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfoSet {
    pub bundle_infos: Option<Vec<BuiltAssetBundleInfo>>,
    pub m_name: Option<String>,
    pub m_use_legacy_importer: Option<bool>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
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
            rdr.align(4)?;
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_frame_range = Some(rdr.read_i32()?);
            m_start_frame = Some(rdr.read_i32()?);
            m_end_frame = Some(rdr.read_i32()?);
            m_color_space = Some(rdr.read_i32()?);
            m_deinterlace = Some(rdr.read_i32()?);
            m_encode_alpha = Some(rdr.read_bool()?);
            m_flip_vertical = Some(rdr.read_bool()?);
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(rdr, ver)?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(rdr, ver)?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
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
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(rdr, ver)?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(rdr, ver)?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 1) {
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
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
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(rdr, ver)?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(rdr, ver)?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
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
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(rdr, ver)?);
            m_pixel_aspect_ratio_numerator = Some(rdr.read_u32()?);
            m_pixel_aspect_ratio_denominator = Some(rdr.read_u32()?);
            rdr.align(4)?;
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_source_audio_channel_count = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_source_audio_sample_rate = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(rdr, ver)?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 3) {
            m_name = Some(rdr.read_aligned_string()?);
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
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(rdr, ver)?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(rdr, ver)?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
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
            m_flip_horizontal = Some(BuiltAssetBundleInfoSetMFlipHorizontalVariant::parse(rdr, ver)?);
            m_import_audio = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_source_audio_channel_count = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_source_audio_sample_rate = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_target_settings = Some(BuiltAssetBundleInfoSetMTargetSettingsVariant::parse(rdr, ver)?);
            m_output = Some(VideoClipImporterOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
            bundle_infos = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuiltAssetBundleInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
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
