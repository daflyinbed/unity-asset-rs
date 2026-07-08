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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_text = None;
        let mut m_offset_z = None;
        let mut m_character_size = None;
        let mut m_line_spacing = None;
        let mut m_anchor = None;
        let mut m_alignment = None;
        let mut m_tab_size = None;
        let mut m_font_size = None;
        let mut m_font_style = None;
        let mut m_font = None;
        let mut m_rich_text = None;
        let mut m_color = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_text = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_offset_z = Some(rdr.read_f32()?);
            m_character_size = Some(rdr.read_f32()?);
            m_line_spacing = Some(rdr.read_f32()?);
            m_anchor = Some(rdr.read_i16()?);
            m_alignment = Some(rdr.read_i16()?);
            m_tab_size = Some(rdr.read_f32()?);
            m_font_size = Some(rdr.read_i32()?);
            m_font_style = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_rich_text = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_font = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_color = Some(ColorRgba::parse(rdr, ver)?);
        } else {
        }

        Ok(TextMesh {
            m_game_object: m_game_object.unwrap(),
            m_text: m_text.unwrap(),
            m_offset_z: m_offset_z.unwrap(),
            m_character_size: m_character_size.unwrap(),
            m_line_spacing: m_line_spacing.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_alignment: m_alignment.unwrap(),
            m_tab_size: m_tab_size.unwrap(),
            m_font_size: m_font_size.unwrap(),
            m_font_style: m_font_style.unwrap(),
            m_font: m_font.unwrap(),
            m_rich_text,
            m_color,
        })
    }
}
