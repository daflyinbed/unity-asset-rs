//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::rectf::Rectf;

#[derive(Debug, Clone)]
pub struct GuiTexture {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_texture = crate::PPtr::parse(r, ver)?;
        let m_color = ColorRgba::parse(r, ver)?;
        let m_pixel_inset = Rectf::parse(r, ver)?;
        let m_left_border = r.read_i32()?;
        let m_right_border = r.read_i32()?;
        let m_top_border = r.read_i32()?;
        let m_bottom_border = r.read_i32()?;

        Ok(GuiTexture {
            m_object_hide_flags,
            m_extension_ptr,
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
