//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HeightMeshBvNode {
    pub min: Vector3f,
    pub max: Vector3f,
    pub i: i32,
    pub n: i32,
}

impl crate::Parse for HeightMeshBvNode {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let min = Vector3f::parse(rdr, ver)?;
        let max = Vector3f::parse(rdr, ver)?;
        let i = rdr.read_i32()?;
        let n = rdr.read_i32()?;

        Ok(HeightMeshBvNode { min, max, i, n })
    }
}

impl crate::ToValue for HeightMeshBvNode {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("min".to_string(), self.min.to_value())),
            Some(("max".to_string(), self.max.to_value())),
            Some(("i".to_string(), self.i.to_value())),
            Some(("n".to_string(), self.n.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "min" => (self.min).inspect_value(path, offset, limit),
                "max" => (self.max).inspect_value(path, offset, limit),
                "i" => (self.i).inspect_value(path, offset, limit),
                "n" => (self.n).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.min).inspect_value(path, offset, limit)).map(|v| ("min".to_string(), v)),
            ((self.max).inspect_value(path, offset, limit)).map(|v| ("max".to_string(), v)),
            ((self.i).inspect_value(path, offset, limit)).map(|v| ("i".to_string(), v)),
            ((self.n).inspect_value(path, offset, limit)).map(|v| ("n".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
