//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_clip_override::AnimationClipOverride;

/// 版本枚举：AnimatorOverrideControllerMClipsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimatorOverrideControllerMClipsVariant {
    Default(Vec<AnimationClipOverride>),
    V2017_1(Vec<AnimationClipOverride>),
}

impl crate::Parse for AnimatorOverrideControllerMClipsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AnimationClipOverride::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AnimationClipOverride::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimatorOverrideControllerMClipsVariant {
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
pub struct AnimatorOverrideController {
    pub m_name: String,
    pub m_controller: PPtr,
    pub m_clips: AnimatorOverrideControllerMClipsVariant,
}

impl crate::Parse for AnimatorOverrideController {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_controller = crate::PPtr::parse(rdr, ver)?;
        let m_clips = AnimatorOverrideControllerMClipsVariant::parse(rdr, ver)?;

        Ok(AnimatorOverrideController {
            m_name,
            m_controller,
            m_clips,
        })
    }
}

impl crate::ToValue for AnimatorOverrideController {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Controller".to_string(), self.m_controller.to_value())),
            Some(("m_Clips".to_string(), self.m_clips.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Controller" => (self.m_controller).inspect_value(path, offset, limit),
                "m_Clips" => (self.m_clips).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_controller).inspect_value(path, offset, limit))
                .map(|v| ("m_Controller".to_string(), v)),
            ((self.m_clips).inspect_value(path, offset, limit)).map(|v| ("m_Clips".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
