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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_mesh_data = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_heightmaps = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { HeightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };

        Ok(NavMesh {
            m_name,
            m_mesh_data,
            m_heightmaps,
        })
    }
}
