//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::vector2f::Vector2f;
use super::gui_element::GuiElement;

#[derive(Debug, Clone)]
pub struct GuiText {
    pub parent: GuiElement,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_text: String,
    pub m_anchor: i16,
    pub m_alignment: i16,
    pub m_pixel_offset: Vector2f,
    pub m_line_spacing: f32,
    pub m_tab_size: f32,
    pub m_font: PPtr,
    pub m_material: PPtr,
    pub m_font_size: i32,
    pub m_font_style: i32,
    pub m_pixel_correct: bool,
    pub m_color: Option<ColorRgba>,
    pub m_rich_text: Option<bool>,
}

impl crate::Parse for GuiText {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GuiElement::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_text = r.read_aligned_string()?;
        r.align(4)?;
        let m_anchor = r.read_i16()?;
        let m_alignment = r.read_i16()?;
        let m_pixel_offset = Vector2f::parse(r, ver)?;
        let m_line_spacing = r.read_f32()?;
        let m_tab_size = r.read_f32()?;
        let m_font = crate::PPtr::parse(r, ver)?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_font_size = r.read_i32()?;
        let m_font_style = r.read_i32()?;
        let m_pixel_correct = r.read_bool()?;
        let m_color = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_rich_text = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(GuiText {
            parent,
            m_game_object,
            m_enabled,
            m_text,
            m_anchor,
            m_alignment,
            m_pixel_offset,
            m_line_spacing,
            m_tab_size,
            m_font,
            m_material,
            m_font_size,
            m_font_style,
            m_pixel_correct,
            m_color,
            m_rich_text,
        })
    }
}
