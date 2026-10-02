//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::splat_prototype::SplatPrototype;

/// 版本枚举：SplatDatabaseMSplatsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SplatDatabaseMSplatsVariant {
    Default(Vec<SplatPrototype>),
    V2017_1(Vec<SplatPrototype>),
}

impl crate::Parse for SplatDatabaseMSplatsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SplatPrototype::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SplatPrototype::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SplatDatabaseMSplatsVariant {
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

/// 版本枚举：SplatDatabaseMAlphaTexturesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SplatDatabaseMAlphaTexturesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for SplatDatabaseMAlphaTexturesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SplatDatabaseMAlphaTexturesVariant {
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
pub struct SplatDatabase {
    pub m_splats: Option<SplatDatabaseMSplatsVariant>,
    pub m_terrain_layers: Option<Vec<PPtr>>,
    pub m_alpha_textures: SplatDatabaseMAlphaTexturesVariant,
    pub m_alphamap_resolution: i32,
    pub m_base_map_resolution: i32,
    pub m_color_space: Option<i32>,
    pub m_material_requires_metallic: Option<bool>,
    pub m_material_requires_smoothness: Option<bool>,
}

impl crate::Parse for SplatDatabase {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_splats = None;
        let mut m_terrain_layers = None;
        let mut m_alpha_textures = None;
        let mut m_alphamap_resolution = None;
        let mut m_base_map_resolution = None;
        let mut m_color_space = None;
        let mut m_material_requires_metallic = None;
        let mut m_material_requires_smoothness = None;
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_terrain_layers = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
            m_splats = Some(SplatDatabaseMSplatsVariant::parse(rdr, ver)?);
        }
        m_alpha_textures = Some(SplatDatabaseMAlphaTexturesVariant::parse(rdr, ver)?);
        m_alphamap_resolution = Some(rdr.read_i32()?);
        m_base_map_resolution = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_color_space = Some(rdr.read_i32()?);
            m_material_requires_metallic = Some(rdr.read_bool()?);
            m_material_requires_smoothness = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(SplatDatabase {
            m_splats,
            m_terrain_layers,
            m_alpha_textures: m_alpha_textures.unwrap(),
            m_alphamap_resolution: m_alphamap_resolution.unwrap(),
            m_base_map_resolution: m_base_map_resolution.unwrap(),
            m_color_space,
            m_material_requires_metallic,
            m_material_requires_smoothness,
        })
    }
}

impl crate::ToValue for SplatDatabase {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_Splats", &self.m_splats),
            crate::opt_field("m_TerrainLayers", &self.m_terrain_layers),
            Some((
                "m_AlphaTextures".to_string(),
                self.m_alpha_textures.to_value(),
            )),
            Some((
                "m_AlphamapResolution".to_string(),
                self.m_alphamap_resolution.to_value(),
            )),
            Some((
                "m_BaseMapResolution".to_string(),
                self.m_base_map_resolution.to_value(),
            )),
            crate::opt_field("m_ColorSpace", &self.m_color_space),
            crate::opt_field(
                "m_MaterialRequiresMetallic",
                &self.m_material_requires_metallic,
            ),
            crate::opt_field(
                "m_MaterialRequiresSmoothness",
                &self.m_material_requires_smoothness,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Splats" => (self.m_splats).inspect_value(path, offset, limit),
                "m_TerrainLayers" => (self.m_terrain_layers).inspect_value(path, offset, limit),
                "m_AlphaTextures" => (self.m_alpha_textures).inspect_value(path, offset, limit),
                "m_AlphamapResolution" => {
                    (self.m_alphamap_resolution).inspect_value(path, offset, limit)
                }
                "m_BaseMapResolution" => {
                    (self.m_base_map_resolution).inspect_value(path, offset, limit)
                }
                "m_ColorSpace" => (self.m_color_space).inspect_value(path, offset, limit),
                "m_MaterialRequiresMetallic" => {
                    (self.m_material_requires_metallic).inspect_value(path, offset, limit)
                }
                "m_MaterialRequiresSmoothness" => {
                    (self.m_material_requires_smoothness).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_splats).inspect_value(path, offset, limit))
                .map(|v| ("m_Splats".to_string(), v)),
            ((self.m_terrain_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_TerrainLayers".to_string(), v)),
            ((self.m_alpha_textures).inspect_value(path, offset, limit))
                .map(|v| ("m_AlphaTextures".to_string(), v)),
            ((self.m_alphamap_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_AlphamapResolution".to_string(), v)),
            ((self.m_base_map_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_BaseMapResolution".to_string(), v)),
            ((self.m_color_space).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorSpace".to_string(), v)),
            ((self.m_material_requires_metallic).inspect_value(path, offset, limit))
                .map(|v| ("m_MaterialRequiresMetallic".to_string(), v)),
            ((self.m_material_requires_smoothness).inspect_value(path, offset, limit))
                .map(|v| ("m_MaterialRequiresSmoothness".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
