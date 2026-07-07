//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct Projector {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_near_clip_plane: f32,
    pub m_far_clip_plane: f32,
    pub m_field_of_view: f32,
    pub m_aspect_ratio: f32,
    pub m_orthographic: bool,
    pub m_orthographic_size: f32,
    pub m_material: PPtr,
    pub m_ignore_layers: BitField,
}

impl crate::Parse for Projector {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_near_clip_plane = r.read_f32()?;
        let m_far_clip_plane = r.read_f32()?;
        let m_field_of_view = r.read_f32()?;
        let m_aspect_ratio = r.read_f32()?;
        let m_orthographic = r.read_bool()?;
        r.align(4)?;
        let m_orthographic_size = r.read_f32()?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_ignore_layers = BitField::parse(r, ver)?;

        Ok(Projector {
            m_game_object,
            m_enabled,
            m_near_clip_plane,
            m_far_clip_plane,
            m_field_of_view,
            m_aspect_ratio,
            m_orthographic,
            m_orthographic_size,
            m_material,
            m_ignore_layers,
        })
    }
}
