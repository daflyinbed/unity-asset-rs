//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BufferBinding {
    pub m_name_index: i32,
    pub m_index: i32,
}

impl crate::Parse for BufferBinding {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = r.read_i32()?;
        let m_index = r.read_i32()?;

        Ok(BufferBinding {
            m_name_index,
            m_index,
        })
    }
}
