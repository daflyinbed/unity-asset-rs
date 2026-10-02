//! 自动生成。请勿手动修改。

use crate::subclasses::bone_weights4::BoneWeights4;
use crate::subclasses::int2_storage::int2_storage;
use crate::subclasses::secondary_sprite_texture::SecondarySpriteTexture;
use crate::subclasses::sprite_bone::SpriteBone;
use crate::subclasses::sprite_custom_metadata::SpriteCustomMetadata;
use crate::subclasses::sprite_meta_data::SpriteMetaData;
use crate::subclasses::vector2f::Vector2f;

/// 版本枚举：SpriteSheetMetaDataMSpritesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteSheetMetaDataMSpritesVariant {
    Default(Vec<SpriteMetaData>),
    V2017_1(Vec<SpriteMetaData>),
}

impl crate::Parse for SpriteSheetMetaDataMSpritesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SpriteMetaData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SpriteMetaData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpriteSheetMetaDataMSpritesVariant {
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

/// 版本枚举：SpriteSheetMetaDataMOutlineVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteSheetMetaDataMOutlineVariant {
    Default(Vec<Vec<Vector2f>>),
    V2017_1(Vec<Vec<Vector2f>>),
}

impl crate::Parse for SpriteSheetMetaDataMOutlineVariant {
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

impl crate::ToValue for SpriteSheetMetaDataMOutlineVariant {
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
pub struct SpriteSheetMetaData {
    pub m_sprites: SpriteSheetMetaDataMSpritesVariant,
    pub m_outline: Option<SpriteSheetMetaDataMOutlineVariant>,
    pub m_physics_shape: Option<Vec<Vec<Vector2f>>>,
    pub m_custom_data: Option<String>,
    pub m_bones: Option<Vec<SpriteBone>>,
    pub m_sprite_id: Option<String>,
    pub m_vertices: Option<Vec<Vector2f>>,
    pub m_internal_id: Option<i64>,
    pub m_indices: Option<Vec<i32>>,
    pub m_edges: Option<Vec<int2_storage>>,
    pub m_weights: Option<Vec<BoneWeights4>>,
    pub m_secondary_textures: Option<Vec<SecondarySpriteTexture>>,
    pub m_name_file_id_table: Option<Vec<(String, i64)>>,
    pub m_sprite_custom_metadata: Option<SpriteCustomMetadata>,
}

impl crate::Parse for SpriteSheetMetaData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_sprites = None;
        let mut m_outline = None;
        let mut m_physics_shape = None;
        let mut m_custom_data = None;
        let mut m_bones = None;
        let mut m_sprite_id = None;
        let mut m_vertices = None;
        let mut m_internal_id = None;
        let mut m_indices = None;
        let mut m_edges = None;
        let mut m_weights = None;
        let mut m_secondary_textures = None;
        let mut m_name_file_id_table = None;
        let mut m_sprite_custom_metadata = None;
        m_sprites = Some(SpriteSheetMetaDataMSpritesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 0, 7) {
            m_outline = Some(SpriteSheetMetaDataMOutlineVariant::parse(rdr, ver)?);
            m_custom_data = Some(rdr.read_aligned_string()?);
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
            m_secondary_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SecondarySpriteTexture::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_sprite_custom_metadata = Some(SpriteCustomMetadata::parse(rdr, ver)?);
            m_name_file_id_table = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i64()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_outline = Some(SpriteSheetMetaDataMOutlineVariant::parse(rdr, ver)?);
            m_custom_data = Some(rdr.read_aligned_string()?);
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
            m_secondary_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SecondarySpriteTexture::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_name_file_id_table = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i64()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_outline = Some(SpriteSheetMetaDataMOutlineVariant::parse(rdr, ver)?);
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
            m_secondary_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SecondarySpriteTexture::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_name_file_id_table = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i64()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_outline = Some(SpriteSheetMetaDataMOutlineVariant::parse(rdr, ver)?);
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
            m_secondary_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SecondarySpriteTexture::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_outline = Some(SpriteSheetMetaDataMOutlineVariant::parse(rdr, ver)?);
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
            m_outline = Some(SpriteSheetMetaDataMOutlineVariant::parse(rdr, ver)?);
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
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_outline = Some(SpriteSheetMetaDataMOutlineVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(SpriteSheetMetaData {
            m_sprites: m_sprites.unwrap(),
            m_outline,
            m_physics_shape,
            m_custom_data,
            m_bones,
            m_sprite_id,
            m_vertices,
            m_internal_id,
            m_indices,
            m_edges,
            m_weights,
            m_secondary_textures,
            m_name_file_id_table,
            m_sprite_custom_metadata,
        })
    }
}

impl crate::ToValue for SpriteSheetMetaData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Sprites".to_string(), self.m_sprites.to_value())),
            crate::opt_field("m_Outline", &self.m_outline),
            crate::opt_field("m_PhysicsShape", &self.m_physics_shape),
            crate::opt_field("m_CustomData", &self.m_custom_data),
            crate::opt_field("m_Bones", &self.m_bones),
            crate::opt_field("m_SpriteID", &self.m_sprite_id),
            crate::opt_field("m_Vertices", &self.m_vertices),
            crate::opt_field("m_InternalID", &self.m_internal_id),
            crate::opt_field("m_Indices", &self.m_indices),
            crate::opt_field("m_Edges", &self.m_edges),
            crate::opt_field("m_Weights", &self.m_weights),
            crate::opt_field("m_SecondaryTextures", &self.m_secondary_textures),
            crate::opt_field("m_NameFileIdTable", &self.m_name_file_id_table),
            crate::opt_field("m_SpriteCustomMetadata", &self.m_sprite_custom_metadata),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Sprites" => (self.m_sprites).inspect_value(path, offset, limit),
                "m_Outline" => (self.m_outline).inspect_value(path, offset, limit),
                "m_PhysicsShape" => (self.m_physics_shape).inspect_value(path, offset, limit),
                "m_CustomData" => (self.m_custom_data).inspect_value(path, offset, limit),
                "m_Bones" => (self.m_bones).inspect_value(path, offset, limit),
                "m_SpriteID" => (self.m_sprite_id).inspect_value(path, offset, limit),
                "m_Vertices" => (self.m_vertices).inspect_value(path, offset, limit),
                "m_InternalID" => (self.m_internal_id).inspect_value(path, offset, limit),
                "m_Indices" => (self.m_indices).inspect_value(path, offset, limit),
                "m_Edges" => (self.m_edges).inspect_value(path, offset, limit),
                "m_Weights" => (self.m_weights).inspect_value(path, offset, limit),
                "m_SecondaryTextures" => {
                    (self.m_secondary_textures).inspect_value(path, offset, limit)
                }
                "m_NameFileIdTable" => {
                    (self.m_name_file_id_table).inspect_value(path, offset, limit)
                }
                "m_SpriteCustomMetadata" => {
                    (self.m_sprite_custom_metadata).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_sprites).inspect_value(path, offset, limit))
                .map(|v| ("m_Sprites".to_string(), v)),
            ((self.m_outline).inspect_value(path, offset, limit))
                .map(|v| ("m_Outline".to_string(), v)),
            ((self.m_physics_shape).inspect_value(path, offset, limit))
                .map(|v| ("m_PhysicsShape".to_string(), v)),
            ((self.m_custom_data).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomData".to_string(), v)),
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
            ((self.m_secondary_textures).inspect_value(path, offset, limit))
                .map(|v| ("m_SecondaryTextures".to_string(), v)),
            ((self.m_name_file_id_table).inspect_value(path, offset, limit))
                .map(|v| ("m_NameFileIdTable".to_string(), v)),
            ((self.m_sprite_custom_metadata).inspect_value(path, offset, limit))
                .map(|v| ("m_SpriteCustomMetadata".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
