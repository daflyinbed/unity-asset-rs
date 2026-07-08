//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ChildAnimatorState {
    pub m_state: PPtr,
    pub m_position: Vector3f,
}

impl crate::Parse for ChildAnimatorState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state = crate::PPtr::parse(rdr, ver)?;
        let m_position = Vector3f::parse(rdr, ver)?;

        Ok(ChildAnimatorState {
            m_state,
            m_position,
        })
    }
}
