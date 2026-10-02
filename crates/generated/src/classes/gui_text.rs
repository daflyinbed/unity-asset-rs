//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba32::ColorRgba32;
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
    pub m_color: Option<ColorRgba32>,
    pub m_rich_text: Option<bool>,
}

impl crate::Parse for GuiText {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_text = Some(rdr.read_aligned_string()?);
        m_anchor = Some(rdr.read_i16()?);
        m_alignment = Some(rdr.read_i16()?);
        m_pixel_offset = Some(Vector2f::parse(rdr, ver)?);
        m_line_spacing = Some(rdr.read_f32()?);
        m_tab_size = Some(rdr.read_f32()?);
        m_font = Some(crate::PPtr::parse(rdr, ver)?);
        m_material = Some(crate::PPtr::parse(rdr, ver)?);
        m_font_size = Some(rdr.read_i32()?);
        m_font_style = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_color = Some(ColorRgba32::parse(rdr, ver)?);
        } else {
        }
        m_pixel_correct = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_rich_text = Some(rdr.read_bool()?);
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

impl crate::ToValue for GuiText {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Text".to_string(), self.m_text.to_value())),
            Some(("m_Anchor".to_string(), self.m_anchor.to_value())),
            Some(("m_Alignment".to_string(), self.m_alignment.to_value())),
            Some(("m_PixelOffset".to_string(), self.m_pixel_offset.to_value())),
            Some(("m_LineSpacing".to_string(), self.m_line_spacing.to_value())),
            Some(("m_TabSize".to_string(), self.m_tab_size.to_value())),
            Some(("m_Font".to_string(), self.m_font.to_value())),
            Some(("m_Material".to_string(), self.m_material.to_value())),
            Some(("m_FontSize".to_string(), self.m_font_size.to_value())),
            Some(("m_FontStyle".to_string(), self.m_font_style.to_value())),
            Some((
                "m_PixelCorrect".to_string(),
                self.m_pixel_correct.to_value(),
            )),
            crate::opt_field("m_Color", &self.m_color),
            crate::opt_field("m_RichText", &self.m_rich_text),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Text" => (self.m_text).inspect_value(path, offset, limit),
                "m_Anchor" => (self.m_anchor).inspect_value(path, offset, limit),
                "m_Alignment" => (self.m_alignment).inspect_value(path, offset, limit),
                "m_PixelOffset" => (self.m_pixel_offset).inspect_value(path, offset, limit),
                "m_LineSpacing" => (self.m_line_spacing).inspect_value(path, offset, limit),
                "m_TabSize" => (self.m_tab_size).inspect_value(path, offset, limit),
                "m_Font" => (self.m_font).inspect_value(path, offset, limit),
                "m_Material" => (self.m_material).inspect_value(path, offset, limit),
                "m_FontSize" => (self.m_font_size).inspect_value(path, offset, limit),
                "m_FontStyle" => (self.m_font_style).inspect_value(path, offset, limit),
                "m_PixelCorrect" => (self.m_pixel_correct).inspect_value(path, offset, limit),
                "m_Color" => (self.m_color).inspect_value(path, offset, limit),
                "m_RichText" => (self.m_rich_text).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_text).inspect_value(path, offset, limit)).map(|v| ("m_Text".to_string(), v)),
            ((self.m_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_Anchor".to_string(), v)),
            ((self.m_alignment).inspect_value(path, offset, limit))
                .map(|v| ("m_Alignment".to_string(), v)),
            ((self.m_pixel_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelOffset".to_string(), v)),
            ((self.m_line_spacing).inspect_value(path, offset, limit))
                .map(|v| ("m_LineSpacing".to_string(), v)),
            ((self.m_tab_size).inspect_value(path, offset, limit))
                .map(|v| ("m_TabSize".to_string(), v)),
            ((self.m_font).inspect_value(path, offset, limit)).map(|v| ("m_Font".to_string(), v)),
            ((self.m_material).inspect_value(path, offset, limit))
                .map(|v| ("m_Material".to_string(), v)),
            ((self.m_font_size).inspect_value(path, offset, limit))
                .map(|v| ("m_FontSize".to_string(), v)),
            ((self.m_font_style).inspect_value(path, offset, limit))
                .map(|v| ("m_FontStyle".to_string(), v)),
            ((self.m_pixel_correct).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelCorrect".to_string(), v)),
            ((self.m_color).inspect_value(path, offset, limit)).map(|v| ("m_Color".to_string(), v)),
            ((self.m_rich_text).inspect_value(path, offset, limit))
                .map(|v| ("m_RichText".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
