//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct C4DImporter {
    pub m_name: String,
    pub m_external_objects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_used_file_i_ds: Vec<i64>,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for C4DImporter {
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
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;

        Ok(C4DImporter {
            m_name,
            m_external_objects,
            m_used_file_i_ds,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
