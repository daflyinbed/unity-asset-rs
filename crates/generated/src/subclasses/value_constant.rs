//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ValueConstant {
    pub m_id: u32,
    pub m_type_id: u32,
    pub m_type: u32,
    pub m_index: u32,
}

impl crate::Parse for ValueConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_id = r.read_u32()?;
        let m_type_id = r.read_u32()?;
        let m_type = r.read_u32()?;
        let m_index = r.read_u32()?;

        Ok(ValueConstant {
            m_id,
            m_type_id,
            m_type,
            m_index,
        })
    }
}
