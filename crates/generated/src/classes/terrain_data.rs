//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::detail_database::DetailDatabase;
use crate::subclasses::heightmap::Heightmap;
use crate::subclasses::splat_database::SplatDatabase;


/// 版本枚举：TerrainDataMSplatDatabaseVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TerrainDataMSplatDatabaseVariant {
    Default(Box<SplatDatabase>),
    V5_0_1(Box<SplatDatabase>),
}

impl crate::Parse for TerrainDataMSplatDatabaseVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 1) {
            let v = ( { Ok(Box::new(SplatDatabase::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V5_0_1(v))
        } else {
            let v = ( { Ok(Box::new(SplatDatabase::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TerrainData {
    pub m_name: String,
    pub m_splat_database: TerrainDataMSplatDatabaseVariant,
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
            rdr.align(4)?;
            m_splat_database = Some(TerrainDataMSplatDatabaseVariant::parse(rdr, ver)?);
            m_detail_database = Some(DetailDatabase::parse(rdr, ver)?);
            rdr.align(4)?;
            m_heightmap = Some(Heightmap::parse(rdr, ver)?);
            rdr.align(4)?;
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
