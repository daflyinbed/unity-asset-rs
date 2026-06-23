//! 自动生成。请勿手动修改。

use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct BaseAnimationTrack {
    pub parent: NamedObject,
}

impl crate::Parse for BaseAnimationTrack {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;

        Ok(BaseAnimationTrack {
            parent,
        })
    }
}
