//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct RaycastCollider {
    pub m_game_object: PPtr,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_enabled: bool,
    pub m_center: Vector3f,
    pub m_length: f32,
}

impl crate::Parse for RaycastCollider {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_material = crate::PPtr::parse(rdr, ver)?;
        let m_is_trigger = rdr.read_bool()?;
        let m_enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_center = Vector3f::parse(rdr, ver)?;
        let m_length = rdr.read_f32()?;

        Ok(RaycastCollider {
            m_game_object,
            m_material,
            m_is_trigger,
            m_enabled,
            m_center,
            m_length,
        })
    }
}

impl crate::ToValue for RaycastCollider {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Material".to_string(), self.m_material.to_value())),
            Some(("m_IsTrigger".to_string(), self.m_is_trigger.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Center".to_string(), self.m_center.to_value())),
            Some(("m_Length".to_string(), self.m_length.to_value())),
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
                "m_Material" => (self.m_material).inspect_value(path, offset, limit),
                "m_IsTrigger" => (self.m_is_trigger).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Center" => (self.m_center).inspect_value(path, offset, limit),
                "m_Length" => (self.m_length).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_material).inspect_value(path, offset, limit))
                .map(|v| ("m_Material".to_string(), v)),
            ((self.m_is_trigger).inspect_value(path, offset, limit))
                .map(|v| ("m_IsTrigger".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_center).inspect_value(path, offset, limit))
                .map(|v| ("m_Center".to_string(), v)),
            ((self.m_length).inspect_value(path, offset, limit))
                .map(|v| ("m_Length".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
