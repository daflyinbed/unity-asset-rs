//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct SerializableManagedHost {
    pub parent: Object,
    pub m_script: PPtr,
}

impl crate::Parse for SerializableManagedHost {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_script = crate::PPtr::parse(r, ver)?;

        Ok(SerializableManagedHost {
            parent,
            m_script,
        })
    }
}
