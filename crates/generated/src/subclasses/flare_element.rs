//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgb_af::ColorRgbAf;

#[derive(Debug, Clone)]
pub struct FlareElement {
    pub m_image_index: u32,
    pub m_position: f32,
    pub m_size: f32,
    pub m_color: ColorRgbAf,
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
        let m_color = ColorRgbAf::parse(rdr, ver)?;
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

impl crate::ToValue for FlareElement {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_ImageIndex".to_string(), self.m_image_index.to_value())),
            Some(("m_Position".to_string(), self.m_position.to_value())),
            Some(("m_Size".to_string(), self.m_size.to_value())),
            Some(("m_Color".to_string(), self.m_color.to_value())),
            Some((
                "m_UseLightColor".to_string(),
                self.m_use_light_color.to_value(),
            )),
            Some(("m_Rotate".to_string(), self.m_rotate.to_value())),
            Some(("m_Zoom".to_string(), self.m_zoom.to_value())),
            Some(("m_Fade".to_string(), self.m_fade.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ImageIndex" => (self.m_image_index).inspect_value(path, offset, limit),
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                "m_Color" => (self.m_color).inspect_value(path, offset, limit),
                "m_UseLightColor" => (self.m_use_light_color).inspect_value(path, offset, limit),
                "m_Rotate" => (self.m_rotate).inspect_value(path, offset, limit),
                "m_Zoom" => (self.m_zoom).inspect_value(path, offset, limit),
                "m_Fade" => (self.m_fade).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_image_index).inspect_value(path, offset, limit))
                .map(|v| ("m_ImageIndex".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
            ((self.m_color).inspect_value(path, offset, limit)).map(|v| ("m_Color".to_string(), v)),
            ((self.m_use_light_color).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLightColor".to_string(), v)),
            ((self.m_rotate).inspect_value(path, offset, limit))
                .map(|v| ("m_Rotate".to_string(), v)),
            ((self.m_zoom).inspect_value(path, offset, limit)).map(|v| ("m_Zoom".to_string(), v)),
            ((self.m_fade).inspect_value(path, offset, limit)).map(|v| ("m_Fade".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
