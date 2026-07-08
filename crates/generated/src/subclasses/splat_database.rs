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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SplatPrototype::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SplatPrototype::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            m_terrain_layers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
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
