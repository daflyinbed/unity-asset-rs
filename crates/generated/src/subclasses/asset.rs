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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LibraryRepresentation::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LibraryRepresentation::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            let v = ( { Ok(Box::new(Hash128::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { Ok(Box::new(MdFour::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = Guid::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V2018_3(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(rdr, ver)?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            hash_of_imported_asset_dependencies = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            hash_of_source_asset_dependencies = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            guid_of_path_location_dependencies = Some(AssetGuidOfPathLocationDependenciesVariant::parse(rdr, ver)?);
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
