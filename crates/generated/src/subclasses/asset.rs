//! 自动生成。请勿手动修改。

use crate::subclasses::asset_labels::AssetLabels;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::library_representation::LibraryRepresentation;
use crate::subclasses::md_four::MdFour;

/// 版本枚举：AssetRepresentationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetRepresentationsVariant {
    Default(Vec<LibraryRepresentation>),
    V2017_1(Vec<LibraryRepresentation>),
}

impl crate::Parse for AssetRepresentationsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| LibraryRepresentation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| LibraryRepresentation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AssetRepresentationsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AssetChildrenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetChildrenVariant {
    Default(Vec<Guid>),
    V2017_1(Vec<Guid>),
}

impl crate::Parse for AssetChildrenVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Guid::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Guid::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AssetChildrenVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AssetHashVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetHashVariant {
    Default(Box<MdFour>),
    V5_0(Box<Hash128>),
}

impl crate::Parse for AssetHashVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ({ Ok(Box::new(Hash128::parse(rdr, ver)?)) })?;
            Ok(Self::V5_0(v))
        } else {
            let v = ({ Ok(Box::new(MdFour::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AssetHashVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AssetGuidOfPathLocationDependenciesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetGuidOfPathLocationDependenciesVariant {
    Default(Vec<(Guid, String)>),
    V2018_3(Vec<(String, Guid)>),
}

impl crate::Parse for AssetGuidOfPathLocationDependenciesVariant {
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
                            let b = Guid::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V2018_3(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = Guid::parse(rdr, ver)?;
                            let b = rdr.read_aligned_string()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AssetGuidOfPathLocationDependenciesVariant {
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
#[derive(Debug, Clone)]
pub struct Asset {
    pub modification_date_0: Option<u32>,
    pub main_representation: LibraryRepresentation,
    pub modification_date_1: Option<u32>,
    pub representations: AssetRepresentationsVariant,
    pub meta_modification_date_0: Option<u32>,
    pub children: AssetChildrenVariant,
    pub meta_modification_date_1: Option<u32>,
    pub parent: Guid,
    pub r#type: i32,
    pub labels: AssetLabels,
    pub importer_class_id: Option<i32>,
    pub importer_version_hash: Option<u32>,
    pub scripted_importer_class_id: Option<String>,
    pub digest: Option<MdFour>,
    pub hash: Option<AssetHashVariant>,
    pub asset_bundle_index: Option<i32>,
    pub hash_of_imported_asset_dependencies: Option<Vec<Guid>>,
    pub hash_of_source_asset_dependencies: Option<Vec<Guid>>,
    pub guid_of_path_location_dependencies: Option<AssetGuidOfPathLocationDependenciesVariant>,
}

impl crate::Parse for Asset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut modification_date_0 = None;
        let mut main_representation = None;
        let mut modification_date_1 = None;
        let mut representations = None;
        let mut meta_modification_date_0 = None;
        let mut children = None;
        let mut meta_modification_date_1 = None;
        let mut parent = None;
        let mut r#type = None;
        let mut labels = None;
        let mut importer_class_id = None;
        let mut importer_version_hash = None;
        let mut scripted_importer_class_id = None;
        let mut digest = None;
        let mut hash = None;
        let mut asset_bundle_index = None;
        let mut hash_of_imported_asset_dependencies = None;
        let mut hash_of_source_asset_dependencies = None;
        let mut guid_of_path_location_dependencies = None;
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            modification_date_0 = Some(rdr.read_u32()?);
            modification_date_1 = Some(rdr.read_u32()?);
            meta_modification_date_0 = Some(rdr.read_u32()?);
            meta_modification_date_1 = Some(rdr.read_u32()?);
        }
        main_representation = Some(LibraryRepresentation::parse(rdr, ver)?);
        representations = Some(AssetRepresentationsVariant::parse(rdr, ver)?);
        children = Some(AssetChildrenVariant::parse(rdr, ver)?);
        parent = Some(Guid::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            digest = Some(MdFour::parse(rdr, ver)?);
        }
        r#type = Some(rdr.read_i32()?);
        labels = Some(AssetLabels::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            importer_class_id = Some(rdr.read_i32()?);
            scripted_importer_class_id = Some(rdr.read_aligned_string()?);
            importer_version_hash = Some(rdr.read_u32()?);
            hash = Some(AssetHashVariant::parse(rdr, ver)?);
            asset_bundle_index = Some(rdr.read_i32()?);
            hash_of_imported_asset_dependencies = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Guid::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            hash_of_source_asset_dependencies = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Guid::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            guid_of_path_location_dependencies =
                Some(AssetGuidOfPathLocationDependenciesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 4, 14) {
            importer_class_id = Some(rdr.read_i32()?);
            scripted_importer_class_id = Some(rdr.read_aligned_string()?);
            importer_version_hash = Some(rdr.read_u32()?);
            hash = Some(AssetHashVariant::parse(rdr, ver)?);
            asset_bundle_index = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            importer_class_id = Some(rdr.read_i32()?);
            importer_version_hash = Some(rdr.read_u32()?);
            hash = Some(AssetHashVariant::parse(rdr, ver)?);
            asset_bundle_index = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 4) {
            importer_class_id = Some(rdr.read_i32()?);
            scripted_importer_class_id = Some(rdr.read_aligned_string()?);
            importer_version_hash = Some(rdr.read_u32()?);
            hash = Some(AssetHashVariant::parse(rdr, ver)?);
            asset_bundle_index = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            importer_class_id = Some(rdr.read_i32()?);
            importer_version_hash = Some(rdr.read_u32()?);
            hash = Some(AssetHashVariant::parse(rdr, ver)?);
            asset_bundle_index = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            importer_class_id = Some(rdr.read_i32()?);
            importer_version_hash = Some(rdr.read_u32()?);
            hash = Some(AssetHashVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            importer_class_id = Some(rdr.read_i32()?);
            importer_version_hash = Some(rdr.read_u32()?);
        } else {
        }

        Ok(Asset {
            modification_date_0,
            main_representation: main_representation.unwrap(),
            modification_date_1,
            representations: representations.unwrap(),
            meta_modification_date_0,
            children: children.unwrap(),
            meta_modification_date_1,
            parent: parent.unwrap(),
            r#type: r#type.unwrap(),
            labels: labels.unwrap(),
            importer_class_id,
            importer_version_hash,
            scripted_importer_class_id,
            digest,
            hash,
            asset_bundle_index,
            hash_of_imported_asset_dependencies,
            hash_of_source_asset_dependencies,
            guid_of_path_location_dependencies,
        })
    }
}

impl crate::ToValue for Asset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("modificationDate[0]", &self.modification_date_0),
            Some((
                "mainRepresentation".to_string(),
                self.main_representation.to_value(),
            )),
            crate::opt_field("modificationDate[1]", &self.modification_date_1),
            Some((
                "representations".to_string(),
                self.representations.to_value(),
            )),
            crate::opt_field("metaModificationDate[0]", &self.meta_modification_date_0),
            Some(("children".to_string(), self.children.to_value())),
            crate::opt_field("metaModificationDate[1]", &self.meta_modification_date_1),
            Some(("parent".to_string(), self.parent.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("labels".to_string(), self.labels.to_value())),
            crate::opt_field("importerClassId", &self.importer_class_id),
            crate::opt_field("importerVersionHash", &self.importer_version_hash),
            crate::opt_field("scriptedImporterClassID", &self.scripted_importer_class_id),
            crate::opt_field("digest", &self.digest),
            crate::opt_field("hash", &self.hash),
            crate::opt_field("assetBundleIndex", &self.asset_bundle_index),
            crate::opt_field(
                "hashOfImportedAssetDependencies",
                &self.hash_of_imported_asset_dependencies,
            ),
            crate::opt_field(
                "hashOfSourceAssetDependencies",
                &self.hash_of_source_asset_dependencies,
            ),
            crate::opt_field(
                "guidOfPathLocationDependencies",
                &self.guid_of_path_location_dependencies,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "modificationDate[0]" => {
                    (self.modification_date_0).inspect_value(path, offset, limit)
                }
                "mainRepresentation" => {
                    (self.main_representation).inspect_value(path, offset, limit)
                }
                "modificationDate[1]" => {
                    (self.modification_date_1).inspect_value(path, offset, limit)
                }
                "representations" => (self.representations).inspect_value(path, offset, limit),
                "metaModificationDate[0]" => {
                    (self.meta_modification_date_0).inspect_value(path, offset, limit)
                }
                "children" => (self.children).inspect_value(path, offset, limit),
                "metaModificationDate[1]" => {
                    (self.meta_modification_date_1).inspect_value(path, offset, limit)
                }
                "parent" => (self.parent).inspect_value(path, offset, limit),
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "labels" => (self.labels).inspect_value(path, offset, limit),
                "importerClassId" => (self.importer_class_id).inspect_value(path, offset, limit),
                "importerVersionHash" => {
                    (self.importer_version_hash).inspect_value(path, offset, limit)
                }
                "scriptedImporterClassID" => {
                    (self.scripted_importer_class_id).inspect_value(path, offset, limit)
                }
                "digest" => (self.digest).inspect_value(path, offset, limit),
                "hash" => (self.hash).inspect_value(path, offset, limit),
                "assetBundleIndex" => (self.asset_bundle_index).inspect_value(path, offset, limit),
                "hashOfImportedAssetDependencies" => {
                    (self.hash_of_imported_asset_dependencies).inspect_value(path, offset, limit)
                }
                "hashOfSourceAssetDependencies" => {
                    (self.hash_of_source_asset_dependencies).inspect_value(path, offset, limit)
                }
                "guidOfPathLocationDependencies" => {
                    (self.guid_of_path_location_dependencies).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.modification_date_0).inspect_value(path, offset, limit))
                .map(|v| ("modificationDate[0]".to_string(), v)),
            ((self.main_representation).inspect_value(path, offset, limit))
                .map(|v| ("mainRepresentation".to_string(), v)),
            ((self.modification_date_1).inspect_value(path, offset, limit))
                .map(|v| ("modificationDate[1]".to_string(), v)),
            ((self.representations).inspect_value(path, offset, limit))
                .map(|v| ("representations".to_string(), v)),
            ((self.meta_modification_date_0).inspect_value(path, offset, limit))
                .map(|v| ("metaModificationDate[0]".to_string(), v)),
            ((self.children).inspect_value(path, offset, limit))
                .map(|v| ("children".to_string(), v)),
            ((self.meta_modification_date_1).inspect_value(path, offset, limit))
                .map(|v| ("metaModificationDate[1]".to_string(), v)),
            ((self.parent).inspect_value(path, offset, limit)).map(|v| ("parent".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.labels).inspect_value(path, offset, limit)).map(|v| ("labels".to_string(), v)),
            ((self.importer_class_id).inspect_value(path, offset, limit))
                .map(|v| ("importerClassId".to_string(), v)),
            ((self.importer_version_hash).inspect_value(path, offset, limit))
                .map(|v| ("importerVersionHash".to_string(), v)),
            ((self.scripted_importer_class_id).inspect_value(path, offset, limit))
                .map(|v| ("scriptedImporterClassID".to_string(), v)),
            ((self.digest).inspect_value(path, offset, limit)).map(|v| ("digest".to_string(), v)),
            ((self.hash).inspect_value(path, offset, limit)).map(|v| ("hash".to_string(), v)),
            ((self.asset_bundle_index).inspect_value(path, offset, limit))
                .map(|v| ("assetBundleIndex".to_string(), v)),
            ((self.hash_of_imported_asset_dependencies).inspect_value(path, offset, limit))
                .map(|v| ("hashOfImportedAssetDependencies".to_string(), v)),
            ((self.hash_of_source_asset_dependencies).inspect_value(path, offset, limit))
                .map(|v| ("hashOfSourceAssetDependencies".to_string(), v)),
            ((self.guid_of_path_location_dependencies).inspect_value(path, offset, limit))
                .map(|v| ("guidOfPathLocationDependencies".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
