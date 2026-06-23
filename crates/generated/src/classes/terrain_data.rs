//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::detail_database::DetailDatabase;
use crate::subclasses::heightmap::Heightmap;
use crate::subclasses::splat_database::SplatDatabase;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct TerrainData {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_splat_database: SplatDatabase,
    pub m_detail_database: DetailDatabase,
    pub m_heightmap: Heightmap,
    pub m_preload_shaders: Option<Vec<PPtr>>,
}

impl crate::Parse for TerrainData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_splat_database = SplatDatabase::parse(r, ver)?;
        let m_detail_database = DetailDatabase::parse(r, ver)?;
        r.align(4)?;
        let m_heightmap = Heightmap::parse(r, ver)?;
        r.align(4)?;
        let m_preload_shaders = if *ver >= crate::UnityVersion::new(2018, 4, 14) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(TerrainData {
            parent,
            m_name,
            m_splat_database,
            m_detail_database,
            m_heightmap,
            m_preload_shaders,
        })
    }
}
