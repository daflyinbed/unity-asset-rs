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
        let m_state_machine_behaviours = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(StateBehavioursPair {
            m_state,
            m_state_machine_behaviours,
        })
    }
}

impl crate::ToValue for StateBehavioursPair {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_State".to_string(), self.m_state.to_value())),
            Some((
                "m_StateMachineBehaviours".to_string(),
                self.m_state_machine_behaviours.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_State" => (self.m_state).inspect_value(path, offset, limit),
                "m_StateMachineBehaviours" => {
                    (self.m_state_machine_behaviours).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_state).inspect_value(path, offset, limit)).map(|v| ("m_State".to_string(), v)),
            ((self.m_state_machine_behaviours).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineBehaviours".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
