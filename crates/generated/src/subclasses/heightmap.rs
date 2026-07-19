//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：HeightmapMPrecomputedErrorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HeightmapMPrecomputedErrorVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for HeightmapMPrecomputedErrorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：HeightmapMMinMaxPatchHeightsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HeightmapMMinMaxPatchHeightsVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for HeightmapMMinMaxPatchHeightsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Heightmap {
    pub m_heights: Vec<i16>,
    pub m_precomputed_error: HeightmapMPrecomputedErrorVariant,
    pub m_holes: Option<Vec<u8>>,
    pub m_min_max_patch_heights: HeightmapMMinMaxPatchHeightsVariant,
    pub m_holes_lod: Option<Vec<u8>>,
    pub m_default_physic_material: Option<PPtr>,
    pub m_width: Option<i32>,
    pub m_enable_holes_texture_compression: Option<bool>,
    pub m_height: Option<i32>,
    pub m_thickness: Option<f32>,
    pub m_levels: i32,
    pub m_resolution: Option<i32>,
    pub m_scale: Vector3f,
}

impl crate::Parse for Heightmap {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_heights = None;
        let mut m_precomputed_error = None;
        let mut m_holes = None;
        let mut m_min_max_patch_heights = None;
        let mut m_holes_lod = None;
        let mut m_default_physic_material = None;
        let mut m_width = None;
        let mut m_enable_holes_texture_compression = None;
        let mut m_height = None;
        let mut m_thickness = None;
        let mut m_levels = None;
        let mut m_resolution = None;
        let mut m_scale = None;
        m_heights = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i16())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_holes = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_holes_lod = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_enable_holes_texture_compression = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_precomputed_error = Some(HeightmapMPrecomputedErrorVariant::parse(rdr, ver)?);
        m_min_max_patch_heights = Some(HeightmapMMinMaxPatchHeightsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_resolution = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
            m_thickness = Some(rdr.read_f32()?);
        } else {
            m_default_physic_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_width = Some(rdr.read_i32()?);
            m_height = Some(rdr.read_i32()?);
        }
        m_levels = Some(rdr.read_i32()?);
        m_scale = Some(Vector3f::parse(rdr, ver)?);

        Ok(Heightmap {
            m_heights: m_heights.unwrap(),
            m_precomputed_error: m_precomputed_error.unwrap(),
            m_holes,
            m_min_max_patch_heights: m_min_max_patch_heights.unwrap(),
            m_holes_lod,
            m_default_physic_material,
            m_width,
            m_enable_holes_texture_compression,
            m_height,
            m_thickness,
            m_levels: m_levels.unwrap(),
            m_resolution,
            m_scale: m_scale.unwrap(),
        })
    }
}
