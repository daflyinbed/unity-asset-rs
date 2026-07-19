//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::color_rgba32::ColorRgba32;

/// 版本枚举：GradientKey0Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientKey0Variant {
    Default(Box<ColorRgba32>),
    V5_6(Box<ColorRgbAf>),
}

impl crate::Parse for GradientKey0Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GradientKey1Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientKey1Variant {
    Default(Box<ColorRgba32>),
    V5_6(Box<ColorRgbAf>),
}

impl crate::Parse for GradientKey1Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GradientKey2Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientKey2Variant {
    Default(Box<ColorRgba32>),
    V5_6(Box<ColorRgbAf>),
}

impl crate::Parse for GradientKey2Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GradientKey3Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientKey3Variant {
    Default(Box<ColorRgba32>),
    V5_6(Box<ColorRgbAf>),
}

impl crate::Parse for GradientKey3Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GradientKey4Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientKey4Variant {
    Default(Box<ColorRgba32>),
    V5_6(Box<ColorRgbAf>),
}

impl crate::Parse for GradientKey4Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GradientKey5Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientKey5Variant {
    Default(Box<ColorRgba32>),
    V5_6(Box<ColorRgbAf>),
}

impl crate::Parse for GradientKey5Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GradientKey6Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientKey6Variant {
    Default(Box<ColorRgba32>),
    V5_6(Box<ColorRgbAf>),
}

impl crate::Parse for GradientKey6Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：GradientKey7Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GradientKey7Variant {
    Default(Box<ColorRgba32>),
    V5_6(Box<ColorRgbAf>),
}

impl crate::Parse for GradientKey7Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

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
    pub m_color_0: Option<ColorRgba32>,
    pub key0: Option<GradientKey0Variant>,
    pub m_color_1: Option<ColorRgba32>,
    pub key1: Option<GradientKey1Variant>,
    pub m_color_2: Option<ColorRgba32>,
    pub key2: Option<GradientKey2Variant>,
    pub m_color_3: Option<ColorRgba32>,
    pub key3: Option<GradientKey3Variant>,
    pub m_color_4: Option<ColorRgba32>,
    pub key4: Option<GradientKey4Variant>,
    pub key5: Option<GradientKey5Variant>,
    pub key6: Option<GradientKey6Variant>,
    pub key7: Option<GradientKey7Variant>,
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
    pub m_num_color_keys: Option<u8>,
    pub m_mode: Option<GradientMModeVariant>,
    pub m_num_alpha_keys: Option<u8>,
    pub m_color_space: Option<i8>,
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
        let mut m_num_color_keys = None;
        let mut m_mode = None;
        let mut m_num_alpha_keys = None;
        let mut m_color_space = None;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            key0 = Some(GradientKey0Variant::parse(rdr, ver)?);
            key1 = Some(GradientKey1Variant::parse(rdr, ver)?);
            key2 = Some(GradientKey2Variant::parse(rdr, ver)?);
            key3 = Some(GradientKey3Variant::parse(rdr, ver)?);
            key4 = Some(GradientKey4Variant::parse(rdr, ver)?);
            key5 = Some(GradientKey5Variant::parse(rdr, ver)?);
            key6 = Some(GradientKey6Variant::parse(rdr, ver)?);
            key7 = Some(GradientKey7Variant::parse(rdr, ver)?);
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
            key0 = Some(GradientKey0Variant::parse(rdr, ver)?);
            key1 = Some(GradientKey1Variant::parse(rdr, ver)?);
            key2 = Some(GradientKey2Variant::parse(rdr, ver)?);
            key3 = Some(GradientKey3Variant::parse(rdr, ver)?);
            key4 = Some(GradientKey4Variant::parse(rdr, ver)?);
            key5 = Some(GradientKey5Variant::parse(rdr, ver)?);
            key6 = Some(GradientKey6Variant::parse(rdr, ver)?);
            key7 = Some(GradientKey7Variant::parse(rdr, ver)?);
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
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            key0 = Some(GradientKey0Variant::parse(rdr, ver)?);
            key1 = Some(GradientKey1Variant::parse(rdr, ver)?);
            key2 = Some(GradientKey2Variant::parse(rdr, ver)?);
            key3 = Some(GradientKey3Variant::parse(rdr, ver)?);
            key4 = Some(GradientKey4Variant::parse(rdr, ver)?);
            key5 = Some(GradientKey5Variant::parse(rdr, ver)?);
            key6 = Some(GradientKey6Variant::parse(rdr, ver)?);
            key7 = Some(GradientKey7Variant::parse(rdr, ver)?);
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
            m_num_color_keys = Some(rdr.read_u8()?);
            m_num_alpha_keys = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 3) {
            m_color_0 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_1 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_2 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_3 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_4 = Some(ColorRgba32::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            key0 = Some(GradientKey0Variant::parse(rdr, ver)?);
            key1 = Some(GradientKey1Variant::parse(rdr, ver)?);
            key2 = Some(GradientKey2Variant::parse(rdr, ver)?);
            key3 = Some(GradientKey3Variant::parse(rdr, ver)?);
            key4 = Some(GradientKey4Variant::parse(rdr, ver)?);
            key5 = Some(GradientKey5Variant::parse(rdr, ver)?);
            key6 = Some(GradientKey6Variant::parse(rdr, ver)?);
            key7 = Some(GradientKey7Variant::parse(rdr, ver)?);
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
            m_num_color_keys = Some(rdr.read_u8()?);
            m_num_alpha_keys = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_color_0 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_1 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_2 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_3 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_4 = Some(ColorRgba32::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            key0 = Some(GradientKey0Variant::parse(rdr, ver)?);
            key1 = Some(GradientKey1Variant::parse(rdr, ver)?);
            key2 = Some(GradientKey2Variant::parse(rdr, ver)?);
            key3 = Some(GradientKey3Variant::parse(rdr, ver)?);
            key4 = Some(GradientKey4Variant::parse(rdr, ver)?);
            key5 = Some(GradientKey5Variant::parse(rdr, ver)?);
            key6 = Some(GradientKey6Variant::parse(rdr, ver)?);
            key7 = Some(GradientKey7Variant::parse(rdr, ver)?);
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
            m_num_color_keys = Some(rdr.read_u8()?);
            m_num_alpha_keys = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else {
            m_color_0 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_1 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_2 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_3 = Some(ColorRgba32::parse(rdr, ver)?);
            m_color_4 = Some(ColorRgba32::parse(rdr, ver)?);
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
            m_num_color_keys,
            m_mode,
            m_num_alpha_keys,
            m_color_space,
        })
    }
}
