//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SpriteData {
    pub sprite: PPtr,
}

impl crate::Parse for SpriteData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let sprite = crate::PPtr::parse(r, ver)?;

        Ok(SpriteData {
            sprite,
        })
    }
}
