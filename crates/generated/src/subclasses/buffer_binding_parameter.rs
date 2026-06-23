//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BufferBindingParameter {
    pub m_name_index: i32,
    pub m_index: i32,
    pub m_array_size: i32,
}

impl crate::Parse for BufferBindingParameter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = r.read_i32()?;
        let m_index = r.read_i32()?;
        let m_array_size = r.read_i32()?;

        Ok(BufferBindingParameter {
            m_name_index,
            m_index,
            m_array_size,
        })
    }
}
