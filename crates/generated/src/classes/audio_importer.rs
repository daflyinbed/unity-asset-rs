//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::output::Output;
use crate::subclasses::preview_data::PreviewData;
use crate::subclasses::sample_settings::SampleSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct AudioImporter {
    pub parent: AssetImporter,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_format: Option<i32>,
    pub m_default_settings: Option<SampleSettings>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_quality: Option<f32>,
    pub m_platform_setting_overrides: Option<Vec<(i32, SampleSettings)>>,
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
    pub m_output: Option<Output>,
    pub m_user_data: Option<String>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for AudioImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_format = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_default_settings = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(SampleSettings::parse(r, ver)?)
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
        let m_old_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_quality = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_platform_setting_overrides = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = SampleSettings::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
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
        let m_new_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_stream = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_force_to_mono = r.read_bool()?;
        let m_preview_data_length = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_3d = r.read_bool()?;
        let m_preload_audio_data = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_normalize = if *ver >= crate::UnityVersion::new(5, 1, 2) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let audio_preview_data = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_load_in_background = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_use_hardware = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_loopable = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_preview_data = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(PreviewData::parse(r, ver)?)
        } else {
            None
        };
        let m_ambisonic = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_output = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = Output::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_user_data = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_name = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_variant = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AudioImporter {
            parent,
            m_name,
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
            m_force_to_mono,
            m_preview_data_length,
            m_3d,
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
