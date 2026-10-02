//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct StateMotionPair {
    pub m_state: PPtr,
    pub m_motion: PPtr,
}

impl crate::Parse for StateMotionPair {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_state = crate::PPtr::parse(rdr, ver)?;
        let m_motion = crate::PPtr::parse(rdr, ver)?;

        Ok(StateMotionPair { m_state, m_motion })
    }
}

impl crate::ToValue for StateMotionPair {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_State".to_string(), self.m_state.to_value())),
            Some(("m_Motion".to_string(), self.m_motion.to_value())),
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
                "m_Motion" => (self.m_motion).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_state).inspect_value(path, offset, limit)).map(|v| ("m_State".to_string(), v)),
            ((self.m_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_Motion".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
