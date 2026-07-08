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
        let mut m_id = None;
        let mut m_type_id = None;
        let mut m_type = None;
        let mut m_index = None;
            m_id = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_type_id = Some(rdr.read_u32()?);
        }
            m_type = Some(rdr.read_u32()?);
            m_index = Some(rdr.read_u32()?);

        Ok(ValueConstant {
            m_id: m_id.unwrap(),
            m_type_id,
            m_type: m_type.unwrap(),
            m_index: m_index.unwrap(),
        })
    }
}
