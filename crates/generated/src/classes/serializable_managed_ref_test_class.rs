//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SerializableManagedRefTestClass {
    pub m_script: PPtr,
}

impl crate::Parse for SerializableManagedRefTestClass {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_script = crate::PPtr::parse(rdr, ver)?;

        Ok(SerializableManagedRefTestClass { m_script })
    }
}
