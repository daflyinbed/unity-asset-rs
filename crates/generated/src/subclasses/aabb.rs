//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Aabb {
    pub m_center: Vector3f,
    pub m_extent: Vector3f,
}

impl crate::Parse for Aabb {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_center = Vector3f::parse(rdr, ver)?;
        let m_extent = Vector3f::parse(rdr, ver)?;

        Ok(Aabb { m_center, m_extent })
    }
}

impl crate::ToValue for Aabb {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Center".to_string(), self.m_center.to_value())),
            Some(("m_Extent".to_string(), self.m_extent.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Center" => (self.m_center).inspect_value(path, offset, limit),
                "m_Extent" => (self.m_extent).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_center).inspect_value(path, offset, limit))
                .map(|v| ("m_Center".to_string(), v)),
            ((self.m_extent).inspect_value(path, offset, limit))
                .map(|v| ("m_Extent".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
