//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::rectf::Rectf;

#[derive(Debug, Clone)]
pub struct GuiTexture {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_texture: PPtr,
    pub m_color: ColorRgba,
    pub m_pixel_inset: Rectf,
    pub m_left_border: i32,
    pub m_right_border: i32,
    pub m_top_border: i32,
    pub m_bottom_border: i32,
}

impl crate::Parse for GuiTexture {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_texture = crate::PPtr::parse(rdr, ver)?;
        let m_color = ColorRgba::parse(rdr, ver)?;
        let m_pixel_inset = Rectf::parse(rdr, ver)?;
        let m_left_border = rdr.read_i32()?;
        let m_right_border = rdr.read_i32()?;
        let m_top_border = rdr.read_i32()?;
        let m_bottom_border = rdr.read_i32()?;

        Ok(GuiTexture {
            m_game_object,
            m_enabled,
            m_texture,
            m_color,
            m_pixel_inset,
            m_left_border,
            m_right_border,
            m_top_border,
            m_bottom_border,
        })
    }
}
