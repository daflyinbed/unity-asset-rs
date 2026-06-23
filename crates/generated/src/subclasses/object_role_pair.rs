//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ObjectRolePair {
    pub m_object: PPtr,
    pub m_roles_mask: i32,
}

impl crate::Parse for ObjectRolePair {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object = crate::PPtr::parse(r, ver)?;
        let m_roles_mask = r.read_i32()?;

        Ok(ObjectRolePair {
            m_object,
            m_roles_mask,
        })
    }
}
