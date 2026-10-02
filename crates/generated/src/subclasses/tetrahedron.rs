//! 自动生成。请勿手动修改。

use crate::subclasses::matrix3x4f::Matrix3x4f;

#[derive(Debug, Clone)]
pub struct Tetrahedron {
    pub indices_0: i32,
    pub indices_1: i32,
    pub indices_2: i32,
    pub indices_3: i32,
    pub neighbors_0: i32,
    pub neighbors_1: i32,
    pub neighbors_2: i32,
    pub neighbors_3: i32,
    pub matrix: Matrix3x4f,
}

impl crate::Parse for Tetrahedron {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let indices_0 = rdr.read_i32()?;
        let indices_1 = rdr.read_i32()?;
        let indices_2 = rdr.read_i32()?;
        let indices_3 = rdr.read_i32()?;
        let neighbors_0 = rdr.read_i32()?;
        let neighbors_1 = rdr.read_i32()?;
        let neighbors_2 = rdr.read_i32()?;
        let neighbors_3 = rdr.read_i32()?;
        let matrix = Matrix3x4f::parse(rdr, ver)?;

        Ok(Tetrahedron {
            indices_0,
            indices_1,
            indices_2,
            indices_3,
            neighbors_0,
            neighbors_1,
            neighbors_2,
            neighbors_3,
            matrix,
        })
    }
}

impl crate::ToValue for Tetrahedron {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("indices[0]".to_string(), self.indices_0.to_value())),
            Some(("indices[1]".to_string(), self.indices_1.to_value())),
            Some(("indices[2]".to_string(), self.indices_2.to_value())),
            Some(("indices[3]".to_string(), self.indices_3.to_value())),
            Some(("neighbors[0]".to_string(), self.neighbors_0.to_value())),
            Some(("neighbors[1]".to_string(), self.neighbors_1.to_value())),
            Some(("neighbors[2]".to_string(), self.neighbors_2.to_value())),
            Some(("neighbors[3]".to_string(), self.neighbors_3.to_value())),
            Some(("matrix".to_string(), self.matrix.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "indices[0]" => (self.indices_0).inspect_value(path, offset, limit),
                "indices[1]" => (self.indices_1).inspect_value(path, offset, limit),
                "indices[2]" => (self.indices_2).inspect_value(path, offset, limit),
                "indices[3]" => (self.indices_3).inspect_value(path, offset, limit),
                "neighbors[0]" => (self.neighbors_0).inspect_value(path, offset, limit),
                "neighbors[1]" => (self.neighbors_1).inspect_value(path, offset, limit),
                "neighbors[2]" => (self.neighbors_2).inspect_value(path, offset, limit),
                "neighbors[3]" => (self.neighbors_3).inspect_value(path, offset, limit),
                "matrix" => (self.matrix).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.indices_0).inspect_value(path, offset, limit))
                .map(|v| ("indices[0]".to_string(), v)),
            ((self.indices_1).inspect_value(path, offset, limit))
                .map(|v| ("indices[1]".to_string(), v)),
            ((self.indices_2).inspect_value(path, offset, limit))
                .map(|v| ("indices[2]".to_string(), v)),
            ((self.indices_3).inspect_value(path, offset, limit))
                .map(|v| ("indices[3]".to_string(), v)),
            ((self.neighbors_0).inspect_value(path, offset, limit))
                .map(|v| ("neighbors[0]".to_string(), v)),
            ((self.neighbors_1).inspect_value(path, offset, limit))
                .map(|v| ("neighbors[1]".to_string(), v)),
            ((self.neighbors_2).inspect_value(path, offset, limit))
                .map(|v| ("neighbors[2]".to_string(), v)),
            ((self.neighbors_3).inspect_value(path, offset, limit))
                .map(|v| ("neighbors[3]".to_string(), v)),
            ((self.matrix).inspect_value(path, offset, limit)).map(|v| ("matrix".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
