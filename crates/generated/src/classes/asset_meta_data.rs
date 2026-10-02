//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::md_four::MdFour;

/// 版本枚举：AssetMetaDataLabelsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetMetaDataLabelsVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for AssetMetaDataLabelsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AssetMetaDataLabelsVariant {
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

/// 版本枚举：AssetMetaDataOriginalDigestVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetMetaDataOriginalDigestVariant {
    Default(Box<MdFour>),
    V5_0(Box<Hash128>),
}

impl crate::Parse for AssetMetaDataOriginalDigestVariant {
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

impl crate::ToValue for AssetMetaDataOriginalDigestVariant {
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
#[derive(Debug, Clone)]
pub struct AssetMetaData {
    pub guid: Guid,
    pub path_name: String,
    pub original_changeset: Option<u32>,
    pub original_name: String,
    pub time_created: Option<u64>,
    pub labels: AssetMetaDataLabelsVariant,
    pub original_parent: Option<Guid>,
    pub asset_store_ref: u64,
    pub original_digest: Option<AssetMetaDataOriginalDigestVariant>,
    pub license_type: Option<i32>,
}

impl crate::Parse for AssetMetaData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut guid = None;
        let mut path_name = None;
        let mut original_changeset = None;
        let mut original_name = None;
        let mut time_created = None;
        let mut labels = None;
        let mut original_parent = None;
        let mut asset_store_ref = None;
        let mut original_digest = None;
        let mut license_type = None;
        guid = Some(Guid::parse(rdr, ver)?);
        path_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2017, 4, 6) {
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            time_created = Some(rdr.read_u64()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 3) {
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            time_created = Some(rdr.read_u64()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            time_created = Some(rdr.read_u64()?);
            original_changeset = Some(rdr.read_u32()?);
        } else {
            original_changeset = Some(rdr.read_u32()?);
        }
        original_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
        } else {
            original_parent = Some(Guid::parse(rdr, ver)?);
            original_digest = Some(AssetMetaDataOriginalDigestVariant::parse(rdr, ver)?);
        }
        labels = Some(AssetMetaDataLabelsVariant::parse(rdr, ver)?);
        asset_store_ref = Some(rdr.read_u64()?);
        if *ver >= crate::UnityVersion::new(2017, 4, 6) {
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            license_type = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 3) {
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            license_type = Some(rdr.read_i32()?);
        } else {
        }

        Ok(AssetMetaData {
            guid: guid.unwrap(),
            path_name: path_name.unwrap(),
            original_changeset,
            original_name: original_name.unwrap(),
            time_created,
            labels: labels.unwrap(),
            original_parent,
            asset_store_ref: asset_store_ref.unwrap(),
            original_digest,
            license_type,
        })
    }
}

impl crate::ToValue for AssetMetaData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("guid".to_string(), self.guid.to_value())),
            Some(("pathName".to_string(), self.path_name.to_value())),
            crate::opt_field("originalChangeset", &self.original_changeset),
            Some(("originalName".to_string(), self.original_name.to_value())),
            crate::opt_field("timeCreated", &self.time_created),
            Some(("labels".to_string(), self.labels.to_value())),
            crate::opt_field("originalParent", &self.original_parent),
            Some(("assetStoreRef".to_string(), self.asset_store_ref.to_value())),
            crate::opt_field("originalDigest", &self.original_digest),
            crate::opt_field("licenseType", &self.license_type),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "pathName" => (self.path_name).inspect_value(path, offset, limit),
                "originalChangeset" => (self.original_changeset).inspect_value(path, offset, limit),
                "originalName" => (self.original_name).inspect_value(path, offset, limit),
                "timeCreated" => (self.time_created).inspect_value(path, offset, limit),
                "labels" => (self.labels).inspect_value(path, offset, limit),
                "originalParent" => (self.original_parent).inspect_value(path, offset, limit),
                "assetStoreRef" => (self.asset_store_ref).inspect_value(path, offset, limit),
                "originalDigest" => (self.original_digest).inspect_value(path, offset, limit),
                "licenseType" => (self.license_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.guid).inspect_value(path, offset, limit)).map(|v| ("guid".to_string(), v)),
            ((self.path_name).inspect_value(path, offset, limit))
                .map(|v| ("pathName".to_string(), v)),
            ((self.original_changeset).inspect_value(path, offset, limit))
                .map(|v| ("originalChangeset".to_string(), v)),
            ((self.original_name).inspect_value(path, offset, limit))
                .map(|v| ("originalName".to_string(), v)),
            ((self.time_created).inspect_value(path, offset, limit))
                .map(|v| ("timeCreated".to_string(), v)),
            ((self.labels).inspect_value(path, offset, limit)).map(|v| ("labels".to_string(), v)),
            ((self.original_parent).inspect_value(path, offset, limit))
                .map(|v| ("originalParent".to_string(), v)),
            ((self.asset_store_ref).inspect_value(path, offset, limit))
                .map(|v| ("assetStoreRef".to_string(), v)),
            ((self.original_digest).inspect_value(path, offset, limit))
                .map(|v| ("originalDigest".to_string(), v)),
            ((self.license_type).inspect_value(path, offset, limit))
                .map(|v| ("licenseType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
