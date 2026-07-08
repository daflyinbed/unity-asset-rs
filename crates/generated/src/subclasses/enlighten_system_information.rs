//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct EnlightenSystemInformation {
    pub renderer_index: u32,
    pub renderer_size: u32,
    pub atlas_index: i32,
    pub atlas_offset_x: i32,
    pub atlas_offset_y: i32,
    pub input_system_hash: Hash128,
    pub radiosity_system_hash: Hash128,
}

impl crate::Parse for EnlightenSystemInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let renderer_index = rdr.read_u32()?;
        let renderer_size = rdr.read_u32()?;
        let atlas_index = rdr.read_i32()?;
        let atlas_offset_x = rdr.read_i32()?;
        let atlas_offset_y = rdr.read_i32()?;
        let input_system_hash = Hash128::parse(rdr, ver)?;
        let radiosity_system_hash = Hash128::parse(rdr, ver)?;

        Ok(EnlightenSystemInformation {
            renderer_index,
            renderer_size,
            atlas_index,
            atlas_offset_x,
            atlas_offset_y,
            input_system_hash,
            radiosity_system_hash,
        })
    }
}
