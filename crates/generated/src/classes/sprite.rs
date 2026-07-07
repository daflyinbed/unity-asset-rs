//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::sprite_bone::SpriteBone;
use crate::subclasses::sprite_render_data::SpriteRenderData;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub m_name: String,
    pub m_rect: Rectf,
    pub m_offset: Vector2f,
    pub m_pixels_to_units: f32,
    pub m_border: Option<Vector4f>,
    pub m_extrude: u32,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_rect = Rectf::parse(r, ver)?;
        let m_offset = Vector2f::parse(r, ver)?;
        let m_pixels_to_units = r.read_f32()?;
        let m_border = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(Vector4f::parse(r, ver)?)
        } else {
            None
        };
        let m_extrude = r.read_u32()?;
        r.align(4)?;
        let m_rd = SpriteRenderData::parse(r, ver)?;
        r.align(4)?;
        let m_pivot = if *ver >= crate::UnityVersion::new(5, 4, 2) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_is_polygon = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_render_data_key = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(( { let a = Guid::parse(r, ver)?; let b = r.read_i64()?; Ok((a, b)) } )?)
        } else {
            None
        };
        let m_atlas_tags = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sprite_atlas = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_physics_shape = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_bones = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SpriteBone::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_scriptable_objects = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Sprite {
            m_name,
            m_rect,
            m_offset,
            m_pixels_to_units,
            m_border,
            m_extrude,
            m_rd,
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
