//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct RayTracingShaderImporter {
    pub m_name: String,
    pub m_external_objects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_used_file_i_ds: Vec<i64>,
    pub m_current_api_mask: Option<u32>,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for RayTracingShaderImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_external_objects = None;
        let mut m_used_file_i_ds = None;
        let mut m_current_api_mask = None;
        let mut m_user_data = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
            m_name = Some(rdr.read_aligned_string()?);
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
        } else {
            m_current_api_mask = Some(rdr.read_u32()?);
            rdr.align(4)?;
        }
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);

        Ok(RayTracingShaderImporter {
            m_name: m_name.unwrap(),
            m_external_objects: m_external_objects.unwrap(),
            m_used_file_i_ds: m_used_file_i_ds.unwrap(),
            m_current_api_mask,
            m_user_data: m_user_data.unwrap(),
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
        })
    }
}
