//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct Gradient {
    pub m_color_0: ColorRgba,
    pub m_color_1: ColorRgba,
    pub m_color_2: ColorRgba,
    pub m_color_3: ColorRgba,
    pub m_color_4: ColorRgba,
}

impl crate::Parse for Gradient {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_color_0 = ColorRgba::parse(r, ver)?;
        let m_color_1 = ColorRgba::parse(r, ver)?;
        let m_color_2 = ColorRgba::parse(r, ver)?;
        let m_color_3 = ColorRgba::parse(r, ver)?;
        let m_color_4 = ColorRgba::parse(r, ver)?;

        Ok(Gradient {
            m_color_0,
            m_color_1,
            m_color_2,
            m_color_3,
            m_color_4,
        })
    }
}
