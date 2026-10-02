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

impl crate::ToValue for ChildAnimatorStateMachine {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_StateMachine".to_string(),
                self.m_state_machine.to_value(),
            )),
            Some(("m_Position".to_string(), self.m_position.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_StateMachine" => (self.m_state_machine).inspect_value(path, offset, limit),
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_state_machine).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachine".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
