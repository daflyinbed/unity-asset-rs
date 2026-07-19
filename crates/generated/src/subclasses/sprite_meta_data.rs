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
