//! 自动生成。请勿手动修改。

use super::visual_effect_object::VisualEffectObject;

#[derive(Debug, Clone)]
pub struct VisualEffectSubgraph {
    pub parent: VisualEffectObject,
}

impl crate::Parse for VisualEffectSubgraph {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = VisualEffectObject::parse(r, ver)?;

        Ok(VisualEffectSubgraph {
            parent,
        })
    }
}
