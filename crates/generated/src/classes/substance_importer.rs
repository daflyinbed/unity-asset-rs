//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::material_import_output::MaterialImportOutput;
use crate::subclasses::material_instance_settings::MaterialInstanceSettings;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct SubstanceImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_material_instances: Option<Vec<MaterialInstanceSettings>>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_is_first_import: Option<i32>,
    pub m_deleted_prototypes: Option<Vec<String>>,
    pub m_user_data: Option<String>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_material_import_outputs: Option<Vec<MaterialImportOutput>>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for SubstanceImporter {
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
        let m_material_instances = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MaterialInstanceSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
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
        let m_is_first_import = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_deleted_prototypes = if *ver >= crate::UnityVersion::new(3, 5, 2) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
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
        let m_material_import_outputs = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MaterialImportOutput::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
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
        let m_asset_bundle_variant = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SubstanceImporter {
            m_name,
            m_file_id_to_recycle_name,
            m_material_instances,
            m_external_objects,
            m_old_hash_identity,
            m_is_first_import,
            m_deleted_prototypes,
            m_user_data,
            m_used_file_i_ds,
            m_new_hash_identity,
            m_material_import_outputs,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
