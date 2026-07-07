//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct PrefabImporter {
    pub m_name: String,
    pub m_external_objects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_added_object_file_i_ds: Vec<i64>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_is_prefab_variant: bool,
    pub m_user_data: String,
    pub m_unable_to_import_on_previous_domain_reload: Option<bool>,
    pub m_asset_bundle_name: String,
    pub m_variant_parent_guid: Option<Guid>,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for PrefabImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_external_objects = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_added_object_file_i_ds = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_prefab_variant = r.read_bool()?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_unable_to_import_on_previous_domain_reload = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_variant_parent_guid = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = Guid::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;

        Ok(PrefabImporter {
            m_name,
            m_external_objects,
            m_added_object_file_i_ds,
            m_used_file_i_ds,
            m_is_prefab_variant,
            m_user_data,
            m_unable_to_import_on_previous_domain_reload,
            m_asset_bundle_name,
            m_variant_parent_guid,
            m_asset_bundle_variant,
        })
    }
}
