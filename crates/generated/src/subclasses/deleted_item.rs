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
