//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::built_asset_bundle_info::BuiltAssetBundleInfo;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::video_clip_importer_output::VideoClipImporterOutput;
use crate::subclasses::video_clip_importer_target_settings::VideoClipImporterTargetSettings;
use super::asset_importer::AssetImporter;
use super::object::Object;

#[derive(Debug, Clone)]
pub enum BuiltAssetBundleInfoSetParent {
    Object(Object),
    AssetImporter(AssetImporter),
}

#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfoSet {
    pub parent: BuiltAssetBundleInfoSetParent,
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
    pub m_flip_horizontal: Option<bool>,
    pub m_import_audio: Option<bool>,
    pub m_target_settings: Option<Vec<(i32, VideoClipImporterTargetSettings)>>,
    pub m_source_file_size: Option<u64>,
    pub m_output: Option<VideoClipImporterOutput>,
    pub m_user_data: Option<String>,
    pub m_source_has_alpha: Option<bool>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_audio_import_mode: Option<i32>,
    pub m_source_audio_channel_count: Option<Vec<u16>>,
    pub m_pixel_aspect_ratio_numerator: Option<u32>,
    pub m_source_audio_sample_rate: Option<Vec<u32>>,
    pub m_pixel_aspect_ratio_denominator: Option<u32>,
}

impl crate::Parse for BuiltAssetBundleInfoSet {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(5, 5, 4) {
            BuiltAssetBundleInfoSetParent::Object(Object::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            BuiltAssetBundleInfoSetParent::AssetImporter(AssetImporter::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let bundle_infos = if *ver >= crate::UnityVersion::new(5, 5, 4) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuiltAssetBundleInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_name = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_legacy_importer = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_external_objects = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_quality = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_color_linear = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_frame_range = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_original_width = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_start_frame = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_original_height = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_end_frame = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_color_space = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_deinterlace = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_encode_alpha = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_frame_count = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_flip_vertical = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_frame_rate = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_f64()?)
        } else {
            None
        };
        let m_flip_horizontal = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_import_audio = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_target_settings = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = VideoClipImporterTargetSettings::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_source_file_size = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_u64()?)
        } else {
            None
        };
        let m_output = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = VideoClipImporterOutput::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_user_data = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_source_has_alpha = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_asset_bundle_name = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_variant = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_audio_import_mode = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_source_audio_channel_count = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_pixel_aspect_ratio_numerator = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_source_audio_sample_rate = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_pixel_aspect_ratio_denominator = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_u32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(BuiltAssetBundleInfoSet {
            parent,
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
            m_audio_import_mode,
            m_source_audio_channel_count,
            m_pixel_aspect_ratio_numerator,
            m_source_audio_sample_rate,
            m_pixel_aspect_ratio_denominator,
        })
    }
}
