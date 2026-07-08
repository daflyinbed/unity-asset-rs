//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::detail_database::DetailDatabase;
use crate::subclasses::heightmap::Heightmap;
use crate::subclasses::splat_database::SplatDatabase;

#[derive(Debug, Clone)]
pub struct TerrainData {
    pub m_name: String,
    pub m_splat_database: SplatDatabase,
    pub m_detail_database: DetailDatabase,
    pub m_heightmap: Heightmap,
    pub m_preload_shaders: Option<Vec<PPtr>>,
}

impl crate::Parse for TerrainData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_splat_database = None;
        let mut m_detail_database = None;
        let mut m_heightmap = None;
        let mut m_preload_shaders = None;
            m_name = Some(rdr.read_aligned_string()?);
            m_splat_database = Some(SplatDatabase::parse(rdr, ver)?);
            m_detail_database = Some(DetailDatabase::parse(rdr, ver)?);
            m_heightmap = Some(Heightmap::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 2, 17) {
            m_preload_shaders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2018, 4, 14) {
            m_preload_shaders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }

        Ok(TerrainData {
            m_name: m_name.unwrap(),
            m_splat_database: m_splat_database.unwrap(),
            m_detail_database: m_detail_database.unwrap(),
            m_heightmap: m_heightmap.unwrap(),
            m_preload_shaders,
        })
    }
}
