//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::sprite_bone::SpriteBone;
use crate::subclasses::sprite_render_data::SpriteRenderData;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：SpriteMExtrudeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteMExtrudeVariant {
    Default(u32),
    V6000_5(u32),
}

impl crate::Parse for SpriteMExtrudeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_u32()?;
            rdr.align(4)?;
            Ok(Self::V6000_5(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Sprite {
    pub m_name: String,
    pub m_rect: Rectf,
    pub m_offset: Vector2f,
    pub m_pixels_to_units: f32,
    pub m_border: Option<Vector4f>,
    pub m_extrude: SpriteMExtrudeVariant,
    pub m_rd: SpriteRenderData,
    pub m_pivot: Option<Vector2f>,
    pub m_is_polygon: Option<bool>,
    pub m_render_data_key: Option<(Guid, i64)>,
    pub m_atlas_tags: Option<Vec<String>>,
    pub m_sprite_atlas: Option<PPtr>,
    pub m_physics_shape: Option<Vec<Vec<Vector2f>>>,
    pub m_bones: Option<Vec<SpriteBone>>,
    pub m_scriptable_objects: Option<Vec<PPtr>>,
}

impl crate::Parse for Sprite {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_rect = None;
        let mut m_offset = None;
        let mut m_pixels_to_units = None;
        let mut m_border = None;
        let mut m_extrude = None;
        let mut m_rd = None;
        let mut m_pivot = None;
        let mut m_is_polygon = None;
        let mut m_render_data_key = None;
        let mut m_atlas_tags = None;
        let mut m_sprite_atlas = None;
        let mut m_physics_shape = None;
        let mut m_bones = None;
        let mut m_scriptable_objects = None;
            m_name = Some(rdr.read_aligned_string()?);
            m_rect = Some(Rectf::parse(rdr, ver)?);
            m_offset = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_border = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }
            m_pixels_to_units = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 4, 2) {
            m_pivot = Some(Vector2f::parse(rdr, ver)?);
        } else {
        }
            m_extrude = Some(SpriteMExtrudeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_render_data_key = Some(( { let a = Guid::parse(rdr, ver)?; let b = rdr.read_i64()?; Ok((a, b)) } )?);
            m_atlas_tags = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_sprite_atlas = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_is_polygon = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_render_data_key = Some(( { let a = Guid::parse(rdr, ver)?; let b = rdr.read_i64()?; Ok((a, b)) } )?);
            m_atlas_tags = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_sprite_atlas = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_is_polygon = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_rd = Some(SpriteRenderData::parse(rdr, ver)?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_physics_shape = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_bones = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SpriteBone::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_scriptable_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_physics_shape = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_bones = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SpriteBone::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_physics_shape = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }

        Ok(Sprite {
            m_name: m_name.unwrap(),
            m_rect: m_rect.unwrap(),
            m_offset: m_offset.unwrap(),
            m_pixels_to_units: m_pixels_to_units.unwrap(),
            m_border,
            m_extrude: m_extrude.unwrap(),
            m_rd: m_rd.unwrap(),
            m_pivot,
            m_is_polygon,
            m_render_data_key,
            m_atlas_tags,
            m_sprite_atlas,
            m_physics_shape,
            m_bones,
            m_scriptable_objects,
        })
    }
}
