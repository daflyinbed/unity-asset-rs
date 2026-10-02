//! 自动生成。请勿手动修改。

use crate::subclasses::bone_weights4::BoneWeights4;
use crate::subclasses::int2_storage::int2_storage;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::sprite_bone::SpriteBone;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：SpriteMetaDataMOutlineVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteMetaDataMOutlineVariant {
    Default(Vec<Vec<Vector2f>>),
    V2017_1(Vec<Vec<Vector2f>>),
}

impl crate::Parse for SpriteMetaDataMOutlineVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let n = rdr.read_i32()? as usize;
                            (0..n)
                                .map(|_| Vector2f::parse(rdr, ver))
                                .collect::<Result<Vec<_>, _>>()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let n = rdr.read_i32()? as usize;
                            (0..n)
                                .map(|_| Vector2f::parse(rdr, ver))
                                .collect::<Result<Vec<_>, _>>()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpriteMetaDataMOutlineVariant {
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
pub struct SpriteMetaData {
    pub m_name: String,
    pub m_rect: Rectf,
    pub m_alignment: i32,
    pub m_pivot: Vector2f,
    pub m_border: Option<Vector4f>,
    pub m_outline: Option<SpriteMetaDataMOutlineVariant>,
    pub m_custom_data: Option<String>,
    pub m_tessellation_detail: Option<f32>,
    pub m_physics_shape: Option<Vec<Vec<Vector2f>>>,
    pub m_bones: Option<Vec<SpriteBone>>,
    pub m_sprite_id: Option<String>,
    pub m_vertices: Option<Vec<Vector2f>>,
    pub m_internal_id: Option<i64>,
    pub m_indices: Option<Vec<i32>>,
    pub m_edges: Option<Vec<int2_storage>>,
    pub m_weights: Option<Vec<BoneWeights4>>,
}

impl crate::Parse for SpriteMetaData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_rect = None;
        let mut m_alignment = None;
        let mut m_pivot = None;
        let mut m_border = None;
        let mut m_outline = None;
        let mut m_custom_data = None;
        let mut m_tessellation_detail = None;
        let mut m_physics_shape = None;
        let mut m_bones = None;
        let mut m_sprite_id = None;
        let mut m_vertices = None;
        let mut m_internal_id = None;
        let mut m_indices = None;
        let mut m_edges = None;
        let mut m_weights = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_rect = Some(Rectf::parse(rdr, ver)?);
        m_alignment = Some(rdr.read_i32()?);
        m_pivot = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_border = Some(Vector4f::parse(rdr, ver)?);
            m_custom_data = Some(rdr.read_aligned_string()?);
            m_outline = Some(SpriteMetaDataMOutlineVariant::parse(rdr, ver)?);
            m_physics_shape = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let n = rdr.read_i32()? as usize;
                                (0..n)
                                    .map(|_| Vector2f::parse(rdr, ver))
                                    .collect::<Result<Vec<_>, _>>()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_tessellation_detail = Some(rdr.read_f32()?);
            m_bones = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SpriteBone::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_sprite_id = Some(rdr.read_aligned_string()?);
            m_internal_id = Some(rdr.read_i64()?);
            m_vertices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector2f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_edges = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| int2_storage::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_weights = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| BoneWeights4::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_border = Some(Vector4f::parse(rdr, ver)?);
            m_outline = Some(SpriteMetaDataMOutlineVariant::parse(rdr, ver)?);
            m_physics_shape = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let n = rdr.read_i32()? as usize;
                                (0..n)
                                    .map(|_| Vector2f::parse(rdr, ver))
                                    .collect::<Result<Vec<_>, _>>()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_tessellation_detail = Some(rdr.read_f32()?);
            m_bones = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SpriteBone::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_sprite_id = Some(rdr.read_aligned_string()?);
            m_internal_id = Some(rdr.read_i64()?);
            m_vertices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector2f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_edges = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| int2_storage::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_weights = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| BoneWeights4::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_border = Some(Vector4f::parse(rdr, ver)?);
            m_outline = Some(SpriteMetaDataMOutlineVariant::parse(rdr, ver)?);
            m_physics_shape = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let n = rdr.read_i32()? as usize;
                                (0..n)
                                    .map(|_| Vector2f::parse(rdr, ver))
                                    .collect::<Result<Vec<_>, _>>()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_tessellation_detail = Some(rdr.read_f32()?);
            m_bones = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SpriteBone::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_sprite_id = Some(rdr.read_aligned_string()?);
            m_vertices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Vector2f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_edges = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| int2_storage::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_weights = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| BoneWeights4::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_border = Some(Vector4f::parse(rdr, ver)?);
            m_outline = Some(SpriteMetaDataMOutlineVariant::parse(rdr, ver)?);
            m_physics_shape = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let n = rdr.read_i32()? as usize;
                                (0..n)
                                    .map(|_| Vector2f::parse(rdr, ver))
                                    .collect::<Result<Vec<_>, _>>()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_tessellation_detail = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_border = Some(Vector4f::parse(rdr, ver)?);
            m_outline = Some(SpriteMetaDataMOutlineVariant::parse(rdr, ver)?);
            m_tessellation_detail = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_border = Some(Vector4f::parse(rdr, ver)?);
            m_outline = Some(SpriteMetaDataMOutlineVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_border = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }

        Ok(SpriteMetaData {
            m_name: m_name.unwrap(),
            m_rect: m_rect.unwrap(),
            m_alignment: m_alignment.unwrap(),
            m_pivot: m_pivot.unwrap(),
            m_border,
            m_outline,
            m_custom_data,
            m_tessellation_detail,
            m_physics_shape,
            m_bones,
            m_sprite_id,
            m_vertices,
            m_internal_id,
            m_indices,
            m_edges,
            m_weights,
        })
    }
}

impl crate::ToValue for SpriteMetaData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Rect".to_string(), self.m_rect.to_value())),
            Some(("m_Alignment".to_string(), self.m_alignment.to_value())),
            Some(("m_Pivot".to_string(), self.m_pivot.to_value())),
            crate::opt_field("m_Border", &self.m_border),
            crate::opt_field("m_Outline", &self.m_outline),
            crate::opt_field("m_CustomData", &self.m_custom_data),
            crate::opt_field("m_TessellationDetail", &self.m_tessellation_detail),
            crate::opt_field("m_PhysicsShape", &self.m_physics_shape),
            crate::opt_field("m_Bones", &self.m_bones),
            crate::opt_field("m_SpriteID", &self.m_sprite_id),
            crate::opt_field("m_Vertices", &self.m_vertices),
            crate::opt_field("m_InternalID", &self.m_internal_id),
            crate::opt_field("m_Indices", &self.m_indices),
            crate::opt_field("m_Edges", &self.m_edges),
            crate::opt_field("m_Weights", &self.m_weights),
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
                "m_Rect" => (self.m_rect).inspect_value(path, offset, limit),
                "m_Alignment" => (self.m_alignment).inspect_value(path, offset, limit),
                "m_Pivot" => (self.m_pivot).inspect_value(path, offset, limit),
                "m_Border" => (self.m_border).inspect_value(path, offset, limit),
                "m_Outline" => (self.m_outline).inspect_value(path, offset, limit),
                "m_CustomData" => (self.m_custom_data).inspect_value(path, offset, limit),
                "m_TessellationDetail" => {
                    (self.m_tessellation_detail).inspect_value(path, offset, limit)
                }
                "m_PhysicsShape" => (self.m_physics_shape).inspect_value(path, offset, limit),
                "m_Bones" => (self.m_bones).inspect_value(path, offset, limit),
                "m_SpriteID" => (self.m_sprite_id).inspect_value(path, offset, limit),
                "m_Vertices" => (self.m_vertices).inspect_value(path, offset, limit),
                "m_InternalID" => (self.m_internal_id).inspect_value(path, offset, limit),
                "m_Indices" => (self.m_indices).inspect_value(path, offset, limit),
                "m_Edges" => (self.m_edges).inspect_value(path, offset, limit),
                "m_Weights" => (self.m_weights).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_rect).inspect_value(path, offset, limit)).map(|v| ("m_Rect".to_string(), v)),
            ((self.m_alignment).inspect_value(path, offset, limit))
                .map(|v| ("m_Alignment".to_string(), v)),
            ((self.m_pivot).inspect_value(path, offset, limit)).map(|v| ("m_Pivot".to_string(), v)),
            ((self.m_border).inspect_value(path, offset, limit))
                .map(|v| ("m_Border".to_string(), v)),
            ((self.m_outline).inspect_value(path, offset, limit))
                .map(|v| ("m_Outline".to_string(), v)),
            ((self.m_custom_data).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomData".to_string(), v)),
            ((self.m_tessellation_detail).inspect_value(path, offset, limit))
                .map(|v| ("m_TessellationDetail".to_string(), v)),
            ((self.m_physics_shape).inspect_value(path, offset, limit))
                .map(|v| ("m_PhysicsShape".to_string(), v)),
            ((self.m_bones).inspect_value(path, offset, limit)).map(|v| ("m_Bones".to_string(), v)),
            ((self.m_sprite_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SpriteID".to_string(), v)),
            ((self.m_vertices).inspect_value(path, offset, limit))
                .map(|v| ("m_Vertices".to_string(), v)),
            ((self.m_internal_id).inspect_value(path, offset, limit))
                .map(|v| ("m_InternalID".to_string(), v)),
            ((self.m_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_Indices".to_string(), v)),
            ((self.m_edges).inspect_value(path, offset, limit)).map(|v| ("m_Edges".to_string(), v)),
            ((self.m_weights).inspect_value(path, offset, limit))
                .map(|v| ("m_Weights".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
