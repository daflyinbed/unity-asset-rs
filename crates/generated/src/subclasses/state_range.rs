//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StateRange {
    pub m_start_index: u32,
    pub m_count: u32,
}

impl crate::Parse for StateRange {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_start_index = r.read_u32()?;
        let m_count = r.read_u32()?;

        Ok(StateRange {
            m_start_index,
            m_count,
        })
    }
}
