//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;


/// 版本枚举：GradientMModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientMModeVariant {
    Default(i32),
    V2022_2(u8),
}

impl crate::Parse for GradientMModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2022_2(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Gradient {
    pub m_color_0: Option<ColorRgba>,
    pub key0: Option<ColorRgba>,
    pub m_color_1: Option<ColorRgba>,
    pub key1: Option<ColorRgba>,
    pub m_color_2: Option<ColorRgba>,
    pub key2: Option<ColorRgba>,
    pub m_color_3: Option<ColorRgba>,
    pub key3: Option<ColorRgba>,
    pub m_color_4: Option<ColorRgba>,
    pub key4: Option<ColorRgba>,
    pub key5: Option<ColorRgba>,
    pub key6: Option<ColorRgba>,
    pub key7: Option<ColorRgba>,
    pub ctime0: Option<u16>,
    pub ctime1: Option<u16>,
    pub ctime2: Option<u16>,
    pub ctime3: Option<u16>,
    pub ctime4: Option<u16>,
    pub ctime5: Option<u16>,
    pub ctime6: Option<u16>,
    pub ctime7: Option<u16>,
    pub atime0: Option<u16>,
    pub atime1: Option<u16>,
    pub atime2: Option<u16>,
    pub atime3: Option<u16>,
    pub atime4: Option<u16>,
    pub atime5: Option<u16>,
    pub atime6: Option<u16>,
    pub atime7: Option<u16>,
    pub m_mode: Option<GradientMModeVariant>,
    pub m_num_color_keys: Option<u8>,
    pub m_color_space: Option<i8>,
    pub m_num_alpha_keys: Option<u8>,
}

impl crate::Parse for Gradient {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_color_0 = None;
        let mut key0 = None;
        let mut m_color_1 = None;
        let mut key1 = None;
        let mut m_color_2 = None;
        let mut key2 = None;
        let mut m_color_3 = None;
        let mut key3 = None;
        let mut m_color_4 = None;
        let mut key4 = None;
        let mut key5 = None;
        let mut key6 = None;
        let mut key7 = None;
        let mut ctime0 = None;
        let mut ctime1 = None;
        let mut ctime2 = None;
        let mut ctime3 = None;
        let mut ctime4 = None;
        let mut ctime5 = None;
        let mut ctime6 = None;
        let mut ctime7 = None;
        let mut atime0 = None;
        let mut atime1 = None;
        let mut atime2 = None;
        let mut atime3 = None;
        let mut atime4 = None;
        let mut atime5 = None;
        let mut atime6 = None;
        let mut atime7 = None;
        let mut m_mode = None;
        let mut m_num_color_keys = None;
        let mut m_color_space = None;
        let mut m_num_alpha_keys = None;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            key0 = Some(ColorRgba::parse(rdr, ver)?);
            key1 = Some(ColorRgba::parse(rdr, ver)?);
            key2 = Some(ColorRgba::parse(rdr, ver)?);
            key3 = Some(ColorRgba::parse(rdr, ver)?);
            key4 = Some(ColorRgba::parse(rdr, ver)?);
            key5 = Some(ColorRgba::parse(rdr, ver)?);
            key6 = Some(ColorRgba::parse(rdr, ver)?);
            key7 = Some(ColorRgba::parse(rdr, ver)?);
            ctime0 = Some(rdr.read_u16()?);
            ctime1 = Some(rdr.read_u16()?);
            ctime2 = Some(rdr.read_u16()?);
            ctime3 = Some(rdr.read_u16()?);
            ctime4 = Some(rdr.read_u16()?);
            ctime5 = Some(rdr.read_u16()?);
            ctime6 = Some(rdr.read_u16()?);
            ctime7 = Some(rdr.read_u16()?);
            atime0 = Some(rdr.read_u16()?);
            atime1 = Some(rdr.read_u16()?);
            atime2 = Some(rdr.read_u16()?);
            atime3 = Some(rdr.read_u16()?);
            atime4 = Some(rdr.read_u16()?);
            atime5 = Some(rdr.read_u16()?);
            atime6 = Some(rdr.read_u16()?);
            atime7 = Some(rdr.read_u16()?);
            m_mode = Some(GradientMModeVariant::parse(rdr, ver)?);
            m_color_space = Some(rdr.read_i8()?);
            m_num_color_keys = Some(rdr.read_u8()?);
            m_num_alpha_keys = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            key0 = Some(ColorRgba::parse(rdr, ver)?);
            key1 = Some(ColorRgba::parse(rdr, ver)?);
            key2 = Some(ColorRgba::parse(rdr, ver)?);
            key3 = Some(ColorRgba::parse(rdr, ver)?);
            key4 = Some(ColorRgba::parse(rdr, ver)?);
            key5 = Some(ColorRgba::parse(rdr, ver)?);
            key6 = Some(ColorRgba::parse(rdr, ver)?);
            key7 = Some(ColorRgba::parse(rdr, ver)?);
            ctime0 = Some(rdr.read_u16()?);
            ctime1 = Some(rdr.read_u16()?);
            ctime2 = Some(rdr.read_u16()?);
            ctime3 = Some(rdr.read_u16()?);
            ctime4 = Some(rdr.read_u16()?);
            ctime5 = Some(rdr.read_u16()?);
            ctime6 = Some(rdr.read_u16()?);
            ctime7 = Some(rdr.read_u16()?);
            atime0 = Some(rdr.read_u16()?);
            atime1 = Some(rdr.read_u16()?);
            atime2 = Some(rdr.read_u16()?);
            atime3 = Some(rdr.read_u16()?);
            atime4 = Some(rdr.read_u16()?);
            atime5 = Some(rdr.read_u16()?);
            atime6 = Some(rdr.read_u16()?);
            atime7 = Some(rdr.read_u16()?);
            m_mode = Some(GradientMModeVariant::parse(rdr, ver)?);
            m_num_color_keys = Some(rdr.read_u8()?);
            m_num_alpha_keys = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else {
            m_color_0 = Some(ColorRgba::parse(rdr, ver)?);
            m_color_1 = Some(ColorRgba::parse(rdr, ver)?);
            m_color_2 = Some(ColorRgba::parse(rdr, ver)?);
            m_color_3 = Some(ColorRgba::parse(rdr, ver)?);
            m_color_4 = Some(ColorRgba::parse(rdr, ver)?);
        }

        Ok(Gradient {
            m_color_0,
            key0,
            m_color_1,
            key1,
            m_color_2,
            key2,
            m_color_3,
            key3,
            m_color_4,
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
            m_mode,
            m_num_color_keys,
            m_color_space,
            m_num_alpha_keys,
        })
    }
}
