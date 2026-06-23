//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct CharacterController {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_height: f32,
    pub m_radius: f32,
    pub m_slope_limit: f32,
    pub m_step_offset: f32,
    pub m_skin_width: f32,
    pub m_min_move_distance: f32,
    pub m_center: Vector3f,
}

impl crate::Parse for CharacterController {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        r.align(4)?;
        let m_height = r.read_f32()?;
        let m_radius = r.read_f32()?;
        let m_slope_limit = r.read_f32()?;
        let m_step_offset = r.read_f32()?;
        let m_skin_width = r.read_f32()?;
        let m_min_move_distance = r.read_f32()?;
        let m_center = Vector3f::parse(r, ver)?;

        Ok(CharacterController {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_height,
            m_radius,
            m_slope_limit,
            m_step_offset,
            m_skin_width,
            m_min_move_distance,
            m_center,
        })
    }
}
