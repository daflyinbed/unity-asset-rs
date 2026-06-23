//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct StateMotionPair {
    pub m_state: PPtr,
    pub m_motion: PPtr,
}

impl crate::Parse for StateMotionPair {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state = crate::PPtr::parse(r, ver)?;
        let m_motion = crate::PPtr::parse(r, ver)?;

        Ok(StateMotionPair {
            m_state,
            m_motion,
        })
    }
}
