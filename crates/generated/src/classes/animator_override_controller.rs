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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AnimationClipOverride::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AnimationClipOverride::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_controller = crate::PPtr::parse(r, ver)?;
        let m_clips = AnimatorOverrideControllerMClipsVariant::parse(r, ver)?;

        Ok(AnimatorOverrideController {
            m_name,
            m_controller,
            m_clips,
        })
    }
}
