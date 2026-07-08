//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::platform_settings_data::PlatformSettingsData;
use crate::subclasses::plugin_import_output::PluginImportOutput;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;


/// 版本枚举：MonoAssemblyImporterMIsPreloadedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoAssemblyImporterMIsPreloadedVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for MonoAssemblyImporterMIsPreloadedVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MonoAssemblyImporterMPlatformDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoAssemblyImporterMPlatformDataVariant {
    Default(Vec<(String, PlatformSettingsData)>),
    V5_6(Vec<((String, String), PlatformSettingsData)>),
    V2023_3(Vec<(String, PlatformSettingsData)>),
}

impl crate::Parse for MonoAssemblyImporterMPlatformDataVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2023_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } )?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V5_6(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = PlatformSettingsData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MonoAssemblyImporterMIsOverridableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoAssemblyImporterMIsOverridableVariant {
    Default(bool),
    V2018_3(bool),
}

impl crate::Parse for MonoAssemblyImporterMIsOverridableVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MonoAssemblyImporterMIsExplicitlyReferencedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoAssemblyImporterMIsExplicitlyReferencedVariant {
    Default(bool),
    V2018_3_10(bool),
}

impl crate::Parse for MonoAssemblyImporterMIsExplicitlyReferencedVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 10) {
            let v = r.read_bool()?;
            Ok(Self::V2018_3_10(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
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
    pub m_is_preloaded: Option<MonoAssemblyImporterMIsPreloadedVariant>,
    pub m_platform_data: Option<MonoAssemblyImporterMPlatformDataVariant>,
    pub m_is_overridable: Option<MonoAssemblyImporterMIsOverridableVariant>,
    pub m_define_constraints: Option<Vec<String>>,
    pub m_output: Option<PluginImportOutput>,
    pub m_asset_bundle_name: Option<String>,
    pub m_is_explicitly_referenced: Option<MonoAssemblyImporterMIsExplicitlyReferencedVariant>,
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
        if *ver >= crate::UnityVersion::new(2018, 3, 10) {
            m_execution_order = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_define_constraints = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_preloaded = Some(MonoAssemblyImporterMIsPreloadedVariant::parse(r, ver)?);
            m_is_overridable = Some(MonoAssemblyImporterMIsOverridableVariant::parse(r, ver)?);
            m_is_explicitly_referenced = Some(MonoAssemblyImporterMIsExplicitlyReferencedVariant::parse(r, ver)?);
            m_validate_references = Some(r.read_bool()?);
            r.align(4)?;
            m_platform_data = Some(MonoAssemblyImporterMPlatformDataVariant::parse(r, ver)?);
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
            m_is_preloaded = Some(MonoAssemblyImporterMIsPreloadedVariant::parse(r, ver)?);
            m_is_overridable = Some(MonoAssemblyImporterMIsOverridableVariant::parse(r, ver)?);
            m_is_explicitly_referenced = Some(MonoAssemblyImporterMIsExplicitlyReferencedVariant::parse(r, ver)?);
            m_platform_data = Some(MonoAssemblyImporterMPlatformDataVariant::parse(r, ver)?);
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
            m_is_preloaded = Some(MonoAssemblyImporterMIsPreloadedVariant::parse(r, ver)?);
            m_is_overridable = Some(MonoAssemblyImporterMIsOverridableVariant::parse(r, ver)?);
            m_platform_data = Some(MonoAssemblyImporterMPlatformDataVariant::parse(r, ver)?);
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
            m_is_preloaded = Some(MonoAssemblyImporterMIsPreloadedVariant::parse(r, ver)?);
            m_platform_data = Some(MonoAssemblyImporterMPlatformDataVariant::parse(r, ver)?);
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
