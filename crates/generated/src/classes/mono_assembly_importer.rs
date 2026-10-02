//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::platform_settings_data_editor::PlatformSettingsData_Editor;
use crate::subclasses::plugin_import_output::PluginImportOutput;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

/// 版本枚举：MonoAssemblyImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoAssemblyImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for MonoAssemblyImporterMExternalObjectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MonoAssemblyImporterMExternalObjectsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：MonoAssemblyImporterMExecutionOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoAssemblyImporterMExecutionOrderVariant {
    Default(Vec<(String, i32)>),
    V4_0(Vec<(String, i32)>),
    V2018_3(Vec<(String, i32)>),
}

impl crate::Parse for MonoAssemblyImporterMExecutionOrderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = rdr.read_i32()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V2018_3(v))
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = rdr.read_i32()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V4_0(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = rdr.read_i32()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MonoAssemblyImporterMExecutionOrderVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：MonoAssemblyImporterMIsPreloadedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoAssemblyImporterMIsPreloadedVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for MonoAssemblyImporterMIsPreloadedVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MonoAssemblyImporterMIsPreloadedVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：MonoAssemblyImporterMPlatformDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoAssemblyImporterMPlatformDataVariant {
    Default(Vec<(String, PlatformSettingsData_Editor)>),
    V5_6(Vec<((String, String), PlatformSettingsData_Editor)>),
    V2023_3(Vec<(String, PlatformSettingsData_Editor)>),
}

impl crate::Parse for MonoAssemblyImporterMPlatformDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V2023_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })?;
                            let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = PlatformSettingsData_Editor::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MonoAssemblyImporterMPlatformDataVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
            Self::V2023_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_3(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MonoAssemblyImporterMIsOverridableVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 10) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3_10(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MonoAssemblyImporterMIsExplicitlyReferencedVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3_10(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3_10(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MonoAssemblyImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_icon_map: Vec<(String, PPtr)>,
    pub m_external_objects: Option<MonoAssemblyImporterMExternalObjectsVariant>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_execution_order: MonoAssemblyImporterMExecutionOrderVariant,
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
        rdr: &mut R,
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
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(MonoAssemblyImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(MonoAssemblyImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_execution_order = Some(MonoAssemblyImporterMExecutionOrderVariant::parse(rdr, ver)?);
        }
        m_icon_map = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(2018, 3, 10) {
            m_execution_order = Some(MonoAssemblyImporterMExecutionOrderVariant::parse(rdr, ver)?);
            m_define_constraints = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_is_preloaded = Some(MonoAssemblyImporterMIsPreloadedVariant::parse(rdr, ver)?);
            m_is_overridable = Some(MonoAssemblyImporterMIsOverridableVariant::parse(rdr, ver)?);
            m_is_explicitly_referenced = Some(
                MonoAssemblyImporterMIsExplicitlyReferencedVariant::parse(rdr, ver)?,
            );
            m_validate_references = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_platform_data = Some(MonoAssemblyImporterMPlatformDataVariant::parse(rdr, ver)?);
            m_output = Some(PluginImportOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_execution_order = Some(MonoAssemblyImporterMExecutionOrderVariant::parse(rdr, ver)?);
            m_define_constraints = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_is_preloaded = Some(MonoAssemblyImporterMIsPreloadedVariant::parse(rdr, ver)?);
            m_is_overridable = Some(MonoAssemblyImporterMIsOverridableVariant::parse(rdr, ver)?);
            m_is_explicitly_referenced = Some(
                MonoAssemblyImporterMIsExplicitlyReferencedVariant::parse(rdr, ver)?,
            );
            m_platform_data = Some(MonoAssemblyImporterMPlatformDataVariant::parse(rdr, ver)?);
            m_output = Some(PluginImportOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_execution_order = Some(MonoAssemblyImporterMExecutionOrderVariant::parse(rdr, ver)?);
            m_is_preloaded = Some(MonoAssemblyImporterMIsPreloadedVariant::parse(rdr, ver)?);
            m_is_overridable = Some(MonoAssemblyImporterMIsOverridableVariant::parse(rdr, ver)?);
            m_platform_data = Some(MonoAssemblyImporterMPlatformDataVariant::parse(rdr, ver)?);
            m_output = Some(PluginImportOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_execution_order = Some(MonoAssemblyImporterMExecutionOrderVariant::parse(rdr, ver)?);
            m_is_preloaded = Some(MonoAssemblyImporterMIsPreloadedVariant::parse(rdr, ver)?);
            m_platform_data = Some(MonoAssemblyImporterMPlatformDataVariant::parse(rdr, ver)?);
            m_output = Some(PluginImportOutput::parse(rdr, ver)?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_execution_order = Some(MonoAssemblyImporterMExecutionOrderVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_execution_order = Some(MonoAssemblyImporterMExecutionOrderVariant::parse(rdr, ver)?);
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

impl crate::ToValue for MonoAssemblyImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_FileIDToRecycleName", &self.m_file_id_to_recycle_name),
            Some(("m_IconMap".to_string(), self.m_icon_map.to_value())),
            crate::opt_field("m_ExternalObjects", &self.m_external_objects),
            crate::opt_field("m_OldHashIdentity", &self.m_old_hash_identity),
            Some((
                "m_ExecutionOrder".to_string(),
                self.m_execution_order.to_value(),
            )),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            crate::opt_field("m_NewHashIdentity", &self.m_new_hash_identity),
            crate::opt_field("m_UserData", &self.m_user_data),
            crate::opt_field("m_IsPreloaded", &self.m_is_preloaded),
            crate::opt_field("m_PlatformData", &self.m_platform_data),
            crate::opt_field("m_IsOverridable", &self.m_is_overridable),
            crate::opt_field("m_DefineConstraints", &self.m_define_constraints),
            crate::opt_field("m_Output", &self.m_output),
            crate::opt_field("m_AssetBundleName", &self.m_asset_bundle_name),
            crate::opt_field("m_IsExplicitlyReferenced", &self.m_is_explicitly_referenced),
            crate::opt_field("m_AssetBundleVariant", &self.m_asset_bundle_variant),
            crate::opt_field("m_ValidateReferences", &self.m_validate_references),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_FileIDToRecycleName" => {
                    (self.m_file_id_to_recycle_name).inspect_value(path, offset, limit)
                }
                "m_IconMap" => (self.m_icon_map).inspect_value(path, offset, limit),
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_OldHashIdentity" => {
                    (self.m_old_hash_identity).inspect_value(path, offset, limit)
                }
                "m_ExecutionOrder" => (self.m_execution_order).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_NewHashIdentity" => {
                    (self.m_new_hash_identity).inspect_value(path, offset, limit)
                }
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_IsPreloaded" => (self.m_is_preloaded).inspect_value(path, offset, limit),
                "m_PlatformData" => (self.m_platform_data).inspect_value(path, offset, limit),
                "m_IsOverridable" => (self.m_is_overridable).inspect_value(path, offset, limit),
                "m_DefineConstraints" => {
                    (self.m_define_constraints).inspect_value(path, offset, limit)
                }
                "m_Output" => (self.m_output).inspect_value(path, offset, limit),
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_IsExplicitlyReferenced" => {
                    (self.m_is_explicitly_referenced).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                "m_ValidateReferences" => {
                    (self.m_validate_references).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_file_id_to_recycle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_FileIDToRecycleName".to_string(), v)),
            ((self.m_icon_map).inspect_value(path, offset, limit))
                .map(|v| ("m_IconMap".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_old_hash_identity).inspect_value(path, offset, limit))
                .map(|v| ("m_OldHashIdentity".to_string(), v)),
            ((self.m_execution_order).inspect_value(path, offset, limit))
                .map(|v| ("m_ExecutionOrder".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_new_hash_identity).inspect_value(path, offset, limit))
                .map(|v| ("m_NewHashIdentity".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_is_preloaded).inspect_value(path, offset, limit))
                .map(|v| ("m_IsPreloaded".to_string(), v)),
            ((self.m_platform_data).inspect_value(path, offset, limit))
                .map(|v| ("m_PlatformData".to_string(), v)),
            ((self.m_is_overridable).inspect_value(path, offset, limit))
                .map(|v| ("m_IsOverridable".to_string(), v)),
            ((self.m_define_constraints).inspect_value(path, offset, limit))
                .map(|v| ("m_DefineConstraints".to_string(), v)),
            ((self.m_output).inspect_value(path, offset, limit))
                .map(|v| ("m_Output".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_is_explicitly_referenced).inspect_value(path, offset, limit))
                .map(|v| ("m_IsExplicitlyReferenced".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
            ((self.m_validate_references).inspect_value(path, offset, limit))
                .map(|v| ("m_ValidateReferences".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
