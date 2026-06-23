//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct CanvasGroup {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_alpha: f32,
    pub m_interactable: bool,
    pub m_blocks_raycasts: bool,
    pub m_ignore_parent_groups: bool,
}

impl crate::Parse for CanvasGroup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_alpha = r.read_f32()?;
        let m_interactable = r.read_bool()?;
        let m_blocks_raycasts = r.read_bool()?;
        let m_ignore_parent_groups = r.read_bool()?;
        r.align(4)?;

        Ok(CanvasGroup {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_alpha,
            m_interactable,
            m_blocks_raycasts,
            m_ignore_parent_groups,
        })
    }
}
