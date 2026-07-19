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
