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
