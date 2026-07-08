//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct GuiText {
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
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_text = None;
        let mut m_anchor = None;
        let mut m_alignment = None;
        let mut m_pixel_offset = None;
        let mut m_line_spacing = None;
        let mut m_tab_size = None;
        let mut m_font = None;
        let mut m_material = None;
        let mut m_font_size = None;
        let mut m_font_style = None;
        let mut m_pixel_correct = None;
        let mut m_color = None;
        let mut m_rich_text = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
            m_text = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_anchor = Some(r.read_i16()?);
            m_alignment = Some(r.read_i16()?);
            m_pixel_offset = Some(Vector2f::parse(r, ver)?);
            m_line_spacing = Some(r.read_f32()?);
            m_tab_size = Some(r.read_f32()?);
            m_font = Some(crate::PPtr::parse(r, ver)?);
            m_material = Some(crate::PPtr::parse(r, ver)?);
            m_font_size = Some(r.read_i32()?);
            m_font_style = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_color = Some(ColorRgba::parse(r, ver)?);
        } else {
        }
            m_pixel_correct = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_rich_text = Some(r.read_bool()?);
        } else {
        }

        Ok(GuiText {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_text: m_text.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_alignment: m_alignment.unwrap(),
            m_pixel_offset: m_pixel_offset.unwrap(),
            m_line_spacing: m_line_spacing.unwrap(),
            m_tab_size: m_tab_size.unwrap(),
            m_font: m_font.unwrap(),
            m_material: m_material.unwrap(),
            m_font_size: m_font_size.unwrap(),
            m_font_style: m_font_style.unwrap(),
            m_pixel_correct: m_pixel_correct.unwrap(),
            m_color,
            m_rich_text,
        })
    }
}
