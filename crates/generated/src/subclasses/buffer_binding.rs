//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BufferBinding {
    pub m_name_index: i32,
    pub m_index: i32,
    pub m_array_size: Option<i32>,
}

impl crate::Parse for BufferBinding {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = rdr.read_i32()?;
        let m_index = rdr.read_i32()?;
        let m_array_size = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(BufferBinding {
            m_name_index,
            m_index,
            m_array_size,
        })
    }
}
