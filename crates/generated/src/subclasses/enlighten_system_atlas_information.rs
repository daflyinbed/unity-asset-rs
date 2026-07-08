//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct EnlightenSystemAtlasInformation {
    pub atlas_size: i32,
    pub atlas_hash: Hash128,
    pub first_system_id: i32,
}

impl crate::Parse for EnlightenSystemAtlasInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let atlas_size = rdr.read_i32()?;
        let atlas_hash = Hash128::parse(rdr, ver)?;
        let first_system_id = rdr.read_i32()?;

        Ok(EnlightenSystemAtlasInformation {
            atlas_size,
            atlas_hash,
            first_system_id,
        })
    }
}
