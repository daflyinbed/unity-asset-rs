//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

/// 版本枚举：IhvImageFormatImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum IhvImageFormatImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for IhvImageFormatImporterMExternalObjectsVariant {
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

impl crate::ToValue for IhvImageFormatImporterMExternalObjectsVariant {
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

/// 版本枚举：IhvImageFormatImporterMIsReadableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum IhvImageFormatImporterMIsReadableVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for IhvImageFormatImporterMIsReadableVariant {
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

impl crate::ToValue for IhvImageFormatImporterMIsReadableVariant {
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

/// 版本枚举：IhvImageFormatImporterMSRgbTextureVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum IhvImageFormatImporterMSRgbTextureVariant {
    Default(bool),
    V2018_2(bool),
}

impl crate::Parse for IhvImageFormatImporterMSRgbTextureVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for IhvImageFormatImporterMSRgbTextureVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：IhvImageFormatImporterMStreamingMipmapsPriorityVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum IhvImageFormatImporterMStreamingMipmapsPriorityVariant {
    Default(i32),
    V2022_2(i32),
}

impl crate::Parse for IhvImageFormatImporterMStreamingMipmapsPriorityVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2022_2(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for IhvImageFormatImporterMStreamingMipmapsPriorityVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IhvImageFormatImporter {
    pub m_name: String,
    pub m_texture_settings: GlTextureSettings,
    pub m_external_objects: Option<IhvImageFormatImporterMExternalObjectsVariant>,
    pub m_is_readable: IhvImageFormatImporterMIsReadableVariant,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_s_rgb_texture: Option<IhvImageFormatImporterMSRgbTextureVariant>,
    pub m_asset_bundle_variant: String,
    pub m_streaming_mipmaps: Option<bool>,
    pub m_streaming_mipmaps_priority:
        Option<IhvImageFormatImporterMStreamingMipmapsPriorityVariant>,
    pub m_ignore_mipmap_limit: Option<bool>,
    pub m_mipmap_limit_group_name: Option<String>,
}

impl crate::Parse for IhvImageFormatImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_texture_settings = None;
        let mut m_external_objects = None;
        let mut m_is_readable = None;
        let mut m_used_file_i_ds = None;
        let mut m_user_data = None;
        let mut m_asset_bundle_name = None;
        let mut m_s_rgb_texture = None;
        let mut m_asset_bundle_variant = None;
        let mut m_streaming_mipmaps = None;
        let mut m_streaming_mipmaps_priority = None;
        let mut m_ignore_mipmap_limit = None;
        let mut m_mipmap_limit_group_name = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(IhvImageFormatImporterMExternalObjectsVariant::parse(
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
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(IhvImageFormatImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
        } else {
        }
        m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
        m_is_readable = Some(IhvImageFormatImporterMIsReadableVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_s_rgb_texture = Some(IhvImageFormatImporterMSRgbTextureVariant::parse(rdr, ver)?);
            m_streaming_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_streaming_mipmaps_priority =
                Some(IhvImageFormatImporterMStreamingMipmapsPriorityVariant::parse(rdr, ver)?);
            m_ignore_mipmap_limit = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mipmap_limit_group_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_s_rgb_texture = Some(IhvImageFormatImporterMSRgbTextureVariant::parse(rdr, ver)?);
            m_streaming_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_streaming_mipmaps_priority =
                Some(IhvImageFormatImporterMStreamingMipmapsPriorityVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_s_rgb_texture = Some(IhvImageFormatImporterMSRgbTextureVariant::parse(rdr, ver)?);
        } else {
        }
        m_user_data = Some(rdr.read_aligned_string()?);
        m_asset_bundle_name = Some(rdr.read_aligned_string()?);
        m_asset_bundle_variant = Some(rdr.read_aligned_string()?);

        Ok(IhvImageFormatImporter {
            m_name: m_name.unwrap(),
            m_texture_settings: m_texture_settings.unwrap(),
            m_external_objects,
            m_is_readable: m_is_readable.unwrap(),
            m_used_file_i_ds,
            m_user_data: m_user_data.unwrap(),
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_s_rgb_texture,
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
            m_streaming_mipmaps,
            m_streaming_mipmaps_priority,
            m_ignore_mipmap_limit,
            m_mipmap_limit_group_name,
        })
    }
}

impl crate::ToValue for IhvImageFormatImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_TextureSettings".to_string(),
                self.m_texture_settings.to_value(),
            )),
            crate::opt_field("m_ExternalObjects", &self.m_external_objects),
            Some(("m_IsReadable".to_string(), self.m_is_readable.to_value())),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            Some(("m_UserData".to_string(), self.m_user_data.to_value())),
            Some((
                "m_AssetBundleName".to_string(),
                self.m_asset_bundle_name.to_value(),
            )),
            crate::opt_field("m_sRGBTexture", &self.m_s_rgb_texture),
            Some((
                "m_AssetBundleVariant".to_string(),
                self.m_asset_bundle_variant.to_value(),
            )),
            crate::opt_field("m_StreamingMipmaps", &self.m_streaming_mipmaps),
            crate::opt_field(
                "m_StreamingMipmapsPriority",
                &self.m_streaming_mipmaps_priority,
            ),
            crate::opt_field("m_IgnoreMipmapLimit", &self.m_ignore_mipmap_limit),
            crate::opt_field("m_MipmapLimitGroupName", &self.m_mipmap_limit_group_name),
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
                "m_TextureSettings" => (self.m_texture_settings).inspect_value(path, offset, limit),
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_IsReadable" => (self.m_is_readable).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_sRGBTexture" => (self.m_s_rgb_texture).inspect_value(path, offset, limit),
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                "m_StreamingMipmaps" => {
                    (self.m_streaming_mipmaps).inspect_value(path, offset, limit)
                }
                "m_StreamingMipmapsPriority" => {
                    (self.m_streaming_mipmaps_priority).inspect_value(path, offset, limit)
                }
                "m_IgnoreMipmapLimit" => {
                    (self.m_ignore_mipmap_limit).inspect_value(path, offset, limit)
                }
                "m_MipmapLimitGroupName" => {
                    (self.m_mipmap_limit_group_name).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_texture_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureSettings".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_is_readable).inspect_value(path, offset, limit))
                .map(|v| ("m_IsReadable".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_s_rgb_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_sRGBTexture".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
            ((self.m_streaming_mipmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamingMipmaps".to_string(), v)),
            ((self.m_streaming_mipmaps_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamingMipmapsPriority".to_string(), v)),
            ((self.m_ignore_mipmap_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_IgnoreMipmapLimit".to_string(), v)),
            ((self.m_mipmap_limit_group_name).inspect_value(path, offset, limit))
                .map(|v| ("m_MipmapLimitGroupName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
