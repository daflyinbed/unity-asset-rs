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
            let v = ( { Ok(Box::new(Hash128::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { Ok(Box::new(MdFour::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
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
