//! 自动生成。请勿手动修改。

use super::object::Object;

#[derive(Debug, Clone)]
pub struct MarshallingTestObject {
    pub parent: Object,
    pub m_prop: i32,
}

impl crate::Parse for MarshallingTestObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_prop = r.read_i32()?;

        Ok(MarshallingTestObject {
            parent,
            m_prop,
        })
    }
}
