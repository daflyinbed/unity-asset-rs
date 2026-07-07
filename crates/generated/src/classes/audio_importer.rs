//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::audio_importer_output::AudioImporterOutput;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::preview_data::PreviewData;
use crate::subclasses::sample_settings::SampleSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct AudioImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_format: Option<i32>,
    pub m_default_settings: Option<SampleSettings>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_quality: Option<f32>,
    pub m_platform_setting_overrides: Option<Vec<(String, SampleSettings)>>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_stream: Option<i32>,
    pub m_force_to_mono: bool,
    pub m_preview_data_length: Option<u32>,
    pub m_3d: bool,
    pub m_preload_audio_data: Option<bool>,
    pub m_normalize: Option<bool>,
    pub audio_preview_data: Option<Vec<u8>>,
    pub m_load_in_background: Option<bool>,
    pub m_use_hardware: Option<bool>,
    pub m_loopable: Option<bool>,
    pub m_preview_data: Option<PreviewData>,
    pub m_ambisonic: Option<bool>,
    pub m_output: Option<AudioImporterOutput>,
    pub m_user_data: Option<String>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for AudioImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_format = None;
        let mut m_default_settings = None;
        let mut m_external_objects = None;
        let mut m_old_hash_identity = None;
        let mut m_quality = None;
        let mut m_platform_setting_overrides = None;
        let mut m_used_file_i_ds = None;
        let mut m_new_hash_identity = None;
        let mut m_stream = None;
        let mut m_force_to_mono = None;
        let mut m_preview_data_length = None;
        let mut m_3d = None;
        let mut m_preload_audio_data = None;
        let mut m_normalize = None;
        let mut audio_preview_data = None;
        let mut m_load_in_background = None;
        let mut m_use_hardware = None;
        let mut m_loopable = None;
        let mut m_preview_data = None;
        let mut m_ambisonic = None;
        let mut m_output = None;
        let mut m_user_data = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_default_settings = Some(SampleSettings::parse(r, ver)?);
            r.align(4)?;
            m_platform_setting_overrides = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = SampleSettings::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_default_settings = Some(SampleSettings::parse(r, ver)?);
            r.align(4)?;
            m_platform_setting_overrides = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = SampleSettings::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_default_settings = Some(SampleSettings::parse(r, ver)?);
            r.align(4)?;
            m_platform_setting_overrides = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = SampleSettings::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_format = Some(r.read_i32()?);
            m_quality = Some(r.read_f32()?);
            m_stream = Some(r.read_i32()?);
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_format = Some(r.read_i32()?);
            m_quality = Some(r.read_f32()?);
            m_stream = Some(r.read_i32()?);
            m_preview_data_length = Some(r.read_u32()?);
            audio_preview_data = Some(( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?);
            r.align(4)?;
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
        } else {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_old_hash_identity = Some(MdFour::parse(r, ver)?);
            r.align(4)?;
            m_new_hash_identity = Some(MdFour::parse(r, ver)?);
            r.align(4)?;
            m_format = Some(r.read_i32()?);
            m_quality = Some(r.read_f32()?);
            m_stream = Some(r.read_i32()?);
            m_preview_data_length = Some(r.read_u32()?);
            audio_preview_data = Some(( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?);
            r.align(4)?;
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
        }
            m_force_to_mono = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_normalize = Some(r.read_bool()?);
            m_load_in_background = Some(r.read_bool()?);
            m_ambisonic = Some(r.read_bool()?);
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
            m_preview_data = Some(PreviewData::parse(r, ver)?);
            r.align(4)?;
            m_output = Some(AudioImporterOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_normalize = Some(r.read_bool()?);
            m_preload_audio_data = Some(r.read_bool()?);
            m_load_in_background = Some(r.read_bool()?);
            m_ambisonic = Some(r.read_bool()?);
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
            m_preview_data = Some(PreviewData::parse(r, ver)?);
            r.align(4)?;
            m_output = Some(AudioImporterOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_normalize = Some(r.read_bool()?);
            m_preload_audio_data = Some(r.read_bool()?);
            m_load_in_background = Some(r.read_bool()?);
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
            m_preview_data = Some(PreviewData::parse(r, ver)?);
            r.align(4)?;
            m_output = Some(AudioImporterOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_normalize = Some(r.read_bool()?);
            m_preload_audio_data = Some(r.read_bool()?);
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
            m_preview_data = Some(PreviewData::parse(r, ver)?);
            r.align(4)?;
            m_output = Some(AudioImporterOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 1, 2) {
            m_normalize = Some(r.read_bool()?);
            m_preload_audio_data = Some(r.read_bool()?);
            m_load_in_background = Some(r.read_bool()?);
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
            m_preview_data = Some(PreviewData::parse(r, ver)?);
            r.align(4)?;
            m_output = Some(AudioImporterOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_preload_audio_data = Some(r.read_bool()?);
            m_load_in_background = Some(r.read_bool()?);
            m_3d = Some(r.read_bool()?);
            r.align(4)?;
            m_preview_data = Some(PreviewData::parse(r, ver)?);
            r.align(4)?;
            m_output = Some(AudioImporterOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_use_hardware = Some(r.read_bool()?);
            m_loopable = Some(r.read_bool()?);
            m_output = Some(AudioImporterOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else {
            m_use_hardware = Some(r.read_bool()?);
            m_loopable = Some(r.read_bool()?);
        }

        Ok(AudioImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_format,
            m_default_settings,
            m_external_objects,
            m_old_hash_identity,
            m_quality,
            m_platform_setting_overrides,
            m_used_file_i_ds,
            m_new_hash_identity,
            m_stream,
            m_force_to_mono: m_force_to_mono.unwrap(),
            m_preview_data_length,
            m_3d: m_3d.unwrap(),
            m_preload_audio_data,
            m_normalize,
            audio_preview_data,
            m_load_in_background,
            m_use_hardware,
            m_loopable,
            m_preview_data,
            m_ambisonic,
            m_output,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
