//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ChildAnimatorState {
    pub m_state: PPtr,
    pub m_position: Vector3f,
}

impl crate::Parse for ChildAnimatorState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state = crate::PPtr::parse(r, ver)?;
        let m_position = Vector3f::parse(r, ver)?;

        Ok(ChildAnimatorState {
            m_state,
            m_position,
        })
    }
}
