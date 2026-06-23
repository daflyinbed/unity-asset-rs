//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::vfx_template::VfxTemplate;
use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct VisualEffectImporter {
    pub parent: AssetImporter,
    pub m_name: String,
    pub m_external_objects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_user_data: String,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_asset_bundle_name: String,
    pub m_template: Option<VfxTemplate>,
    pub m_asset_bundle_variant: String,
    pub m_use_as_template: Option<bool>,
}

impl crate::Parse for VisualEffectImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_external_objects = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_template = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = VfxTemplate::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;
        let m_use_as_template = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(VisualEffectImporter {
            parent,
            m_name,
            m_external_objects,
            m_user_data,
            m_used_file_i_ds,
            m_asset_bundle_name,
            m_template,
            m_asset_bundle_variant,
            m_use_as_template,
        })
    }
}
