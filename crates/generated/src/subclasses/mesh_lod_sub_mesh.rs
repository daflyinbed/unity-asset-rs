//! 自动生成。请勿手动修改。

use crate::subclasses::mesh_lod_range::MeshLodRange;

#[derive(Debug, Clone)]
pub struct MeshLodSubMesh {
    pub m_levels: Vec<MeshLodRange>,
}

impl crate::Parse for MeshLodSubMesh {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_levels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MeshLodRange::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(MeshLodSubMesh {
            m_levels,
        })
    }
}
