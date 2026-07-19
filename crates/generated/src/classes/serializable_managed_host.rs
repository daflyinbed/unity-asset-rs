//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SerializableManagedHost {
    pub m_script: PPtr,
}

impl crate::Parse for SerializableManagedHost {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_script = crate::PPtr::parse(rdr, ver)?;

        Ok(SerializableManagedHost { m_script })
    }
}
