//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Binding {
    pub m_set: u32,
    pub m_slot: u32,
}

impl crate::Parse for Binding {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_set = rdr.read_u32()?;
        let m_slot = rdr.read_u32()?;

        Ok(Binding {
            m_set,
            m_slot,
        })
    }
}
