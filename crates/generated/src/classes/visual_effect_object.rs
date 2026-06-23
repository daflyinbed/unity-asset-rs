//! 自动生成。请勿手动修改。

use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct VisualEffectObject {
    pub parent: NamedObject,
}

impl crate::Parse for VisualEffectObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;

        Ok(VisualEffectObject {
            parent,
        })
    }
}
