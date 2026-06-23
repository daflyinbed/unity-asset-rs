//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MeshLodRange {
    pub m_index_start: u32,
    pub m_index_count: u32,
}

impl crate::Parse for MeshLodRange {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_index_start = r.read_u32()?;
        let m_index_count = r.read_u32()?;
        r.align(4)?;

        Ok(MeshLodRange {
            m_index_start,
            m_index_count,
        })
    }
}
