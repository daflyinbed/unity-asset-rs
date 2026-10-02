//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::md_four::MdFour;

/// 版本枚举：DeletedItemDigestVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DeletedItemDigestVariant {
    Default(Box<MdFour>),
    V5_0(Box<Hash128>),
}

impl crate::Parse for DeletedItemDigestVariant {
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

impl crate::ToValue for DeletedItemDigestVariant {
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
pub struct DeletedItem {
    pub changeset: i32,
    pub guid: Guid,
    pub parent: Guid,
    pub full_path: String,
    pub r#type: i32,
    pub digest: DeletedItemDigestVariant,
}

impl crate::Parse for DeletedItem {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let changeset = rdr.read_i32()?;
        let guid = Guid::parse(rdr, ver)?;
        let parent = Guid::parse(rdr, ver)?;
        let full_path = rdr.read_aligned_string()?;
        let r#type = rdr.read_i32()?;
        let digest = DeletedItemDigestVariant::parse(rdr, ver)?;

        Ok(DeletedItem {
            changeset,
            guid,
            parent,
            full_path,
            r#type,
            digest,
        })
    }
}

impl crate::ToValue for DeletedItem {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("changeset".to_string(), self.changeset.to_value())),
            Some(("guid".to_string(), self.guid.to_value())),
            Some(("parent".to_string(), self.parent.to_value())),
            Some(("fullPath".to_string(), self.full_path.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("digest".to_string(), self.digest.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "changeset" => (self.changeset).inspect_value(path, offset, limit),
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "parent" => (self.parent).inspect_value(path, offset, limit),
                "fullPath" => (self.full_path).inspect_value(path, offset, limit),
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "digest" => (self.digest).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.changeset).inspect_value(path, offset, limit))
                .map(|v| ("changeset".to_string(), v)),
            ((self.guid).inspect_value(path, offset, limit)).map(|v| ("guid".to_string(), v)),
            ((self.parent).inspect_value(path, offset, limit)).map(|v| ("parent".to_string(), v)),
            ((self.full_path).inspect_value(path, offset, limit))
                .map(|v| ("fullPath".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.digest).inspect_value(path, offset, limit)).map(|v| ("digest".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
