//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::md_four::MdFour;

/// 版本枚举：ItemDigestVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ItemDigestVariant {
    Default(Box<MdFour>),
    V5_0(Box<Hash128>),
}

impl crate::Parse for ItemDigestVariant {
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

impl crate::ToValue for ItemDigestVariant {
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
pub struct Item {
    pub changeset: i32,
    pub guid: Guid,
    pub name: String,
    pub parent: Guid,
    pub int_type: i32,
    pub digest: ItemDigestVariant,
    pub int_origin: i32,
    pub old_version: i32,
    pub parent_folder_id: i32,
    pub int_change_flags: i32,
}

impl crate::Parse for Item {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let changeset = rdr.read_i32()?;
        let guid = Guid::parse(rdr, ver)?;
        let name = rdr.read_aligned_string()?;
        let parent = Guid::parse(rdr, ver)?;
        let int_type = rdr.read_i32()?;
        let digest = ItemDigestVariant::parse(rdr, ver)?;
        let int_origin = rdr.read_i32()?;
        let old_version = rdr.read_i32()?;
        let parent_folder_id = rdr.read_i32()?;
        let int_change_flags = rdr.read_i32()?;

        Ok(Item {
            changeset,
            guid,
            name,
            parent,
            int_type,
            digest,
            int_origin,
            old_version,
            parent_folder_id,
            int_change_flags,
        })
    }
}

impl crate::ToValue for Item {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("changeset".to_string(), self.changeset.to_value())),
            Some(("guid".to_string(), self.guid.to_value())),
            Some(("name".to_string(), self.name.to_value())),
            Some(("parent".to_string(), self.parent.to_value())),
            Some(("(int&)type".to_string(), self.int_type.to_value())),
            Some(("digest".to_string(), self.digest.to_value())),
            Some(("(int&)origin".to_string(), self.int_origin.to_value())),
            Some(("oldVersion".to_string(), self.old_version.to_value())),
            Some((
                "parentFolderID".to_string(),
                self.parent_folder_id.to_value(),
            )),
            Some((
                "(int&)changeFlags".to_string(),
                self.int_change_flags.to_value(),
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
                "changeset" => (self.changeset).inspect_value(path, offset, limit),
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                "parent" => (self.parent).inspect_value(path, offset, limit),
                "(int&)type" => (self.int_type).inspect_value(path, offset, limit),
                "digest" => (self.digest).inspect_value(path, offset, limit),
                "(int&)origin" => (self.int_origin).inspect_value(path, offset, limit),
                "oldVersion" => (self.old_version).inspect_value(path, offset, limit),
                "parentFolderID" => (self.parent_folder_id).inspect_value(path, offset, limit),
                "(int&)changeFlags" => (self.int_change_flags).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.changeset).inspect_value(path, offset, limit))
                .map(|v| ("changeset".to_string(), v)),
            ((self.guid).inspect_value(path, offset, limit)).map(|v| ("guid".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.parent).inspect_value(path, offset, limit)).map(|v| ("parent".to_string(), v)),
            ((self.int_type).inspect_value(path, offset, limit))
                .map(|v| ("(int&)type".to_string(), v)),
            ((self.digest).inspect_value(path, offset, limit)).map(|v| ("digest".to_string(), v)),
            ((self.int_origin).inspect_value(path, offset, limit))
                .map(|v| ("(int&)origin".to_string(), v)),
            ((self.old_version).inspect_value(path, offset, limit))
                .map(|v| ("oldVersion".to_string(), v)),
            ((self.parent_folder_id).inspect_value(path, offset, limit))
                .map(|v| ("parentFolderID".to_string(), v)),
            ((self.int_change_flags).inspect_value(path, offset, limit))
                .map(|v| ("(int&)changeFlags".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
