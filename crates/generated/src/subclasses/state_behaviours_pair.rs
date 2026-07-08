//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct StateBehavioursPair {
    pub m_state: PPtr,
    pub m_state_machine_behaviours: Vec<PPtr>,
}

impl crate::Parse for StateBehavioursPair {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state = crate::PPtr::parse(rdr, ver)?;
        let m_state_machine_behaviours = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(StateBehavioursPair {
            m_state,
            m_state_machine_behaviours,
        })
    }
}
