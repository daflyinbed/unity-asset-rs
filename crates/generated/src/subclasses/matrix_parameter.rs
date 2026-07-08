//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MatrixParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_offset_in_constant_buffer: Option<i32>,
    pub m_array_size: i32,
    pub m_type: i8,
    pub m_row_count: i8,
}

impl crate::Parse for MatrixParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = rdr.read_i32()?;
        let m_index = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_offset_in_constant_buffer = if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_array_size = rdr.read_i32()?;
        let m_type = rdr.read_i8()?;
        let m_row_count = rdr.read_i8()?;
        rdr.align(4)?;

        Ok(MatrixParameter {
            m_name_index,
            m_index,
            m_offset_in_constant_buffer,
            m_array_size,
            m_type,
            m_row_count,
        })
    }
}
