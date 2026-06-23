//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::video_clip_importer_output::VideoClipImporterOutput;
use crate::subclasses::video_clip_importer_target_settings::VideoClipImporterTargetSettings;

#[derive(Debug, Clone)]
pub struct VideoClipImporter {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_use_legacy_importer: bool,
    pub m_quality: f32,
    pub m_is_color_linear: bool,
    pub m_original_width: i32,
    pub m_original_height: i32,
    pub m_frame_range: i32,
    pub m_start_frame: i32,
    pub m_end_frame: i32,
    pub m_frame_count: i32,
    pub m_frame_rate: f64,
    pub m_color_space: i32,
    pub m_deinterlace: i32,
    pub m_source_file_size: u64,
    pub m_encode_alpha: bool,
    pub m_source_has_alpha: bool,
    pub m_flip_vertical: bool,
    pub m_flip_horizontal: bool,
    pub m_audio_import_mode: i32,
    pub m_source_audio_channel_count: Vec<u16>,
    pub m_source_audio_sample_rate: Vec<u32>,
    pub m_target_settings: Vec<(i32, VideoClipImporterTargetSettings)>,
    pub m_output: VideoClipImporterOutput,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for VideoClipImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_use_legacy_importer = r.read_bool()?;
        r.align(4)?;
        let m_quality = r.read_f32()?;
        let m_is_color_linear = r.read_bool()?;
        r.align(4)?;
        let m_original_width = r.read_i32()?;
        let m_original_height = r.read_i32()?;
        let m_frame_range = r.read_i32()?;
        let m_start_frame = r.read_i32()?;
        let m_end_frame = r.read_i32()?;
        let m_frame_count = r.read_i32()?;
        let m_frame_rate = r.read_f64()?;
        let m_color_space = r.read_i32()?;
        let m_deinterlace = r.read_i32()?;
        let m_source_file_size = r.read_u64()?;
        let m_encode_alpha = r.read_bool()?;
        let m_source_has_alpha = r.read_bool()?;
        let m_flip_vertical = r.read_bool()?;
        let m_flip_horizontal = r.read_bool()?;
        r.align(4)?;
        let m_audio_import_mode = r.read_i32()?;
        r.align(4)?;
        let m_source_audio_channel_count = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_source_audio_sample_rate = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_target_settings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = VideoClipImporterTargetSettings::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_output = VideoClipImporterOutput::parse(r, ver)?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;

        Ok(VideoClipImporter {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_use_legacy_importer,
            m_quality,
            m_is_color_linear,
            m_original_width,
            m_original_height,
            m_frame_range,
            m_start_frame,
            m_end_frame,
            m_frame_count,
            m_frame_rate,
            m_color_space,
            m_deinterlace,
            m_source_file_size,
            m_encode_alpha,
            m_source_has_alpha,
            m_flip_vertical,
            m_flip_horizontal,
            m_audio_import_mode,
            m_source_audio_channel_count,
            m_source_audio_sample_rate,
            m_target_settings,
            m_output,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
