//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct CachedAssetMetaData {
    pub guid: Guid,
    pub path_name: String,
    pub original_changeset: u32,
    pub original_name: String,
    pub original_parent: Guid,
    pub original_digest: MdFour,
}

impl crate::Parse for CachedAssetMetaData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(r, ver)?;
        let path_name = r.read_aligned_string()?;
        r.align(4)?;
        let original_changeset = r.read_u32()?;
        let original_name = r.read_aligned_string()?;
        r.align(4)?;
        let original_parent = Guid::parse(r, ver)?;
        let original_digest = MdFour::parse(r, ver)?;
        r.align(4)?;

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
