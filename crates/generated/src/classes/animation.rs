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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimationMAnimationsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
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
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_animation = Some(crate::PPtr::parse(rdr, ver)?);
        m_animations = Some(AnimationMAnimationsVariant::parse(rdr, ver)?);
        m_wrap_mode = Some(rdr.read_i32()?);
        m_play_automatically = Some(rdr.read_bool()?);
        m_animate_physics = Some(rdr.read_bool()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_update_mode = Some(rdr.read_i32()?);
        } else {
        }
        m_culling_type = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            m_user_aabb = Some(Aabb::parse(rdr, ver)?);
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

impl crate::ToValue for Animation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Animation".to_string(), self.m_animation.to_value())),
            Some(("m_Animations".to_string(), self.m_animations.to_value())),
            Some(("m_WrapMode".to_string(), self.m_wrap_mode.to_value())),
            Some((
                "m_PlayAutomatically".to_string(),
                self.m_play_automatically.to_value(),
            )),
            Some((
                "m_AnimatePhysics".to_string(),
                self.m_animate_physics.to_value(),
            )),
            Some(("m_CullingType".to_string(), self.m_culling_type.to_value())),
            crate::opt_field("m_UpdateMode", &self.m_update_mode),
            crate::opt_field("m_UserAABB", &self.m_user_aabb),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Animation" => (self.m_animation).inspect_value(path, offset, limit),
                "m_Animations" => (self.m_animations).inspect_value(path, offset, limit),
                "m_WrapMode" => (self.m_wrap_mode).inspect_value(path, offset, limit),
                "m_PlayAutomatically" => {
                    (self.m_play_automatically).inspect_value(path, offset, limit)
                }
                "m_AnimatePhysics" => (self.m_animate_physics).inspect_value(path, offset, limit),
                "m_CullingType" => (self.m_culling_type).inspect_value(path, offset, limit),
                "m_UpdateMode" => (self.m_update_mode).inspect_value(path, offset, limit),
                "m_UserAABB" => (self.m_user_aabb).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_animation).inspect_value(path, offset, limit))
                .map(|v| ("m_Animation".to_string(), v)),
            ((self.m_animations).inspect_value(path, offset, limit))
                .map(|v| ("m_Animations".to_string(), v)),
            ((self.m_wrap_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_WrapMode".to_string(), v)),
            ((self.m_play_automatically).inspect_value(path, offset, limit))
                .map(|v| ("m_PlayAutomatically".to_string(), v)),
            ((self.m_animate_physics).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimatePhysics".to_string(), v)),
            ((self.m_culling_type).inspect_value(path, offset, limit))
                .map(|v| ("m_CullingType".to_string(), v)),
            ((self.m_update_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateMode".to_string(), v)),
            ((self.m_user_aabb).inspect_value(path, offset, limit))
                .map(|v| ("m_UserAABB".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
