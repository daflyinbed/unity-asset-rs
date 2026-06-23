//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::platform_settings_data::PlatformSettingsData;
use crate::subclasses::plugin_import_output::PluginImportOutput;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct MonoAssemblyImporter {
    pub parent: AssetImporter,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_icon_map: Vec<(String, PPtr)>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_execution_order: Vec<(String, i32)>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_user_data: Option<String>,
    pub m_is_preloaded: Option<bool>,
    pub m_platform_data: Option<Vec<(String, PlatformSettingsData)>>,
    pub m_is_overridable: Option<bool>,
    pub m_define_constraints: Option<Vec<String>>,
    pub m_output: Option<PluginImportOutput>,
    pub m_asset_bundle_name: Option<String>,
    pub m_is_explicitly_referenced: Option<bool>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_validate_references: Option<bool>,
}

impl crate::Parse for MonoAssemblyImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_icon_map = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
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
        let m_execution_order = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
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
        let m_user_data = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_preloaded = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_platform_data = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_overridable = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_define_constraints = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_output = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = PluginImportOutput::parse(r, ver)?;
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
        let m_is_explicitly_referenced = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
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
        let m_validate_references = if *ver >= crate::UnityVersion::new(2018, 3, 10) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(MonoAssemblyImporter {
            parent,
            m_name,
            m_file_id_to_recycle_name,
            m_icon_map,
            m_external_objects,
            m_old_hash_identity,
            m_execution_order,
            m_used_file_i_ds,
            m_new_hash_identity,
            m_user_data,
            m_is_preloaded,
            m_platform_data,
            m_is_overridable,
            m_define_constraints,
            m_output,
            m_asset_bundle_name,
            m_is_explicitly_referenced,
            m_asset_bundle_variant,
            m_validate_references,
        })
    }
}
