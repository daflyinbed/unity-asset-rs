//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::material_import_output::MaterialImportOutput;
use crate::subclasses::material_instance_settings::MaterialInstanceSettings;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;


/// 版本枚举：SubstanceImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SubstanceImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2018_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for SubstanceImporterMExternalObjectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2018_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubstanceImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_material_instances: Option<Vec<MaterialInstanceSettings>>,
    pub m_external_objects: Option<SubstanceImporterMExternalObjectsVariant>,
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_material_instances = None;
        let mut m_external_objects = None;
        let mut m_old_hash_identity = None;
        let mut m_is_first_import = None;
        let mut m_deleted_prototypes = None;
        let mut m_user_data = None;
        let mut m_used_file_i_ds = None;
        let mut m_new_hash_identity = None;
        let mut m_material_import_outputs = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(SubstanceImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_external_objects = Some(SubstanceImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(SubstanceImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_material_instances = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialInstanceSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_deleted_prototypes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            m_material_import_outputs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialImportOutput::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_material_instances = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialInstanceSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_deleted_prototypes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            m_material_import_outputs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialImportOutput::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_material_instances = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialInstanceSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_deleted_prototypes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            m_material_import_outputs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialImportOutput::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 2) {
            m_material_instances = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialInstanceSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_deleted_prototypes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_material_instances = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialInstanceSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_is_first_import = Some(rdr.read_i32()?);
        } else {
            m_file_id_to_recycle_name = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_material_instances = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialInstanceSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_is_first_import = Some(rdr.read_i32()?);
        }

        Ok(SubstanceImporter {
            m_name: m_name.unwrap(),
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
