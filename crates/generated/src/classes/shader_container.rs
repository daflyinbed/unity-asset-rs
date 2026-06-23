//! 自动生成。请勿手动修改。

use super::object::Object;

#[derive(Debug, Clone)]
pub struct ShaderContainer {
    pub parent: Object,
}

impl crate::Parse for ShaderContainer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;

        Ok(ShaderContainer {
            parent,
        })
    }
}
