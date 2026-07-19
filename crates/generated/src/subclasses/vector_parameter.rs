//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VectorParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_offset_in_constant_buffer: Option<i32>,
    pub m_array_size: i32,
    pub m_type: i8,
    pub m_dim: i8,
}

impl crate::Parse for VectorParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_offset_in_constant_buffer = None;
        let mut m_array_size = None;
        let mut m_type = None;
        let mut m_dim = None;
        m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_offset_in_constant_buffer = Some(rdr.read_i32()?);
        } else {
            m_index = Some(rdr.read_i32()?);
        }
        m_array_size = Some(rdr.read_i32()?);
        m_type = Some(rdr.read_i8()?);
        m_dim = Some(rdr.read_i8()?);
        rdr.align(4)?;

        Ok(VectorParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_offset_in_constant_buffer,
            m_array_size: m_array_size.unwrap(),
            m_type: m_type.unwrap(),
            m_dim: m_dim.unwrap(),
        })
    }
}
