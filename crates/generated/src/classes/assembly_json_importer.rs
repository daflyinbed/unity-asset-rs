//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct AssemblyJsonImporter {
    pub m_name: String,
    pub m_user_data: String,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_asset_bundle_name: String,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for AssemblyJsonImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_external_objects = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;

        Ok(AssemblyJsonImporter {
            m_name,
            m_user_data,
            m_external_objects,
            m_asset_bundle_name,
            m_used_file_i_ds,
            m_asset_bundle_variant,
        })
    }
}
