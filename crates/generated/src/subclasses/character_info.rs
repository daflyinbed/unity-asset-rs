//! 自动生成。请勿手动修改。

use crate::subclasses::rectf::Rectf;

#[derive(Debug, Clone)]
pub struct CharacterInfo {
    pub index: u32,
    pub uv: Rectf,
    pub vert: Rectf,
    pub width: Option<f32>,
    pub advance: Option<f32>,
    pub flipped: Option<bool>,
}

impl crate::Parse for CharacterInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let index = rdr.read_u32()?;
        let uv = Rectf::parse(rdr, ver)?;
        let vert = Rectf::parse(rdr, ver)?;
        let width = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let advance = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let flipped = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(CharacterInfo {
            index,
            uv,
            vert,
            width,
            advance,
            flipped,
        })
    }
}
