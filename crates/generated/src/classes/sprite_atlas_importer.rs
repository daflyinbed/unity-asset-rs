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
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_texture_settings = Some(TextureSettings::parse(rdr, ver)?);
            m_platform_settings = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TextureImporterPlatformSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_packing_settings = Some(PackingSettings::parse(rdr, ver)?);
            m_secondary_texture_settings = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = SecondaryTextureSettings::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
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
