//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgb_af::ColorRgbAf;

#[derive(Debug, Clone)]
pub struct LensFlare {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_flare: PPtr,
    pub m_color: ColorRgbAf,
    pub m_brightness: f32,
    pub m_ignore_layers: BitField,
    pub m_fade_speed: Option<f32>,
    pub m_directional: bool,
}

impl crate::Parse for LensFlare {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_flare = None;
        let mut m_color = None;
        let mut m_brightness = None;
        let mut m_ignore_layers = None;
        let mut m_fade_speed = None;
        let mut m_directional = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_flare = Some(crate::PPtr::parse(rdr, ver)?);
        m_color = Some(ColorRgbAf::parse(rdr, ver)?);
        m_brightness = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_fade_speed = Some(rdr.read_f32()?);
        } else {
        }
        m_ignore_layers = Some(BitField::parse(rdr, ver)?);
        m_directional = Some(rdr.read_bool()?);

        Ok(LensFlare {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_flare: m_flare.unwrap(),
            m_color: m_color.unwrap(),
            m_brightness: m_brightness.unwrap(),
            m_ignore_layers: m_ignore_layers.unwrap(),
            m_fade_speed,
            m_directional: m_directional.unwrap(),
        })
    }
}

impl crate::ToValue for LensFlare {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Flare".to_string(), self.m_flare.to_value())),
            Some(("m_Color".to_string(), self.m_color.to_value())),
            Some(("m_Brightness".to_string(), self.m_brightness.to_value())),
            Some((
                "m_IgnoreLayers".to_string(),
                self.m_ignore_layers.to_value(),
            )),
            crate::opt_field("m_FadeSpeed", &self.m_fade_speed),
            Some(("m_Directional".to_string(), self.m_directional.to_value())),
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
                "m_Flare" => (self.m_flare).inspect_value(path, offset, limit),
                "m_Color" => (self.m_color).inspect_value(path, offset, limit),
                "m_Brightness" => (self.m_brightness).inspect_value(path, offset, limit),
                "m_IgnoreLayers" => (self.m_ignore_layers).inspect_value(path, offset, limit),
                "m_FadeSpeed" => (self.m_fade_speed).inspect_value(path, offset, limit),
                "m_Directional" => (self.m_directional).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_flare).inspect_value(path, offset, limit)).map(|v| ("m_Flare".to_string(), v)),
            ((self.m_color).inspect_value(path, offset, limit)).map(|v| ("m_Color".to_string(), v)),
            ((self.m_brightness).inspect_value(path, offset, limit))
                .map(|v| ("m_Brightness".to_string(), v)),
            ((self.m_ignore_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_IgnoreLayers".to_string(), v)),
            ((self.m_fade_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_FadeSpeed".to_string(), v)),
            ((self.m_directional).inspect_value(path, offset, limit))
                .map(|v| ("m_Directional".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
