//! 自动生成。请勿手动修改。

use crate::subclasses::sprite_custom_data_entry::SpriteCustomDataEntry;

#[derive(Debug, Clone)]
pub struct SpriteCustomMetadata {
    pub m_entries: Vec<SpriteCustomDataEntry>,
}

impl crate::Parse for SpriteCustomMetadata {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_entries = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SpriteCustomDataEntry::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(SpriteCustomMetadata {
            m_entries,
        })
    }
}
