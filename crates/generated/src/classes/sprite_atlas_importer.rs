//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::packing_settings::PackingSettings;
use crate::subclasses::secondary_texture_settings::SecondaryTextureSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::texture_importer_platform_settings::TextureImporterPlatformSettings;
use crate::subclasses::texture_settings::TextureSettings;

#[derive(Debug, Clone)]
pub struct SpriteAtlasImporter {
    pub m_name: String,
    pub m_external_objects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_used_file_i_ds: Vec<i64>,
    pub m_user_data: String,
    pub m_texture_settings: Option<TextureSettings>,
    pub m_asset_bundle_name: String,
    pub m_platform_settings: Option<Vec<TextureImporterPlatformSettings>>,
    pub m_asset_bundle_variant: String,
    pub m_packing_settings: Option<PackingSettings>,
    pub m_secondary_texture_settings: Option<Vec<(String, SecondaryTextureSettings)>>,
    pub m_variant_multiplier: Option<f32>,
    pub m_bind_as_default: Option<bool>,
}

impl crate::Parse for SpriteAtlasImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_external_objects = None;
        let mut m_used_file_i_ds = None;
        let mut m_user_data = None;
        let mut m_texture_settings = None;
        let mut m_asset_bundle_name = None;
        let mut m_platform_settings = None;
        let mut m_asset_bundle_variant = None;
        let mut m_packing_settings = None;
        let mut m_secondary_texture_settings = None;
        let mut m_variant_multiplier = None;
        let mut m_bind_as_default = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_external_objects = Some(
            ({
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
            })?,
        );
        rdr.align(4)?;
        m_used_file_i_ds = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i64())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_texture_settings = Some(TextureSettings::parse(rdr, ver)?);
            m_platform_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| TextureImporterPlatformSettings::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_packing_settings = Some(PackingSettings::parse(rdr, ver)?);
            m_secondary_texture_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = SecondaryTextureSettings::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_variant_multiplier = Some(rdr.read_f32()?);
            m_bind_as_default = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_user_data = Some(rdr.read_aligned_string()?);
        m_asset_bundle_name = Some(rdr.read_aligned_string()?);
        m_asset_bundle_variant = Some(rdr.read_aligned_string()?);

        Ok(SpriteAtlasImporter {
            m_name: m_name.unwrap(),
            m_external_objects: m_external_objects.unwrap(),
            m_used_file_i_ds: m_used_file_i_ds.unwrap(),
            m_user_data: m_user_data.unwrap(),
            m_texture_settings,
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_platform_settings,
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
            m_packing_settings,
            m_secondary_texture_settings,
            m_variant_multiplier,
            m_bind_as_default,
        })
    }
}

impl crate::ToValue for SpriteAtlasImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_ExternalObjects".to_string(),
                self.m_external_objects.to_value(),
            )),
            Some((
                "m_UsedFileIDs".to_string(),
                self.m_used_file_i_ds.to_value(),
            )),
            Some(("m_UserData".to_string(), self.m_user_data.to_value())),
            crate::opt_field("m_TextureSettings", &self.m_texture_settings),
            Some((
                "m_AssetBundleName".to_string(),
                self.m_asset_bundle_name.to_value(),
            )),
            crate::opt_field("m_PlatformSettings", &self.m_platform_settings),
            Some((
                "m_AssetBundleVariant".to_string(),
                self.m_asset_bundle_variant.to_value(),
            )),
            crate::opt_field("m_PackingSettings", &self.m_packing_settings),
            crate::opt_field(
                "m_SecondaryTextureSettings",
                &self.m_secondary_texture_settings,
            ),
            crate::opt_field("m_VariantMultiplier", &self.m_variant_multiplier),
            crate::opt_field("m_BindAsDefault", &self.m_bind_as_default),
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
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_TextureSettings" => (self.m_texture_settings).inspect_value(path, offset, limit),
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_PlatformSettings" => {
                    (self.m_platform_settings).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                "m_PackingSettings" => (self.m_packing_settings).inspect_value(path, offset, limit),
                "m_SecondaryTextureSettings" => {
                    (self.m_secondary_texture_settings).inspect_value(path, offset, limit)
                }
                "m_VariantMultiplier" => {
                    (self.m_variant_multiplier).inspect_value(path, offset, limit)
                }
                "m_BindAsDefault" => (self.m_bind_as_default).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_texture_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureSettings".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_platform_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_PlatformSettings".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
            ((self.m_packing_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_PackingSettings".to_string(), v)),
            ((self.m_secondary_texture_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_SecondaryTextureSettings".to_string(), v)),
            ((self.m_variant_multiplier).inspect_value(path, offset, limit))
                .map(|v| ("m_VariantMultiplier".to_string(), v)),
            ((self.m_bind_as_default).inspect_value(path, offset, limit))
                .map(|v| ("m_BindAsDefault".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
