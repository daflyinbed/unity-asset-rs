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
            m_preload_shaders = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2018, 4, 14) {
            m_preload_shaders = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for TerrainData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_SplatDatabase".to_string(),
                self.m_splat_database.to_value(),
            )),
            Some((
                "m_DetailDatabase".to_string(),
                self.m_detail_database.to_value(),
            )),
            Some(("m_Heightmap".to_string(), self.m_heightmap.to_value())),
            crate::opt_field("m_PreloadShaders", &self.m_preload_shaders),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_SplatDatabase" => (self.m_splat_database).inspect_value(path, offset, limit),
                "m_DetailDatabase" => (self.m_detail_database).inspect_value(path, offset, limit),
                "m_Heightmap" => (self.m_heightmap).inspect_value(path, offset, limit),
                "m_PreloadShaders" => (self.m_preload_shaders).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_splat_database).inspect_value(path, offset, limit))
                .map(|v| ("m_SplatDatabase".to_string(), v)),
            ((self.m_detail_database).inspect_value(path, offset, limit))
                .map(|v| ("m_DetailDatabase".to_string(), v)),
            ((self.m_heightmap).inspect_value(path, offset, limit))
                .map(|v| ("m_Heightmap".to_string(), v)),
            ((self.m_preload_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_PreloadShaders".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
