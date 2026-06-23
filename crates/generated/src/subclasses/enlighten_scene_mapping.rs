//! 自动生成。请勿手动修改。

use crate::subclasses::enlighten_renderer_information::EnlightenRendererInformation;
use crate::subclasses::enlighten_system_atlas_information::EnlightenSystemAtlasInformation;
use crate::subclasses::enlighten_system_information::EnlightenSystemInformation;
use crate::subclasses::enlighten_terrain_chunks_information::EnlightenTerrainChunksInformation;

#[derive(Debug, Clone)]
pub struct EnlightenSceneMapping {
    pub m_renderers: Vec<EnlightenRendererInformation>,
    pub m_systems: Vec<EnlightenSystemInformation>,
    pub m_system_atlases: Vec<EnlightenSystemAtlasInformation>,
    pub m_terrain_chunks: Vec<EnlightenTerrainChunksInformation>,
}

impl crate::Parse for EnlightenSceneMapping {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_renderers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { EnlightenRendererInformation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_systems = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { EnlightenSystemInformation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_system_atlases = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { EnlightenSystemAtlasInformation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_terrain_chunks = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { EnlightenTerrainChunksInformation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(EnlightenSceneMapping {
            m_renderers,
            m_systems,
            m_system_atlases,
            m_terrain_chunks,
        })
    }
}
