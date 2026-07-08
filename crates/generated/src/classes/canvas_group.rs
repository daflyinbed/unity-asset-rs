//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct CanvasGroup {
    pub m_game_object: PPtr,
    pub m_alpha: f32,
    pub m_enabled: Option<u8>,
    pub m_interactable: bool,
    pub m_blocks_raycasts: bool,
    pub m_ignore_parent_groups: bool,
}

impl crate::Parse for CanvasGroup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_alpha = None;
        let mut m_enabled = None;
        let mut m_interactable = None;
        let mut m_blocks_raycasts = None;
        let mut m_ignore_parent_groups = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(4, 6, 1) {
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
        } else {
        }
            m_alpha = Some(r.read_f32()?);
            m_interactable = Some(r.read_bool()?);
            m_blocks_raycasts = Some(r.read_bool()?);
            m_ignore_parent_groups = Some(r.read_bool()?);
            r.align(4)?;

        Ok(CanvasGroup {
            m_game_object: m_game_object.unwrap(),
            m_alpha: m_alpha.unwrap(),
            m_enabled,
            m_interactable: m_interactable.unwrap(),
            m_blocks_raycasts: m_blocks_raycasts.unwrap(),
            m_ignore_parent_groups: m_ignore_parent_groups.unwrap(),
        })
    }
}
