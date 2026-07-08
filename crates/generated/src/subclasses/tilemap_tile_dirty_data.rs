//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TilemapTileDirtyData {
    pub m_dirty_index: u32,
}

impl crate::Parse for TilemapTileDirtyData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_dirty_index = rdr.read_u32()?;

        Ok(TilemapTileDirtyData {
            m_dirty_index,
        })
    }
}
