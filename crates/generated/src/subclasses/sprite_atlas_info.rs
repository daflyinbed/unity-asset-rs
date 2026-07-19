//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct SpriteAtlasInfo {
    pub guid: Guid,
    pub atlas: PPtr,
}

impl crate::Parse for SpriteAtlasInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(rdr, ver)?;
        let atlas = crate::PPtr::parse(rdr, ver)?;

        Ok(SpriteAtlasInfo { guid, atlas })
    }
}
