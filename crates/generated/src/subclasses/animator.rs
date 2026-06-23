//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Animator {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_avatar: PPtr,
    pub m_controller: PPtr,
    pub m_culling_mode: i32,
    pub m_apply_root_motion: bool,
    pub m_animate_physics: bool,
}

impl crate::Parse for Animator {
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
        let m_avatar = crate::PPtr::parse(r, ver)?;
        let m_controller = crate::PPtr::parse(r, ver)?;
        let m_culling_mode = r.read_i32()?;
        let m_apply_root_motion = r.read_bool()?;
        let m_animate_physics = r.read_bool()?;

        Ok(Animator {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_avatar,
            m_controller,
            m_culling_mode,
            m_apply_root_motion,
            m_animate_physics,
        })
    }
}
