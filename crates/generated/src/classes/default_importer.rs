//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct DefaultImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_user_data: Option<String>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_asset_bundle_name: Option<String>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for DefaultImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_user_data = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_aligned_string()?;
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
        let m_old_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
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
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_new_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
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

        Ok(DefaultImporter {
            m_name,
            m_file_id_to_recycle_name,
            m_user_data,
            m_external_objects,
            m_old_hash_identity,
            m_asset_bundle_name,
            m_used_file_i_ds,
            m_new_hash_identity,
            m_asset_bundle_variant,
        })
    }
}
