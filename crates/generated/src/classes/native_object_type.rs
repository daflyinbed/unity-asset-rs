//! 自动生成。请勿手动修改。

use crate::subclasses::native_type::NativeType;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct NativeObjectType {
    pub parent: Object,
    pub m_inner: NativeType,
}

impl crate::Parse for NativeObjectType {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_inner = NativeType::parse(r, ver)?;
        r.align(4)?;

        Ok(NativeObjectType {
            parent,
            m_inner,
        })
    }
}
