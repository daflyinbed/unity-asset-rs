//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EnlightenTerrainChunksInformation {
    pub first_system_id: i32,
    pub num_chunks_in_x: i32,
    pub num_chunks_in_y: i32,
}

impl crate::Parse for EnlightenTerrainChunksInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let first_system_id = rdr.read_i32()?;
        let num_chunks_in_x = rdr.read_i32()?;
        let num_chunks_in_y = rdr.read_i32()?;

        Ok(EnlightenTerrainChunksInformation {
            first_system_id,
            num_chunks_in_x,
            num_chunks_in_y,
        })
    }
}
