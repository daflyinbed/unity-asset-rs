//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Image {
    pub m_format: u32,
    pub m_width: i32,
    pub m_height: i32,
    pub m_row_bytes: i32,
    pub image_data: Vec<u8>,
}

impl crate::Parse for Image {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_format = r.read_u32()?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_row_bytes = r.read_i32()?;
        let image_data = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;

        Ok(Image {
            m_format,
            m_width,
            m_height,
            m_row_bytes,
            image_data,
        })
    }
}
