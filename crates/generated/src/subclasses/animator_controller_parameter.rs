//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct AnimatorControllerParameter {
    pub m_name: String,
    pub m_type: i32,
    pub m_min: f32,
    pub m_max: f32,
    pub m_default_vector: Vector3f,
    pub m_default_float: f32,
    pub m_default_int: i32,
    pub m_default_bool: bool,
}

impl crate::Parse for AnimatorControllerParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_type = rdr.read_i32()?;
        let m_min = rdr.read_f32()?;
        let m_max = rdr.read_f32()?;
        let m_default_vector = Vector3f::parse(rdr, ver)?;
        let m_default_float = rdr.read_f32()?;
        let m_default_int = rdr.read_i32()?;
        let m_default_bool = rdr.read_bool()?;

        Ok(AnimatorControllerParameter {
            m_name,
            m_type,
            m_min,
            m_max,
            m_default_vector,
            m_default_float,
            m_default_int,
            m_default_bool,
        })
    }
}

impl crate::ToValue for AnimatorControllerParameter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
            Some(("m_Min".to_string(), self.m_min.to_value())),
            Some(("m_Max".to_string(), self.m_max.to_value())),
            Some((
                "m_DefaultVector".to_string(),
                self.m_default_vector.to_value(),
            )),
            Some((
                "m_DefaultFloat".to_string(),
                self.m_default_float.to_value(),
            )),
            Some(("m_DefaultInt".to_string(), self.m_default_int.to_value())),
            Some(("m_DefaultBool".to_string(), self.m_default_bool.to_value())),
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
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_Min" => (self.m_min).inspect_value(path, offset, limit),
                "m_Max" => (self.m_max).inspect_value(path, offset, limit),
                "m_DefaultVector" => (self.m_default_vector).inspect_value(path, offset, limit),
                "m_DefaultFloat" => (self.m_default_float).inspect_value(path, offset, limit),
                "m_DefaultInt" => (self.m_default_int).inspect_value(path, offset, limit),
                "m_DefaultBool" => (self.m_default_bool).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_min).inspect_value(path, offset, limit)).map(|v| ("m_Min".to_string(), v)),
            ((self.m_max).inspect_value(path, offset, limit)).map(|v| ("m_Max".to_string(), v)),
            ((self.m_default_vector).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultVector".to_string(), v)),
            ((self.m_default_float).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultFloat".to_string(), v)),
            ((self.m_default_int).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultInt".to_string(), v)),
            ((self.m_default_bool).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultBool".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
