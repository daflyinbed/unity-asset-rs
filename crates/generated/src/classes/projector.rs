//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct Projector {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_near_clip_plane: f32,
    pub m_far_clip_plane: f32,
    pub m_field_of_view: f32,
    pub m_aspect_ratio: f32,
    pub m_orthographic: bool,
    pub m_orthographic_size: f32,
    pub m_material: PPtr,
    pub m_ignore_layers: BitField,
}

impl crate::Parse for Projector {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_near_clip_plane = rdr.read_f32()?;
        let m_far_clip_plane = rdr.read_f32()?;
        let m_field_of_view = rdr.read_f32()?;
        let m_aspect_ratio = rdr.read_f32()?;
        let m_orthographic = rdr.read_bool()?;
        rdr.align(4)?;
        let m_orthographic_size = rdr.read_f32()?;
        let m_material = crate::PPtr::parse(rdr, ver)?;
        let m_ignore_layers = BitField::parse(rdr, ver)?;

        Ok(Projector {
            m_game_object,
            m_enabled,
            m_near_clip_plane,
            m_far_clip_plane,
            m_field_of_view,
            m_aspect_ratio,
            m_orthographic,
            m_orthographic_size,
            m_material,
            m_ignore_layers,
        })
    }
}

impl crate::ToValue for Projector {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_NearClipPlane".to_string(),
                self.m_near_clip_plane.to_value(),
            )),
            Some((
                "m_FarClipPlane".to_string(),
                self.m_far_clip_plane.to_value(),
            )),
            Some(("m_FieldOfView".to_string(), self.m_field_of_view.to_value())),
            Some(("m_AspectRatio".to_string(), self.m_aspect_ratio.to_value())),
            Some(("m_Orthographic".to_string(), self.m_orthographic.to_value())),
            Some((
                "m_OrthographicSize".to_string(),
                self.m_orthographic_size.to_value(),
            )),
            Some(("m_Material".to_string(), self.m_material.to_value())),
            Some((
                "m_IgnoreLayers".to_string(),
                self.m_ignore_layers.to_value(),
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
                "m_NearClipPlane" => (self.m_near_clip_plane).inspect_value(path, offset, limit),
                "m_FarClipPlane" => (self.m_far_clip_plane).inspect_value(path, offset, limit),
                "m_FieldOfView" => (self.m_field_of_view).inspect_value(path, offset, limit),
                "m_AspectRatio" => (self.m_aspect_ratio).inspect_value(path, offset, limit),
                "m_Orthographic" => (self.m_orthographic).inspect_value(path, offset, limit),
                "m_OrthographicSize" => {
                    (self.m_orthographic_size).inspect_value(path, offset, limit)
                }
                "m_Material" => (self.m_material).inspect_value(path, offset, limit),
                "m_IgnoreLayers" => (self.m_ignore_layers).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_near_clip_plane).inspect_value(path, offset, limit))
                .map(|v| ("m_NearClipPlane".to_string(), v)),
            ((self.m_far_clip_plane).inspect_value(path, offset, limit))
                .map(|v| ("m_FarClipPlane".to_string(), v)),
            ((self.m_field_of_view).inspect_value(path, offset, limit))
                .map(|v| ("m_FieldOfView".to_string(), v)),
            ((self.m_aspect_ratio).inspect_value(path, offset, limit))
                .map(|v| ("m_AspectRatio".to_string(), v)),
            ((self.m_orthographic).inspect_value(path, offset, limit))
                .map(|v| ("m_Orthographic".to_string(), v)),
            ((self.m_orthographic_size).inspect_value(path, offset, limit))
                .map(|v| ("m_OrthographicSize".to_string(), v)),
            ((self.m_material).inspect_value(path, offset, limit))
                .map(|v| ("m_Material".to_string(), v)),
            ((self.m_ignore_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_IgnoreLayers".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
