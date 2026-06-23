//! 自动生成。请勿手动修改。

use crate::subclasses::rectf::Rectf;

#[derive(Debug, Clone)]
pub struct CharacterInfo {
    pub index: u32,
    pub uv: Rectf,
    pub vert: Rectf,
    pub width: f32,
}

impl crate::Parse for CharacterInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let index = r.read_u32()?;
        let uv = Rectf::parse(r, ver)?;
        let vert = Rectf::parse(r, ver)?;
        let width = r.read_f32()?;

        Ok(CharacterInfo {
            index,
            uv,
            vert,
            width,
        })
    }
}
