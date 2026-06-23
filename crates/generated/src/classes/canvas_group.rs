//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::behaviour::Behaviour;
use super::component::Component;

#[derive(Debug, Clone)]
pub enum CanvasGroupParent {
    Component(Component),
    Behaviour(Behaviour),
}

#[derive(Debug, Clone)]
pub struct CanvasGroup {
    pub parent: CanvasGroupParent,
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
        let parent = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            CanvasGroupParent::Component(Component::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(4, 6, 1) {
            CanvasGroupParent::Behaviour(Behaviour::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_alpha = r.read_f32()?;
        let m_enabled = if *ver >= crate::UnityVersion::new(4, 6, 1) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_interactable = r.read_bool()?;
        let m_blocks_raycasts = r.read_bool()?;
        let m_ignore_parent_groups = r.read_bool()?;
        r.align(4)?;

        Ok(CanvasGroup {
            parent,
            m_game_object,
            m_alpha,
            m_enabled,
            m_interactable,
            m_blocks_raycasts,
            m_ignore_parent_groups,
        })
    }
}
