//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::detail_database::DetailDatabase;
use crate::subclasses::heightmap::Heightmap;
use crate::subclasses::splat_database::SplatDatabase;

#[derive(Debug, Clone)]
pub struct TerrainData {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_splat_database: SplatDatabase,
    pub m_detail_database: DetailDatabase,
    pub m_heightmap: Heightmap,
}

impl crate::Parse for TerrainData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_splat_database = SplatDatabase::parse(r, ver)?;
        let m_detail_database = DetailDatabase::parse(r, ver)?;
        r.align(4)?;
        let m_heightmap = Heightmap::parse(r, ver)?;
        r.align(4)?;

        Ok(TerrainData {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_splat_database,
            m_detail_database,
            m_heightmap,
        })
    }
}
