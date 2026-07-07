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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_external_objects = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_used_file_i_ds = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_texture_settings = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = TextureSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_platform_settings = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TextureImporterPlatformSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;
        let m_packing_settings = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = PackingSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_secondary_texture_settings = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = SecondaryTextureSettings::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_variant_multiplier = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_bind_as_default = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SpriteAtlasImporter {
            m_name,
            m_external_objects,
            m_used_file_i_ds,
            m_user_data,
            m_texture_settings,
            m_asset_bundle_name,
            m_platform_settings,
            m_asset_bundle_variant,
            m_packing_settings,
            m_secondary_texture_settings,
            m_variant_multiplier,
            m_bind_as_default,
        })
    }
}
