//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VectorParameter {
    pub m_name_index: i32,
    pub m_index: i32,
    pub m_array_size: i32,
    pub m_type: i8,
    pub m_dim: i8,
}

impl crate::Parse for VectorParameter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = r.read_i32()?;
        let m_index = r.read_i32()?;
        let m_array_size = r.read_i32()?;
        let m_type = r.read_i8()?;
        let m_dim = r.read_i8()?;
        r.align(4)?;

        Ok(VectorParameter {
            m_name_index,
            m_index,
            m_array_size,
            m_type,
            m_dim,
        })
    }
}
