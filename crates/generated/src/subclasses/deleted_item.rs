//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct DeletedItem {
    pub changeset: i32,
    pub guid: Guid,
    pub parent: Guid,
    pub full_path: String,
    pub r#type: i32,
    pub digest: MdFour,
}

impl crate::Parse for DeletedItem {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let changeset = r.read_i32()?;
        let guid = Guid::parse(r, ver)?;
        let parent = Guid::parse(r, ver)?;
        let full_path = r.read_aligned_string()?;
        r.align(4)?;
        let r#type = r.read_i32()?;
        let digest = MdFour::parse(r, ver)?;
        r.align(4)?;

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
