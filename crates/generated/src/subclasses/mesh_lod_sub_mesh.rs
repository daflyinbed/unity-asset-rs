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
        let m_levels = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MeshLodRange::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(MeshLodSubMesh {
            m_levels,
        })
    }
}
