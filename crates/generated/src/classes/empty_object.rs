//! 自动生成。请勿手动修改。

use super::object::Object;

#[derive(Debug, Clone)]
pub struct EmptyObject {
    pub parent: Object,
}

impl crate::Parse for EmptyObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;

        Ok(EmptyObject {
            parent,
        })
    }
}
