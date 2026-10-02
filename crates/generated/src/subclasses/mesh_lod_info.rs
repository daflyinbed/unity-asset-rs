//! 自动生成。请勿手动修改。

use crate::subclasses::lod_selection_curve::LodSelectionCurve;
use crate::subclasses::mesh_lod_sub_mesh::MeshLodSubMesh;

#[derive(Debug, Clone)]
pub struct MeshLodInfo {
    pub m_lod_selection_curve: LodSelectionCurve,
    pub m_num_levels: i32,
    pub m_sub_meshes: Vec<MeshLodSubMesh>,
}

impl crate::Parse for MeshLodInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lod_selection_curve = LodSelectionCurve::parse(rdr, ver)?;
        let m_num_levels = rdr.read_i32()?;
        rdr.align(4)?;
        let m_sub_meshes = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| MeshLodSubMesh::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(MeshLodInfo {
            m_lod_selection_curve,
            m_num_levels,
            m_sub_meshes,
        })
    }
}

impl crate::ToValue for MeshLodInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_LodSelectionCurve".to_string(),
                self.m_lod_selection_curve.to_value(),
            )),
            Some(("m_NumLevels".to_string(), self.m_num_levels.to_value())),
            Some(("m_SubMeshes".to_string(), self.m_sub_meshes.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_LodSelectionCurve" => {
                    (self.m_lod_selection_curve).inspect_value(path, offset, limit)
                }
                "m_NumLevels" => (self.m_num_levels).inspect_value(path, offset, limit),
                "m_SubMeshes" => (self.m_sub_meshes).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_lod_selection_curve).inspect_value(path, offset, limit))
                .map(|v| ("m_LodSelectionCurve".to_string(), v)),
            ((self.m_num_levels).inspect_value(path, offset, limit))
                .map(|v| ("m_NumLevels".to_string(), v)),
            ((self.m_sub_meshes).inspect_value(path, offset, limit))
                .map(|v| ("m_SubMeshes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
