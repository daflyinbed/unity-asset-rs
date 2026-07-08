//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;


/// 版本枚举：AnimationMAnimationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationMAnimationsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimationMAnimationsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Animation {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_animation: PPtr,
    pub m_animations: AnimationMAnimationsVariant,
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
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_animation = None;
        let mut m_animations = None;
        let mut m_wrap_mode = None;
        let mut m_play_automatically = None;
        let mut m_animate_physics = None;
        let mut m_culling_type = None;
        let mut m_update_mode = None;
        let mut m_user_aabb = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
            m_animation = Some(crate::PPtr::parse(r, ver)?);
            m_animations = Some(AnimationMAnimationsVariant::parse(r, ver)?);
            m_wrap_mode = Some(r.read_i32()?);
            m_play_automatically = Some(r.read_bool()?);
            m_animate_physics = Some(r.read_bool()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_update_mode = Some(r.read_i32()?);
        } else {
        }
            m_culling_type = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            m_user_aabb = Some(Aabb::parse(r, ver)?);
        }

        Ok(Animation {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_animation: m_animation.unwrap(),
            m_animations: m_animations.unwrap(),
            m_wrap_mode: m_wrap_mode.unwrap(),
            m_play_automatically: m_play_automatically.unwrap(),
            m_animate_physics: m_animate_physics.unwrap(),
            m_culling_type: m_culling_type.unwrap(),
            m_update_mode,
            m_user_aabb,
        })
    }
}
