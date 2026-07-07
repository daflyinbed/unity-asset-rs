//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct TextMesh {
    pub m_game_object: PPtr,
    pub m_text: String,
    pub m_offset_z: f32,
    pub m_character_size: f32,
    pub m_line_spacing: f32,
    pub m_anchor: i16,
    pub m_alignment: i16,
    pub m_tab_size: f32,
    pub m_font_size: i32,
    pub m_font_style: i32,
    pub m_font: PPtr,
    pub m_rich_text: Option<bool>,
    pub m_color: Option<ColorRgba>,
}

impl crate::Parse for TextMesh {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_text = r.read_aligned_string()?;
        r.align(4)?;
        let m_offset_z = r.read_f32()?;
        let m_character_size = r.read_f32()?;
        let m_line_spacing = r.read_f32()?;
        let m_anchor = r.read_i16()?;
        let m_alignment = r.read_i16()?;
        let m_tab_size = r.read_f32()?;
        let m_font_size = r.read_i32()?;
        let m_font_style = r.read_i32()?;
        let m_font = crate::PPtr::parse(r, ver)?;
        let m_rich_text = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_color = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };

        Ok(TextMesh {
            m_game_object,
            m_text,
            m_offset_z,
            m_character_size,
            m_line_spacing,
            m_anchor,
            m_alignment,
            m_tab_size,
            m_font_size,
            m_font_style,
            m_font,
            m_rich_text,
            m_color,
        })
    }
}
