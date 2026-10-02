//! 自动生成。请勿手动修改。

use crate::subclasses::enlighten_renderer_information::EnlightenRendererInformation;
use crate::subclasses::enlighten_system_atlas_information::EnlightenSystemAtlasInformation;
use crate::subclasses::enlighten_system_information::EnlightenSystemInformation;
use crate::subclasses::enlighten_terrain_chunks_information::EnlightenTerrainChunksInformation;
use crate::subclasses::hash128::Hash128;

/// 版本枚举：EnlightenSceneMappingMRenderersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EnlightenSceneMappingMRenderersVariant {
    Default(Vec<EnlightenRendererInformation>),
    V2017_1(Vec<EnlightenRendererInformation>),
}

impl crate::Parse for EnlightenSceneMappingMRenderersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EnlightenRendererInformation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EnlightenRendererInformation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EnlightenSceneMappingMRenderersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EnlightenSceneMappingMSystemsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EnlightenSceneMappingMSystemsVariant {
    Default(Vec<EnlightenSystemInformation>),
    V2017_1(Vec<EnlightenSystemInformation>),
}

impl crate::Parse for EnlightenSceneMappingMSystemsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EnlightenSystemInformation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EnlightenSystemInformation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EnlightenSceneMappingMSystemsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EnlightenSceneMappingMSystemAtlasesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EnlightenSceneMappingMSystemAtlasesVariant {
    Default(Vec<EnlightenSystemAtlasInformation>),
    V2017_1(Vec<EnlightenSystemAtlasInformation>),
}

impl crate::Parse for EnlightenSceneMappingMSystemAtlasesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EnlightenSystemAtlasInformation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EnlightenSystemAtlasInformation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EnlightenSceneMappingMSystemAtlasesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EnlightenSceneMappingMProbesetsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EnlightenSceneMappingMProbesetsVariant {
    Default(Vec<Hash128>),
    V2017_1(Vec<Hash128>),
}

impl crate::Parse for EnlightenSceneMappingMProbesetsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Hash128::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Hash128::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EnlightenSceneMappingMProbesetsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EnlightenSceneMappingMTerrainChunksVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EnlightenSceneMappingMTerrainChunksVariant {
    Default(Vec<EnlightenTerrainChunksInformation>),
    V2017_1(Vec<EnlightenTerrainChunksInformation>),
}

impl crate::Parse for EnlightenSceneMappingMTerrainChunksVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EnlightenTerrainChunksInformation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EnlightenTerrainChunksInformation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EnlightenSceneMappingMTerrainChunksVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EnlightenSceneMapping {
    pub m_renderers: EnlightenSceneMappingMRenderersVariant,
    pub m_systems: EnlightenSceneMappingMSystemsVariant,
    pub m_system_atlases: EnlightenSceneMappingMSystemAtlasesVariant,
    pub m_probesets: Option<EnlightenSceneMappingMProbesetsVariant>,
    pub m_terrain_chunks: EnlightenSceneMappingMTerrainChunksVariant,
}

impl crate::Parse for EnlightenSceneMapping {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_renderers = None;
        let mut m_systems = None;
        let mut m_system_atlases = None;
        let mut m_probesets = None;
        let mut m_terrain_chunks = None;
        m_renderers = Some(EnlightenSceneMappingMRenderersVariant::parse(rdr, ver)?);
        m_systems = Some(EnlightenSceneMappingMSystemsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_probesets = Some(EnlightenSceneMappingMProbesetsVariant::parse(rdr, ver)?);
        } else {
        }
        m_system_atlases = Some(EnlightenSceneMappingMSystemAtlasesVariant::parse(rdr, ver)?);
        m_terrain_chunks = Some(EnlightenSceneMappingMTerrainChunksVariant::parse(rdr, ver)?);

        Ok(EnlightenSceneMapping {
            m_renderers: m_renderers.unwrap(),
            m_systems: m_systems.unwrap(),
            m_system_atlases: m_system_atlases.unwrap(),
            m_probesets,
            m_terrain_chunks: m_terrain_chunks.unwrap(),
        })
    }
}

impl crate::ToValue for EnlightenSceneMapping {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Renderers".to_string(), self.m_renderers.to_value())),
            Some(("m_Systems".to_string(), self.m_systems.to_value())),
            Some((
                "m_SystemAtlases".to_string(),
                self.m_system_atlases.to_value(),
            )),
            crate::opt_field("m_Probesets", &self.m_probesets),
            Some((
                "m_TerrainChunks".to_string(),
                self.m_terrain_chunks.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Renderers" => (self.m_renderers).inspect_value(path, offset, limit),
                "m_Systems" => (self.m_systems).inspect_value(path, offset, limit),
                "m_SystemAtlases" => (self.m_system_atlases).inspect_value(path, offset, limit),
                "m_Probesets" => (self.m_probesets).inspect_value(path, offset, limit),
                "m_TerrainChunks" => (self.m_terrain_chunks).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_renderers).inspect_value(path, offset, limit))
                .map(|v| ("m_Renderers".to_string(), v)),
            ((self.m_systems).inspect_value(path, offset, limit))
                .map(|v| ("m_Systems".to_string(), v)),
            ((self.m_system_atlases).inspect_value(path, offset, limit))
                .map(|v| ("m_SystemAtlases".to_string(), v)),
            ((self.m_probesets).inspect_value(path, offset, limit))
                .map(|v| ("m_Probesets".to_string(), v)),
            ((self.m_terrain_chunks).inspect_value(path, offset, limit))
                .map(|v| ("m_TerrainChunks".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
