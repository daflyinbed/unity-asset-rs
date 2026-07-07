//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::platform_settings_data::PlatformSettingsData;
use crate::subclasses::plugin_import_output::PluginImportOutput;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct MonoAssemblyImporter {
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
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_icon_map = None;
        let mut m_external_objects = None;
        let mut m_old_hash_identity = None;
        let mut m_execution_order = None;
        let mut m_used_file_i_ds = None;
        let mut m_new_hash_identity = None;
        let mut m_user_data = None;
        let mut m_is_preloaded = None;
        let mut m_platform_data = None;
        let mut m_is_overridable = None;
        let mut m_define_constraints = None;
        let mut m_output = None;
        let mut m_asset_bundle_name = None;
        let mut m_is_explicitly_referenced = None;
        let mut m_asset_bundle_variant = None;
        let mut m_validate_references = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_old_hash_identity = Some(MdFour::parse(r, ver)?);
            r.align(4)?;
            m_new_hash_identity = Some(MdFour::parse(r, ver)?);
            r.align(4)?;
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        }
            m_icon_map = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_define_constraints = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(r.read_bool()?);
            m_is_overridable = Some(r.read_bool()?);
            m_is_explicitly_referenced = Some(r.read_bool()?);
            m_validate_references = Some(r.read_bool()?);
            r.align(4)?;
            m_platform_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_output = Some(PluginImportOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_define_constraints = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(r.read_bool()?);
            m_is_overridable = Some(r.read_bool()?);
            m_is_explicitly_referenced = Some(r.read_bool()?);
            m_platform_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_output = Some(PluginImportOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_define_constraints = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(r.read_bool()?);
            m_is_overridable = Some(r.read_bool()?);
            m_is_explicitly_referenced = Some(r.read_bool()?);
            m_validate_references = Some(r.read_bool()?);
            r.align(4)?;
            m_platform_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_output = Some(PluginImportOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_define_constraints = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(r.read_bool()?);
            m_is_overridable = Some(r.read_bool()?);
            m_is_explicitly_referenced = Some(r.read_bool()?);
            m_platform_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_output = Some(PluginImportOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 10) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_define_constraints = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(r.read_bool()?);
            m_is_overridable = Some(r.read_bool()?);
            m_is_explicitly_referenced = Some(r.read_bool()?);
            m_validate_references = Some(r.read_bool()?);
            r.align(4)?;
            m_platform_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_output = Some(PluginImportOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_define_constraints = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(r.read_bool()?);
            m_is_overridable = Some(r.read_bool()?);
            m_is_explicitly_referenced = Some(r.read_bool()?);
            m_platform_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_output = Some(PluginImportOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(r.read_bool()?);
            m_is_overridable = Some(r.read_bool()?);
            m_platform_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_output = Some(PluginImportOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(r.read_bool()?);
            m_platform_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_output = Some(PluginImportOutput::parse(r, ver)?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
        }

        Ok(MonoAssemblyImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_icon_map: m_icon_map.unwrap(),
            m_external_objects,
            m_old_hash_identity,
            m_execution_order: m_execution_order.unwrap(),
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
