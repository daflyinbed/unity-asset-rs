//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AnimationClipOverride {
    pub m_original_clip: PPtr,
    pub m_override_clip: PPtr,
}

impl crate::Parse for AnimationClipOverride {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_original_clip = crate::PPtr::parse(r, ver)?;
        let m_override_clip = crate::PPtr::parse(r, ver)?;

        Ok(AnimationClipOverride {
            m_original_clip,
            m_override_clip,
        })
    }
}
