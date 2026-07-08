//! 自动生成。请勿手动修改。

use crate::subclasses::heightmap_data::HeightmapData;

#[derive(Debug, Clone)]
pub struct NavMesh {
    pub m_name: String,
    pub m_mesh_data: Option<Vec<u8>>,
    pub m_heightmaps: Option<Vec<HeightmapData>>,
}

impl crate::Parse for NavMesh {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_mesh_data = None;
        let mut m_heightmaps = None;
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_mesh_data = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_heightmaps = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }

        Ok(NavMesh {
            m_name: m_name.unwrap(),
            m_mesh_data,
            m_heightmaps,
        })
    }
}
