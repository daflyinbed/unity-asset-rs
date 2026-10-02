//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct NavMeshTileData {
    pub m_mesh_data: Vec<u8>,
    pub m_hash: Option<Hash128>,
}

impl crate::Parse for NavMeshTileData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_mesh_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_hash = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(Hash128::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(NavMeshTileData {
            m_mesh_data,
            m_hash,
        })
    }
}

impl crate::ToValue for NavMeshTileData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_MeshData".to_string(), self.m_mesh_data.to_value())),
            crate::opt_field("m_Hash", &self.m_hash),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_MeshData" => (self.m_mesh_data).inspect_value(path, offset, limit),
                "m_Hash" => (self.m_hash).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_mesh_data).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshData".to_string(), v)),
            ((self.m_hash).inspect_value(path, offset, limit)).map(|v| ("m_Hash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
