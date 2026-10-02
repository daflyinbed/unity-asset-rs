//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

/// 版本枚举：PrefabImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PrefabImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for PrefabImporterMExternalObjectsVariant {
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

impl crate::ToValue for PrefabImporterMExternalObjectsVariant {
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

/// 版本枚举：PrefabImporterMIsPrefabVariantVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PrefabImporterMIsPrefabVariantVariant {
    Default(bool),
    V2020_1(bool),
}

impl crate::Parse for PrefabImporterMIsPrefabVariantVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PrefabImporterMIsPrefabVariantVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PrefabImporter {
    pub m_name: String,
    pub m_external_objects: PrefabImporterMExternalObjectsVariant,
    pub m_added_object_file_i_ds: Vec<i64>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_is_prefab_variant: PrefabImporterMIsPrefabVariantVariant,
    pub m_unable_to_import_on_previous_domain_reload: Option<bool>,
    pub m_user_data: String,
    pub m_variant_parent_guid: Option<Guid>,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for PrefabImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_external_objects = None;
        let mut m_added_object_file_i_ds = None;
        let mut m_used_file_i_ds = None;
        let mut m_is_prefab_variant = None;
        let mut m_unable_to_import_on_previous_domain_reload = None;
        let mut m_user_data = None;
        let mut m_variant_parent_guid = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_external_objects = Some(PrefabImporterMExternalObjectsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        m_added_object_file_i_ds = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i64())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_is_prefab_variant = Some(PrefabImporterMIsPrefabVariantVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_variant_parent_guid = Some(Guid::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
        } else {
            m_unable_to_import_on_previous_domain_reload = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_user_data = Some(rdr.read_aligned_string()?);
        m_asset_bundle_name = Some(rdr.read_aligned_string()?);
        m_asset_bundle_variant = Some(rdr.read_aligned_string()?);

        Ok(PrefabImporter {
            m_name: m_name.unwrap(),
            m_external_objects: m_external_objects.unwrap(),
            m_added_object_file_i_ds: m_added_object_file_i_ds.unwrap(),
            m_used_file_i_ds,
            m_is_prefab_variant: m_is_prefab_variant.unwrap(),
            m_unable_to_import_on_previous_domain_reload,
            m_user_data: m_user_data.unwrap(),
            m_variant_parent_guid,
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
        })
    }
}

impl crate::ToValue for PrefabImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_ExternalObjects".to_string(),
                self.m_external_objects.to_value(),
            )),
            Some((
                "m_AddedObjectFileIDs".to_string(),
                self.m_added_object_file_i_ds.to_value(),
            )),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            Some((
                "m_IsPrefabVariant".to_string(),
                self.m_is_prefab_variant.to_value(),
            )),
            crate::opt_field(
                "m_UnableToImportOnPreviousDomainReload",
                &self.m_unable_to_import_on_previous_domain_reload,
            ),
            Some(("m_UserData".to_string(), self.m_user_data.to_value())),
            crate::opt_field("m_VariantParentGUID", &self.m_variant_parent_guid),
            Some((
                "m_AssetBundleName".to_string(),
                self.m_asset_bundle_name.to_value(),
            )),
            Some((
                "m_AssetBundleVariant".to_string(),
                self.m_asset_bundle_variant.to_value(),
            )),
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
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_AddedObjectFileIDs" => {
                    (self.m_added_object_file_i_ds).inspect_value(path, offset, limit)
                }
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_IsPrefabVariant" => {
                    (self.m_is_prefab_variant).inspect_value(path, offset, limit)
                }
                "m_UnableToImportOnPreviousDomainReload" => (self
                    .m_unable_to_import_on_previous_domain_reload)
                    .inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_VariantParentGUID" => {
                    (self.m_variant_parent_guid).inspect_value(path, offset, limit)
                }
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_added_object_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_AddedObjectFileIDs".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_is_prefab_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_IsPrefabVariant".to_string(), v)),
            ((self.m_unable_to_import_on_previous_domain_reload)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_UnableToImportOnPreviousDomainReload".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_variant_parent_guid).inspect_value(path, offset, limit))
                .map(|v| ("m_VariantParentGUID".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
