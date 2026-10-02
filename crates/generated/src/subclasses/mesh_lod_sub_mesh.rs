//! 自动生成。请勿手动修改。

use crate::subclasses::mesh_lod_range::MeshLodRange;

#[derive(Debug, Clone)]
pub struct MeshLodSubMesh {
    pub m_levels: Vec<MeshLodRange>,
}

impl crate::Parse for MeshLodSubMesh {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_levels = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| MeshLodRange::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(MeshLodSubMesh { m_levels })
    }
}

impl crate::ToValue for MeshLodSubMesh {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("m_Levels".to_string(), self.m_levels.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Levels" => (self.m_levels).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_levels).inspect_value(path, offset, limit))
                .map(|v| ("m_Levels".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
