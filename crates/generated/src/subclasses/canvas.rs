//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_alpha: f32,
    pub m_render_mode: i32,
    pub m_camera: PPtr,
    pub m_normals: bool,
    pub m_position_u_vs: bool,
    pub m_pixel_perfect: bool,
}

impl crate::Parse for Canvas {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_alpha = r.read_f32()?;
        let m_render_mode = r.read_i32()?;
        let m_camera = crate::PPtr::parse(r, ver)?;
        let m_normals = r.read_bool()?;
        let m_position_u_vs = r.read_bool()?;
        let m_pixel_perfect = r.read_bool()?;

        Ok(Canvas {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_alpha,
            m_render_mode,
            m_camera,
            m_normals,
            m_position_u_vs,
            m_pixel_perfect,
        })
    }
}
