//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct IhvImageFormatImporter {
    pub m_name: String,
    pub m_texture_settings: GlTextureSettings,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_is_readable: bool,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_s_rgb_texture: Option<bool>,
    pub m_asset_bundle_variant: String,
    pub m_streaming_mipmaps: Option<bool>,
    pub m_streaming_mipmaps_priority: Option<i32>,
    pub m_ignore_mipmap_limit: Option<bool>,
    pub m_mipmap_limit_group_name: Option<String>,
}

impl crate::Parse for IhvImageFormatImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_external_objects = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_readable = r.read_bool()?;
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_s_rgb_texture = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;
        let m_streaming_mipmaps = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_streaming_mipmaps_priority = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_ignore_mipmap_limit = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mipmap_limit_group_name = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(IhvImageFormatImporter {
            m_name,
            m_texture_settings,
            m_external_objects,
            m_is_readable,
            m_used_file_i_ds,
            m_user_data,
            m_asset_bundle_name,
            m_s_rgb_texture,
            m_asset_bundle_variant,
            m_streaming_mipmaps,
            m_streaming_mipmaps_priority,
            m_ignore_mipmap_limit,
            m_mipmap_limit_group_name,
        })
    }
}
