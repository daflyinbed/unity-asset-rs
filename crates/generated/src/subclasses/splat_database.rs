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
        let m_splats = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(SplatDatabaseMSplatsVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_terrain_layers = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_alpha_textures = SplatDatabaseMAlphaTexturesVariant::parse(rdr, ver)?;
        let m_alphamap_resolution = rdr.read_i32()?;
        let m_base_map_resolution = rdr.read_i32()?;
        let m_color_space = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_material_requires_metallic = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            Some(rdr.read_bool()?)
        } else {
            None
        };
        let m_material_requires_smoothness = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SplatDatabase {
            m_splats,
            m_terrain_layers,
            m_alpha_textures,
            m_alphamap_resolution,
            m_base_map_resolution,
            m_color_space,
            m_material_requires_metallic,
            m_material_requires_smoothness,
        })
    }
}
