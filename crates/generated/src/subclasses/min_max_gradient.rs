//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::color_rgba32::ColorRgba32;
use crate::subclasses::gradient::Gradient;


/// 版本枚举：MinMaxGradientMinMaxStateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MinMaxGradientMinMaxStateVariant {
    Default(i16),
    V5_6_1(u16),
}

impl crate::Parse for MinMaxGradientMinMaxStateVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = rdr.read_u16()?;
            rdr.align(4)?;
            Ok(Self::V5_6_1(v))
        } else {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MinMaxGradientMinColorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MinMaxGradientMinColorVariant {
    Default(Box<ColorRgba32>),
    V5_4(Box<ColorRgbAf>),
}

impl crate::Parse for MinMaxGradientMinColorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_4(v))
        } else {
            let v = ( { Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MinMaxGradientMaxColorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MinMaxGradientMaxColorVariant {
    Default(Box<ColorRgba32>),
    V5_4(Box<ColorRgbAf>),
}

impl crate::Parse for MinMaxGradientMaxColorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { Ok(Box::new(ColorRgbAf::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_4(v))
        } else {
            let v = ( { Ok(Box::new(ColorRgba32::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct MinMaxGradient {
    pub max_gradient: Gradient,
    pub min_max_state: MinMaxGradientMinMaxStateVariant,
    pub min_gradient: Gradient,
    pub min_color: MinMaxGradientMinColorVariant,
    pub max_color: MinMaxGradientMaxColorVariant,
}

impl crate::Parse for MinMaxGradient {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut max_gradient = None;
        let mut min_max_state = None;
        let mut min_gradient = None;
        let mut min_color = None;
        let mut max_color = None;
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            min_max_state = Some(MinMaxGradientMinMaxStateVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            min_max_state = Some(MinMaxGradientMinMaxStateVariant::parse(rdr, ver)?);
            max_gradient = Some(Gradient::parse(rdr, ver)?);
            min_gradient = Some(Gradient::parse(rdr, ver)?);
        } else {
            max_gradient = Some(Gradient::parse(rdr, ver)?);
            min_gradient = Some(Gradient::parse(rdr, ver)?);
        }
            min_color = Some(MinMaxGradientMinColorVariant::parse(rdr, ver)?);
            max_color = Some(MinMaxGradientMaxColorVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            max_gradient = Some(Gradient::parse(rdr, ver)?);
            min_gradient = Some(Gradient::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
        } else {
            min_max_state = Some(MinMaxGradientMinMaxStateVariant::parse(rdr, ver)?);
        }

        Ok(MinMaxGradient {
            max_gradient: max_gradient.unwrap(),
            min_max_state: min_max_state.unwrap(),
            min_gradient: min_gradient.unwrap(),
            min_color: min_color.unwrap(),
            max_color: max_color.unwrap(),
        })
    }
}
