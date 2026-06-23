//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct LineParameters {
    pub start_width: f32,
    pub end_width: f32,
    pub m_start_color: ColorRgba,
    pub m_end_color: ColorRgba,
}

impl crate::Parse for LineParameters {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let start_width = r.read_f32()?;
        let end_width = r.read_f32()?;
        let m_start_color = ColorRgba::parse(r, ver)?;
        let m_end_color = ColorRgba::parse(r, ver)?;

        Ok(LineParameters {
            start_width,
            end_width,
            m_start_color,
            m_end_color,
        })
    }
}
