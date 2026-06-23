//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_clip_override::AnimationClipOverride;
use super::runtime_animator_controller::RuntimeAnimatorController;

#[derive(Debug, Clone)]
pub struct AnimatorOverrideController {
    pub parent: RuntimeAnimatorController,
    pub m_name: String,
    pub m_controller: PPtr,
    pub m_clips: Vec<AnimationClipOverride>,
}

impl crate::Parse for AnimatorOverrideController {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = RuntimeAnimatorController::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_controller = crate::PPtr::parse(r, ver)?;
        let m_clips = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AnimationClipOverride::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(AnimatorOverrideController {
            parent,
            m_name,
            m_controller,
            m_clips,
        })
    }
}
