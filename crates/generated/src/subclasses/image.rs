//! 自动生成。请勿手动修改。


/// 版本枚举：ImageMFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ImageMFormatVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for ImageMFormatVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Image {
    pub m_format: ImageMFormatVariant,
    pub m_width: i32,
    pub m_height: i32,
    pub m_row_bytes: i32,
    pub image_data: Vec<u8>,
}

impl crate::Parse for Image {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_format = ImageMFormatVariant::parse(rdr, ver)?;
        let m_width = rdr.read_i32()?;
        let m_height = rdr.read_i32()?;
        let m_row_bytes = rdr.read_i32()?;
        let image_data = ( { let n = rdr.read_i32()? as usize; rdr.read_bytes(n) } )?;
        rdr.align(4)?;

        Ok(Image {
            m_format,
            m_width,
            m_height,
            m_row_bytes,
            image_data,
        })
    }
}
