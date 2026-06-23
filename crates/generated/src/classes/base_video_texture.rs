//! 自动生成。请勿手动修改。

use super::texture::Texture;

#[derive(Debug, Clone)]
pub struct BaseVideoTexture {
    pub parent: Texture,
}

impl crate::Parse for BaseVideoTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Texture::parse(r, ver)?;

        Ok(BaseVideoTexture {
            parent,
        })
    }
}
