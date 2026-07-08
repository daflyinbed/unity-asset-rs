//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ChildAnimatorStateMachine {
    pub m_state_machine: PPtr,
    pub m_position: Vector3f,
}

impl crate::Parse for ChildAnimatorStateMachine {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state_machine = crate::PPtr::parse(rdr, ver)?;
        let m_position = Vector3f::parse(rdr, ver)?;

        Ok(ChildAnimatorStateMachine {
            m_state_machine,
            m_position,
        })
    }
}
