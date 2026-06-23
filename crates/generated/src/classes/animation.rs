//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct Animation {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_animation: PPtr,
    pub m_animations: Vec<PPtr>,
    pub m_wrap_mode: i32,
    pub m_play_automatically: bool,
    pub m_animate_physics: bool,
    pub m_culling_type: i32,
    pub m_update_mode: Option<i32>,
    pub m_user_aabb: Option<Aabb>,
}

impl crate::Parse for Animation {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_animation = crate::PPtr::parse(r, ver)?;
        let m_animations = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_wrap_mode = r.read_i32()?;
        let m_play_automatically = r.read_bool()?;
        let m_animate_physics = r.read_bool()?;
        r.align(4)?;
        let m_culling_type = r.read_i32()?;
        let m_update_mode = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_user_aabb = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(Aabb::parse(r, ver)?)
        } else {
            None
        };

        Ok(Animation {
            parent,
            m_game_object,
            m_enabled,
            m_animation,
            m_animations,
            m_wrap_mode,
            m_play_automatically,
            m_animate_physics,
            m_culling_type,
            m_update_mode,
            m_user_aabb,
        })
    }
}
