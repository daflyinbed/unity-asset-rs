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

impl crate::ToValue for HeightmapMPrecomputedErrorVariant {
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

impl crate::ToValue for HeightmapMMinMaxPatchHeightsVariant {
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

impl crate::ToValue for Heightmap {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Heights".to_string(), self.m_heights.to_value())),
            Some((
                "m_PrecomputedError".to_string(),
                self.m_precomputed_error.to_value(),
            )),
            crate::opt_field("m_Holes", &self.m_holes),
            Some((
                "m_MinMaxPatchHeights".to_string(),
                self.m_min_max_patch_heights.to_value(),
            )),
            crate::opt_field("m_HolesLOD", &self.m_holes_lod),
            crate::opt_field("m_DefaultPhysicMaterial", &self.m_default_physic_material),
            crate::opt_field("m_Width", &self.m_width),
            crate::opt_field(
                "m_EnableHolesTextureCompression",
                &self.m_enable_holes_texture_compression,
            ),
            crate::opt_field("m_Height", &self.m_height),
            crate::opt_field("m_Thickness", &self.m_thickness),
            Some(("m_Levels".to_string(), self.m_levels.to_value())),
            crate::opt_field("m_Resolution", &self.m_resolution),
            Some(("m_Scale".to_string(), self.m_scale.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Heights" => (self.m_heights).inspect_value(path, offset, limit),
                "m_PrecomputedError" => {
                    (self.m_precomputed_error).inspect_value(path, offset, limit)
                }
                "m_Holes" => (self.m_holes).inspect_value(path, offset, limit),
                "m_MinMaxPatchHeights" => {
                    (self.m_min_max_patch_heights).inspect_value(path, offset, limit)
                }
                "m_HolesLOD" => (self.m_holes_lod).inspect_value(path, offset, limit),
                "m_DefaultPhysicMaterial" => {
                    (self.m_default_physic_material).inspect_value(path, offset, limit)
                }
                "m_Width" => (self.m_width).inspect_value(path, offset, limit),
                "m_EnableHolesTextureCompression" => {
                    (self.m_enable_holes_texture_compression).inspect_value(path, offset, limit)
                }
                "m_Height" => (self.m_height).inspect_value(path, offset, limit),
                "m_Thickness" => (self.m_thickness).inspect_value(path, offset, limit),
                "m_Levels" => (self.m_levels).inspect_value(path, offset, limit),
                "m_Resolution" => (self.m_resolution).inspect_value(path, offset, limit),
                "m_Scale" => (self.m_scale).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_heights).inspect_value(path, offset, limit))
                .map(|v| ("m_Heights".to_string(), v)),
            ((self.m_precomputed_error).inspect_value(path, offset, limit))
                .map(|v| ("m_PrecomputedError".to_string(), v)),
            ((self.m_holes).inspect_value(path, offset, limit)).map(|v| ("m_Holes".to_string(), v)),
            ((self.m_min_max_patch_heights).inspect_value(path, offset, limit))
                .map(|v| ("m_MinMaxPatchHeights".to_string(), v)),
            ((self.m_holes_lod).inspect_value(path, offset, limit))
                .map(|v| ("m_HolesLOD".to_string(), v)),
            ((self.m_default_physic_material).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultPhysicMaterial".to_string(), v)),
            ((self.m_width).inspect_value(path, offset, limit)).map(|v| ("m_Width".to_string(), v)),
            ((self.m_enable_holes_texture_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableHolesTextureCompression".to_string(), v)),
            ((self.m_height).inspect_value(path, offset, limit))
                .map(|v| ("m_Height".to_string(), v)),
            ((self.m_thickness).inspect_value(path, offset, limit))
                .map(|v| ("m_Thickness".to_string(), v)),
            ((self.m_levels).inspect_value(path, offset, limit))
                .map(|v| ("m_Levels".to_string(), v)),
            ((self.m_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_Resolution".to_string(), v)),
            ((self.m_scale).inspect_value(path, offset, limit)).map(|v| ("m_Scale".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
