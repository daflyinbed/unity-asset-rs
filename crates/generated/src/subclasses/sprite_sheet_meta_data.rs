//! 自动生成。请勿手动修改。

use crate::subclasses::sprite_meta_data::SpriteMetaData;

#[derive(Debug, Clone)]
pub struct SpriteSheetMetaData {
    pub m_sprites: Vec<SpriteMetaData>,
}

impl crate::Parse for SpriteSheetMetaData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_sprites = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SpriteMetaData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(SpriteSheetMetaData {
            m_sprites,
        })
    }
}
