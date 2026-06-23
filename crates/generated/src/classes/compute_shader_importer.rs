//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct ComputeShaderImporter {
    pub parent: AssetImporter,
    pub m_name: String,
    pub m_user_data: String,
    pub m_current_build_target: Option<i32>,
    pub m_current_api_mask: Option<u32>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_asset_bundle_name: Option<String>,
    pub m_preprocessor_override: Option<i32>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for ComputeShaderImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_current_build_target = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_current_api_mask = if *ver >= crate::UnityVersion::new(5, 3, 2) {
            let v = r.read_u32()?;
            r.align(4)?;
            Some(v)
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
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
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
        let m_preprocessor_override = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_i32()?;
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

        Ok(ComputeShaderImporter {
            parent,
            m_name,
            m_user_data,
            m_current_build_target,
            m_current_api_mask,
            m_external_objects,
            m_used_file_i_ds,
            m_asset_bundle_name,
            m_preprocessor_override,
            m_asset_bundle_variant,
        })
    }
}
