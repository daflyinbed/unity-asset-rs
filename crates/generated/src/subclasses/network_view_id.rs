//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NetworkViewId {
    pub m_id: u32,
    pub m_type: u32,
}

impl crate::Parse for NetworkViewId {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_id = rdr.read_u32()?;
        let m_type = rdr.read_u32()?;

        Ok(NetworkViewId { m_id, m_type })
    }
}
