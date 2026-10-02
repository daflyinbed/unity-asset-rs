//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TreePrototype {
    pub prefab: PPtr,
    pub bend_factor: f32,
    pub nav_mesh_lod: Option<i32>,
}

impl crate::Parse for TreePrototype {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let prefab = crate::PPtr::parse(rdr, ver)?;
        let bend_factor = rdr.read_f32()?;
        let nav_mesh_lod = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(TreePrototype {
            prefab,
            bend_factor,
            nav_mesh_lod,
        })
    }
}

impl crate::ToValue for TreePrototype {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("prefab".to_string(), self.prefab.to_value())),
            Some(("bendFactor".to_string(), self.bend_factor.to_value())),
            crate::opt_field("navMeshLod", &self.nav_mesh_lod),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "prefab" => (self.prefab).inspect_value(path, offset, limit),
                "bendFactor" => (self.bend_factor).inspect_value(path, offset, limit),
                "navMeshLod" => (self.nav_mesh_lod).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.prefab).inspect_value(path, offset, limit)).map(|v| ("prefab".to_string(), v)),
            ((self.bend_factor).inspect_value(path, offset, limit))
                .map(|v| ("bendFactor".to_string(), v)),
            ((self.nav_mesh_lod).inspect_value(path, offset, limit))
                .map(|v| ("navMeshLod".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
