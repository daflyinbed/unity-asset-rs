//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::audio_importer_output::AudioImporterOutput;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::output::Output;
use crate::subclasses::preview_data::PreviewData;
use crate::subclasses::sample_settings::SampleSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

/// 版本枚举：AudioImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for AudioImporterMExternalObjectsVariant {
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

impl crate::ToValue for AudioImporterMExternalObjectsVariant {
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

/// 版本枚举：AudioImporterMPlatformSettingOverridesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioImporterMPlatformSettingOverridesVariant {
    Default(Vec<(i32, SampleSettings)>),
    V2023_3(Vec<(String, SampleSettings)>),
}

impl crate::Parse for AudioImporterMPlatformSettingOverridesVariant {
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
                            let b = SampleSettings::parse(rdr, ver)?;
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
                            let b = SampleSettings::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioImporterMPlatformSettingOverridesVariant {
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

/// 版本枚举：AudioImporterM3dVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioImporterM3dVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for AudioImporterM3dVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioImporterM3dVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AudioImporterMOutputVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioImporterMOutputVariant {
    Default(Box<Output>),
    V5_0(Box<AudioImporterOutput>),
}

impl crate::Parse for AudioImporterMOutputVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ({ Ok(Box::new(AudioImporterOutput::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = ({ Ok(Box::new(Output::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioImporterMOutputVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_format: Option<i32>,
    pub m_default_settings: Option<SampleSettings>,
    pub m_external_objects: Option<AudioImporterMExternalObjectsVariant>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_quality: Option<f32>,
    pub m_platform_setting_overrides: Option<AudioImporterMPlatformSettingOverridesVariant>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_stream: Option<i32>,
    pub m_force_to_mono: bool,
    pub m_preview_data_length: Option<u32>,
    pub m_3d: AudioImporterM3dVariant,
    pub m_preload_audio_data: Option<bool>,
    pub m_normalize: Option<bool>,
    pub audio_preview_data: Option<Vec<u8>>,
    pub m_load_in_background: Option<bool>,
    pub m_use_hardware: Option<bool>,
    pub m_loopable: Option<bool>,
    pub m_preview_data: Option<PreviewData>,
    pub m_ambisonic: Option<bool>,
    pub m_output: Option<AudioImporterMOutputVariant>,
    pub m_user_data: Option<String>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for AudioImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(AudioImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_default_settings = Some(SampleSettings::parse(rdr, ver)?);
            m_platform_setting_overrides = Some(
                AudioImporterMPlatformSettingOverridesVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(AudioImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_default_settings = Some(SampleSettings::parse(rdr, ver)?);
            m_platform_setting_overrides = Some(
                AudioImporterMPlatformSettingOverridesVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_default_settings = Some(SampleSettings::parse(rdr, ver)?);
            m_platform_setting_overrides = Some(
                AudioImporterMPlatformSettingOverridesVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_format = Some(rdr.read_i32()?);
            m_quality = Some(rdr.read_f32()?);
            m_stream = Some(rdr.read_i32()?);
            m_3d = Some(AudioImporterM3dVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_format = Some(rdr.read_i32()?);
            m_quality = Some(rdr.read_f32()?);
            m_stream = Some(rdr.read_i32()?);
            m_preview_data_length = Some(rdr.read_u32()?);
            audio_preview_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    rdr.read_bytes(n)
                })?,
            );
            rdr.align(4)?;
            m_3d = Some(AudioImporterM3dVariant::parse(rdr, ver)?);
        } else {
            m_file_id_to_recycle_name = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_format = Some(rdr.read_i32()?);
            m_quality = Some(rdr.read_f32()?);
            m_stream = Some(rdr.read_i32()?);
            m_preview_data_length = Some(rdr.read_u32()?);
            audio_preview_data = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    rdr.read_bytes(n)
                })?,
            );
            rdr.align(4)?;
            m_3d = Some(AudioImporterM3dVariant::parse(rdr, ver)?);
        }
        m_force_to_mono = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_normalize = Some(rdr.read_bool()?);
            m_load_in_background = Some(rdr.read_bool()?);
            m_ambisonic = Some(rdr.read_bool()?);
            m_3d = Some(AudioImporterM3dVariant::parse(rdr, ver)?);
            m_preview_data = Some(PreviewData::parse(rdr, ver)?);
            m_output = Some(AudioImporterMOutputVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_normalize = Some(rdr.read_bool()?);
            m_preload_audio_data = Some(rdr.read_bool()?);
            m_load_in_background = Some(rdr.read_bool()?);
            m_ambisonic = Some(rdr.read_bool()?);
            m_3d = Some(AudioImporterM3dVariant::parse(rdr, ver)?);
            m_preview_data = Some(PreviewData::parse(rdr, ver)?);
            m_output = Some(AudioImporterMOutputVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 2) {
            m_normalize = Some(rdr.read_bool()?);
            m_preload_audio_data = Some(rdr.read_bool()?);
            m_load_in_background = Some(rdr.read_bool()?);
            m_3d = Some(AudioImporterM3dVariant::parse(rdr, ver)?);
            m_preview_data = Some(PreviewData::parse(rdr, ver)?);
            m_output = Some(AudioImporterMOutputVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_preload_audio_data = Some(rdr.read_bool()?);
            m_load_in_background = Some(rdr.read_bool()?);
            m_3d = Some(AudioImporterM3dVariant::parse(rdr, ver)?);
            m_preview_data = Some(PreviewData::parse(rdr, ver)?);
            m_output = Some(AudioImporterMOutputVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_use_hardware = Some(rdr.read_bool()?);
            m_loopable = Some(rdr.read_bool()?);
            m_output = Some(AudioImporterMOutputVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
        } else {
            m_use_hardware = Some(rdr.read_bool()?);
            m_loopable = Some(rdr.read_bool()?);
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

impl crate::ToValue for AudioImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_FileIDToRecycleName", &self.m_file_id_to_recycle_name),
            crate::opt_field("m_Format", &self.m_format),
            crate::opt_field("m_DefaultSettings", &self.m_default_settings),
            crate::opt_field("m_ExternalObjects", &self.m_external_objects),
            crate::opt_field("m_OldHashIdentity", &self.m_old_hash_identity),
            crate::opt_field("m_Quality", &self.m_quality),
            crate::opt_field(
                "m_PlatformSettingOverrides",
                &self.m_platform_setting_overrides,
            ),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            crate::opt_field("m_NewHashIdentity", &self.m_new_hash_identity),
            crate::opt_field("m_Stream", &self.m_stream),
            Some(("m_ForceToMono".to_string(), self.m_force_to_mono.to_value())),
            crate::opt_field("m_PreviewDataLength", &self.m_preview_data_length),
            Some(("m_3D".to_string(), self.m_3d.to_value())),
            crate::opt_field("m_PreloadAudioData", &self.m_preload_audio_data),
            crate::opt_field("m_Normalize", &self.m_normalize),
            self.audio_preview_data.as_ref().map(|v| {
                (
                    "audio preview data".to_string(),
                    crate::Value::Bytes(v.clone()),
                )
            }),
            crate::opt_field("m_LoadInBackground", &self.m_load_in_background),
            crate::opt_field("m_UseHardware", &self.m_use_hardware),
            crate::opt_field("m_Loopable", &self.m_loopable),
            crate::opt_field("m_PreviewData", &self.m_preview_data),
            crate::opt_field("m_Ambisonic", &self.m_ambisonic),
            crate::opt_field("m_Output", &self.m_output),
            crate::opt_field("m_UserData", &self.m_user_data),
            crate::opt_field("m_AssetBundleName", &self.m_asset_bundle_name),
            crate::opt_field("m_AssetBundleVariant", &self.m_asset_bundle_variant),
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
                "m_FileIDToRecycleName" => {
                    (self.m_file_id_to_recycle_name).inspect_value(path, offset, limit)
                }
                "m_Format" => (self.m_format).inspect_value(path, offset, limit),
                "m_DefaultSettings" => (self.m_default_settings).inspect_value(path, offset, limit),
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_OldHashIdentity" => {
                    (self.m_old_hash_identity).inspect_value(path, offset, limit)
                }
                "m_Quality" => (self.m_quality).inspect_value(path, offset, limit),
                "m_PlatformSettingOverrides" => {
                    (self.m_platform_setting_overrides).inspect_value(path, offset, limit)
                }
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_NewHashIdentity" => {
                    (self.m_new_hash_identity).inspect_value(path, offset, limit)
                }
                "m_Stream" => (self.m_stream).inspect_value(path, offset, limit),
                "m_ForceToMono" => (self.m_force_to_mono).inspect_value(path, offset, limit),
                "m_PreviewDataLength" => {
                    (self.m_preview_data_length).inspect_value(path, offset, limit)
                }
                "m_3D" => (self.m_3d).inspect_value(path, offset, limit),
                "m_PreloadAudioData" => {
                    (self.m_preload_audio_data).inspect_value(path, offset, limit)
                }
                "m_Normalize" => (self.m_normalize).inspect_value(path, offset, limit),
                "audio preview data" => (self.audio_preview_data)
                    .as_ref()
                    .and_then(|v| crate::inspect_bytes(&(v), path, offset, limit)),
                "m_LoadInBackground" => {
                    (self.m_load_in_background).inspect_value(path, offset, limit)
                }
                "m_UseHardware" => (self.m_use_hardware).inspect_value(path, offset, limit),
                "m_Loopable" => (self.m_loopable).inspect_value(path, offset, limit),
                "m_PreviewData" => (self.m_preview_data).inspect_value(path, offset, limit),
                "m_Ambisonic" => (self.m_ambisonic).inspect_value(path, offset, limit),
                "m_Output" => (self.m_output).inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_file_id_to_recycle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_FileIDToRecycleName".to_string(), v)),
            ((self.m_format).inspect_value(path, offset, limit))
                .map(|v| ("m_Format".to_string(), v)),
            ((self.m_default_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultSettings".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_old_hash_identity).inspect_value(path, offset, limit))
                .map(|v| ("m_OldHashIdentity".to_string(), v)),
            ((self.m_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_Quality".to_string(), v)),
            ((self.m_platform_setting_overrides).inspect_value(path, offset, limit))
                .map(|v| ("m_PlatformSettingOverrides".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_new_hash_identity).inspect_value(path, offset, limit))
                .map(|v| ("m_NewHashIdentity".to_string(), v)),
            ((self.m_stream).inspect_value(path, offset, limit))
                .map(|v| ("m_Stream".to_string(), v)),
            ((self.m_force_to_mono).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceToMono".to_string(), v)),
            ((self.m_preview_data_length).inspect_value(path, offset, limit))
                .map(|v| ("m_PreviewDataLength".to_string(), v)),
            ((self.m_3d).inspect_value(path, offset, limit)).map(|v| ("m_3D".to_string(), v)),
            ((self.m_preload_audio_data).inspect_value(path, offset, limit))
                .map(|v| ("m_PreloadAudioData".to_string(), v)),
            ((self.m_normalize).inspect_value(path, offset, limit))
                .map(|v| ("m_Normalize".to_string(), v)),
            ((self.audio_preview_data)
                .as_ref()
                .and_then(|v| crate::inspect_bytes(&(v), path, offset, limit)))
            .map(|v| ("audio preview data".to_string(), v)),
            ((self.m_load_in_background).inspect_value(path, offset, limit))
                .map(|v| ("m_LoadInBackground".to_string(), v)),
            ((self.m_use_hardware).inspect_value(path, offset, limit))
                .map(|v| ("m_UseHardware".to_string(), v)),
            ((self.m_loopable).inspect_value(path, offset, limit))
                .map(|v| ("m_Loopable".to_string(), v)),
            ((self.m_preview_data).inspect_value(path, offset, limit))
                .map(|v| ("m_PreviewData".to_string(), v)),
            ((self.m_ambisonic).inspect_value(path, offset, limit))
                .map(|v| ("m_Ambisonic".to_string(), v)),
            ((self.m_output).inspect_value(path, offset, limit))
                .map(|v| ("m_Output".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
