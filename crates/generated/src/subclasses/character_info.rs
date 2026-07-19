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
        let mut index = None;
        let mut uv = None;
        let mut vert = None;
        let mut width = None;
        let mut advance = None;
        let mut flipped = None;
        index = Some(rdr.read_u32()?);
        uv = Some(Rectf::parse(rdr, ver)?);
        vert = Some(Rectf::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            advance = Some(rdr.read_f32()?);
            flipped = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            width = Some(rdr.read_f32()?);
            flipped = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            width = Some(rdr.read_f32()?);
        }

        Ok(CharacterInfo {
            index: index.unwrap(),
            uv: uv.unwrap(),
            vert: vert.unwrap(),
            width,
            advance,
            flipped,
        })
    }
}
