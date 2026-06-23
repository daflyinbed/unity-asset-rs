//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::gradient_new::GradientNew;

#[derive(Debug, Clone)]
pub struct MinMaxGradient {
    pub max_gradient: GradientNew,
    pub min_gradient: GradientNew,
    pub min_color: ColorRgba,
    pub max_color: ColorRgba,
    pub min_max_state: i16,
}

impl crate::Parse for MinMaxGradient {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let max_gradient = GradientNew::parse(r, ver)?;
        r.align(4)?;
        let min_gradient = GradientNew::parse(r, ver)?;
        r.align(4)?;
        let min_color = ColorRgba::parse(r, ver)?;
        let max_color = ColorRgba::parse(r, ver)?;
        let min_max_state = r.read_i16()?;
        r.align(4)?;

        Ok(MinMaxGradient {
            max_gradient,
            min_gradient,
            min_color,
            max_color,
            min_max_state,
        })
    }
}
