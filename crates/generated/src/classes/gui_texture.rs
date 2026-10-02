//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::rectf::Rectf;

#[derive(Debug, Clone)]
pub struct GuiTexture {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_texture: PPtr,
    pub m_color: ColorRgbAf,
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
        let m_color = ColorRgbAf::parse(rdr, ver)?;
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

impl crate::ToValue for GuiTexture {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Texture".to_string(), self.m_texture.to_value())),
            Some(("m_Color".to_string(), self.m_color.to_value())),
            Some(("m_PixelInset".to_string(), self.m_pixel_inset.to_value())),
            Some(("m_LeftBorder".to_string(), self.m_left_border.to_value())),
            Some(("m_RightBorder".to_string(), self.m_right_border.to_value())),
            Some(("m_TopBorder".to_string(), self.m_top_border.to_value())),
            Some((
                "m_BottomBorder".to_string(),
                self.m_bottom_border.to_value(),
            )),
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
                "m_Texture" => (self.m_texture).inspect_value(path, offset, limit),
                "m_Color" => (self.m_color).inspect_value(path, offset, limit),
                "m_PixelInset" => (self.m_pixel_inset).inspect_value(path, offset, limit),
                "m_LeftBorder" => (self.m_left_border).inspect_value(path, offset, limit),
                "m_RightBorder" => (self.m_right_border).inspect_value(path, offset, limit),
                "m_TopBorder" => (self.m_top_border).inspect_value(path, offset, limit),
                "m_BottomBorder" => (self.m_bottom_border).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_Texture".to_string(), v)),
            ((self.m_color).inspect_value(path, offset, limit)).map(|v| ("m_Color".to_string(), v)),
            ((self.m_pixel_inset).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelInset".to_string(), v)),
            ((self.m_left_border).inspect_value(path, offset, limit))
                .map(|v| ("m_LeftBorder".to_string(), v)),
            ((self.m_right_border).inspect_value(path, offset, limit))
                .map(|v| ("m_RightBorder".to_string(), v)),
            ((self.m_top_border).inspect_value(path, offset, limit))
                .map(|v| ("m_TopBorder".to_string(), v)),
            ((self.m_bottom_border).inspect_value(path, offset, limit))
                .map(|v| ("m_BottomBorder".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
