//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SecondarySpriteTexture {
    pub texture: PPtr,
    pub name: String,
}

impl crate::Parse for SecondarySpriteTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture = crate::PPtr::parse(r, ver)?;
        let name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(SecondarySpriteTexture {
            texture,
            name,
        })
    }
}
