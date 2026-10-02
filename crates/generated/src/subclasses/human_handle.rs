//! 自动生成。请勿手动修改。

use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HumanHandle {
    pub m_name: String,
    pub m_bone_name: String,
    pub m_position: Vector3f,
    pub m_rotation: Quaternionf,
    pub m_scale: Vector3f,
    pub m_look_at: bool,
}

impl crate::Parse for HumanHandle {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_bone_name = rdr.read_aligned_string()?;
        let m_position = Vector3f::parse(rdr, ver)?;
        let m_rotation = Quaternionf::parse(rdr, ver)?;
        let m_scale = Vector3f::parse(rdr, ver)?;
        let m_look_at = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(HumanHandle {
            m_name,
            m_bone_name,
            m_position,
            m_rotation,
            m_scale,
            m_look_at,
        })
    }
}

impl crate::ToValue for HumanHandle {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_BoneName".to_string(), self.m_bone_name.to_value())),
            Some(("m_Position".to_string(), self.m_position.to_value())),
            Some(("m_Rotation".to_string(), self.m_rotation.to_value())),
            Some(("m_Scale".to_string(), self.m_scale.to_value())),
            Some(("m_LookAt".to_string(), self.m_look_at.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_BoneName" => (self.m_bone_name).inspect_value(path, offset, limit),
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                "m_Rotation" => (self.m_rotation).inspect_value(path, offset, limit),
                "m_Scale" => (self.m_scale).inspect_value(path, offset, limit),
                "m_LookAt" => (self.m_look_at).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_bone_name).inspect_value(path, offset, limit))
                .map(|v| ("m_BoneName".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
            ((self.m_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_Rotation".to_string(), v)),
            ((self.m_scale).inspect_value(path, offset, limit)).map(|v| ("m_Scale".to_string(), v)),
            ((self.m_look_at).inspect_value(path, offset, limit))
                .map(|v| ("m_LookAt".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
