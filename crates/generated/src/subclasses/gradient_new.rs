//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct GradientNew {
    pub key0: ColorRgba,
    pub key1: ColorRgba,
    pub key2: ColorRgba,
    pub key3: ColorRgba,
    pub key4: ColorRgba,
    pub key5: ColorRgba,
    pub key6: ColorRgba,
    pub key7: ColorRgba,
    pub ctime0: u16,
    pub ctime1: u16,
    pub ctime2: u16,
    pub ctime3: u16,
    pub ctime4: u16,
    pub ctime5: u16,
    pub ctime6: u16,
    pub ctime7: u16,
    pub atime0: u16,
    pub atime1: u16,
    pub atime2: u16,
    pub atime3: u16,
    pub atime4: u16,
    pub atime5: u16,
    pub atime6: u16,
    pub atime7: u16,
    pub m_num_color_keys: u8,
    pub m_num_alpha_keys: u8,
}

impl crate::Parse for GradientNew {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let key0 = ColorRgba::parse(rdr, ver)?;
        let key1 = ColorRgba::parse(rdr, ver)?;
        let key2 = ColorRgba::parse(rdr, ver)?;
        let key3 = ColorRgba::parse(rdr, ver)?;
        let key4 = ColorRgba::parse(rdr, ver)?;
        let key5 = ColorRgba::parse(rdr, ver)?;
        let key6 = ColorRgba::parse(rdr, ver)?;
        let key7 = ColorRgba::parse(rdr, ver)?;
        let ctime0 = rdr.read_u16()?;
        let ctime1 = rdr.read_u16()?;
        let ctime2 = rdr.read_u16()?;
        let ctime3 = rdr.read_u16()?;
        let ctime4 = rdr.read_u16()?;
        let ctime5 = rdr.read_u16()?;
        let ctime6 = rdr.read_u16()?;
        let ctime7 = rdr.read_u16()?;
        let atime0 = rdr.read_u16()?;
        let atime1 = rdr.read_u16()?;
        let atime2 = rdr.read_u16()?;
        let atime3 = rdr.read_u16()?;
        let atime4 = rdr.read_u16()?;
        let atime5 = rdr.read_u16()?;
        let atime6 = rdr.read_u16()?;
        let atime7 = rdr.read_u16()?;
        let m_num_color_keys = rdr.read_u8()?;
        let m_num_alpha_keys = rdr.read_u8()?;
        rdr.align(4)?;

        Ok(GradientNew {
            key0,
            key1,
            key2,
            key3,
            key4,
            key5,
            key6,
            key7,
            ctime0,
            ctime1,
            ctime2,
            ctime3,
            ctime4,
            ctime5,
            ctime6,
            ctime7,
            atime0,
            atime1,
            atime2,
            atime3,
            atime4,
            atime5,
            atime6,
            atime7,
            m_num_color_keys,
            m_num_alpha_keys,
        })
    }
}
