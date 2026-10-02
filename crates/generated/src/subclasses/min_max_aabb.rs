//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct MinMaxAabb {
    pub m_min: Vector3f,
    pub m_max: Vector3f,
}

impl crate::Parse for MinMaxAabb {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_min = Vector3f::parse(rdr, ver)?;
        let m_max = Vector3f::parse(rdr, ver)?;

        Ok(MinMaxAabb { m_min, m_max })
    }
}

impl crate::ToValue for MinMaxAabb {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Min".to_string(), self.m_min.to_value())),
            Some(("m_Max".to_string(), self.m_max.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Min" => (self.m_min).inspect_value(path, offset, limit),
                "m_Max" => (self.m_max).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_min).inspect_value(path, offset, limit)).map(|v| ("m_Min".to_string(), v)),
            ((self.m_max).inspect_value(path, offset, limit)).map(|v| ("m_Max".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
