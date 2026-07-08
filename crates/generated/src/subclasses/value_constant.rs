//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ValueConstant {
    pub m_id: u32,
    pub m_type_id: Option<u32>,
    pub m_type: u32,
    pub m_index: u32,
}

impl crate::Parse for ValueConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_id = rdr.read_u32()?;
        let m_type_id = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let m_type = rdr.read_u32()?;
        let m_index = rdr.read_u32()?;

        Ok(ValueConstant {
            m_id,
            m_type_id,
            m_type,
            m_index,
        })
    }
}
