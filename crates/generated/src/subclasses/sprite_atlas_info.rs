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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(r, ver)?;
        let atlas = crate::PPtr::parse(r, ver)?;

        Ok(SpriteAtlasInfo {
            guid,
            atlas,
        })
    }
}
