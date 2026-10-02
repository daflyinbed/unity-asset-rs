//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::md_four::MdFour;

/// 版本枚举：CachedAssetMetaDataOriginalDigestVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CachedAssetMetaDataOriginalDigestVariant {
    Default(Box<MdFour>),
    V5_0(Box<Hash128>),
}

impl crate::Parse for CachedAssetMetaDataOriginalDigestVariant {
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

impl crate::ToValue for CachedAssetMetaDataOriginalDigestVariant {
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
pub struct CachedAssetMetaData {
    pub guid: Guid,
    pub path_name: String,
    pub original_changeset: u32,
    pub original_name: String,
    pub original_parent: Guid,
    pub original_digest: CachedAssetMetaDataOriginalDigestVariant,
}

impl crate::Parse for CachedAssetMetaData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(rdr, ver)?;
        let path_name = rdr.read_aligned_string()?;
        let original_changeset = rdr.read_u32()?;
        let original_name = rdr.read_aligned_string()?;
        let original_parent = Guid::parse(rdr, ver)?;
        let original_digest = CachedAssetMetaDataOriginalDigestVariant::parse(rdr, ver)?;

        Ok(CachedAssetMetaData {
            guid,
            path_name,
            original_changeset,
            original_name,
            original_parent,
            original_digest,
        })
    }
}

impl crate::ToValue for CachedAssetMetaData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("guid".to_string(), self.guid.to_value())),
            Some(("pathName".to_string(), self.path_name.to_value())),
            Some((
                "originalChangeset".to_string(),
                self.original_changeset.to_value(),
            )),
            Some(("originalName".to_string(), self.original_name.to_value())),
            Some((
                "originalParent".to_string(),
                self.original_parent.to_value(),
            )),
            Some((
                "originalDigest".to_string(),
                self.original_digest.to_value(),
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
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "pathName" => (self.path_name).inspect_value(path, offset, limit),
                "originalChangeset" => (self.original_changeset).inspect_value(path, offset, limit),
                "originalName" => (self.original_name).inspect_value(path, offset, limit),
                "originalParent" => (self.original_parent).inspect_value(path, offset, limit),
                "originalDigest" => (self.original_digest).inspect_value(path, offset, limit),
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
            ((self.original_parent).inspect_value(path, offset, limit))
                .map(|v| ("originalParent".to_string(), v)),
            ((self.original_digest).inspect_value(path, offset, limit))
                .map(|v| ("originalDigest".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
