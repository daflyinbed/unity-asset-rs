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
