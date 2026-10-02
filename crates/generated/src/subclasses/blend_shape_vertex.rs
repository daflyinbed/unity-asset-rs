//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct BlendShapeVertex {
    pub vertex: Vector3f,
    pub normal: Vector3f,
    pub tangent: Vector3f,
    pub index: u32,
}

impl crate::Parse for BlendShapeVertex {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vertex = Vector3f::parse(rdr, ver)?;
        let normal = Vector3f::parse(rdr, ver)?;
        let tangent = Vector3f::parse(rdr, ver)?;
        let index = rdr.read_u32()?;

        Ok(BlendShapeVertex {
            vertex,
            normal,
            tangent,
            index,
        })
    }
}

impl crate::ToValue for BlendShapeVertex {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("vertex".to_string(), self.vertex.to_value())),
            Some(("normal".to_string(), self.normal.to_value())),
            Some(("tangent".to_string(), self.tangent.to_value())),
            Some(("index".to_string(), self.index.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "vertex" => (self.vertex).inspect_value(path, offset, limit),
                "normal" => (self.normal).inspect_value(path, offset, limit),
                "tangent" => (self.tangent).inspect_value(path, offset, limit),
                "index" => (self.index).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.vertex).inspect_value(path, offset, limit)).map(|v| ("vertex".to_string(), v)),
            ((self.normal).inspect_value(path, offset, limit)).map(|v| ("normal".to_string(), v)),
            ((self.tangent).inspect_value(path, offset, limit)).map(|v| ("tangent".to_string(), v)),
            ((self.index).inspect_value(path, offset, limit)).map(|v| ("index".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
