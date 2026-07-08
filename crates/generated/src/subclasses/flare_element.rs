//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct FlareElement {
    pub m_image_index: u32,
    pub m_position: f32,
    pub m_size: f32,
    pub m_color: ColorRgba,
    pub m_use_light_color: bool,
    pub m_rotate: bool,
    pub m_zoom: bool,
    pub m_fade: bool,
}

impl crate::Parse for FlareElement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_image_index = rdr.read_u32()?;
        let m_position = rdr.read_f32()?;
        let m_size = rdr.read_f32()?;
        let m_color = ColorRgba::parse(rdr, ver)?;
        let m_use_light_color = rdr.read_bool()?;
        let m_rotate = rdr.read_bool()?;
        let m_zoom = rdr.read_bool()?;
        let m_fade = rdr.read_bool()?;

        Ok(FlareElement {
            m_image_index,
            m_position,
            m_size,
            m_color,
            m_use_light_color,
            m_rotate,
            m_zoom,
            m_fade,
        })
    }
}
