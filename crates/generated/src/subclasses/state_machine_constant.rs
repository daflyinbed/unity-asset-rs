//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct StateMachineConstant {
    pub m_state_constant_array: Vec<OffsetPtr>,
    pub m_any_state_transition_constant_array: Vec<OffsetPtr>,
    pub m_default_state: u32,
    pub m_selector_state_constant_array: Option<Vec<OffsetPtr>>,
    pub m_synchronized_layer_count: u32,
    pub m_evaluate_transitions_on_start: Option<bool>,
}

impl crate::Parse for StateMachineConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_state_constant_array = None;
        let mut m_any_state_transition_constant_array = None;
        let mut m_default_state = None;
        let mut m_selector_state_constant_array = None;
        let mut m_synchronized_layer_count = None;
        let mut m_evaluate_transitions_on_start = None;
        m_state_constant_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| OffsetPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_any_state_transition_constant_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| OffsetPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_selector_state_constant_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| OffsetPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }
        m_default_state = Some(rdr.read_u32()?);
        m_synchronized_layer_count = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_evaluate_transitions_on_start = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(StateMachineConstant {
            m_state_constant_array: m_state_constant_array.unwrap(),
            m_any_state_transition_constant_array: m_any_state_transition_constant_array.unwrap(),
            m_default_state: m_default_state.unwrap(),
            m_selector_state_constant_array,
            m_synchronized_layer_count: m_synchronized_layer_count.unwrap(),
            m_evaluate_transitions_on_start,
        })
    }
}

impl crate::ToValue for StateMachineConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_StateConstantArray".to_string(),
                self.m_state_constant_array.to_value(),
            )),
            Some((
                "m_AnyStateTransitionConstantArray".to_string(),
                self.m_any_state_transition_constant_array.to_value(),
            )),
            Some((
                "m_DefaultState".to_string(),
                self.m_default_state.to_value(),
            )),
            crate::opt_field(
                "m_SelectorStateConstantArray",
                &self.m_selector_state_constant_array,
            ),
            Some((
                "m_SynchronizedLayerCount".to_string(),
                self.m_synchronized_layer_count.to_value(),
            )),
            crate::opt_field(
                "m_EvaluateTransitionsOnStart",
                &self.m_evaluate_transitions_on_start,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_StateConstantArray" => {
                    (self.m_state_constant_array).inspect_value(path, offset, limit)
                }
                "m_AnyStateTransitionConstantArray" => {
                    (self.m_any_state_transition_constant_array).inspect_value(path, offset, limit)
                }
                "m_DefaultState" => (self.m_default_state).inspect_value(path, offset, limit),
                "m_SelectorStateConstantArray" => {
                    (self.m_selector_state_constant_array).inspect_value(path, offset, limit)
                }
                "m_SynchronizedLayerCount" => {
                    (self.m_synchronized_layer_count).inspect_value(path, offset, limit)
                }
                "m_EvaluateTransitionsOnStart" => {
                    (self.m_evaluate_transitions_on_start).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_state_constant_array).inspect_value(path, offset, limit))
                .map(|v| ("m_StateConstantArray".to_string(), v)),
            ((self.m_any_state_transition_constant_array).inspect_value(path, offset, limit))
                .map(|v| ("m_AnyStateTransitionConstantArray".to_string(), v)),
            ((self.m_default_state).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultState".to_string(), v)),
            ((self.m_selector_state_constant_array).inspect_value(path, offset, limit))
                .map(|v| ("m_SelectorStateConstantArray".to_string(), v)),
            ((self.m_synchronized_layer_count).inspect_value(path, offset, limit))
                .map(|v| ("m_SynchronizedLayerCount".to_string(), v)),
            ((self.m_evaluate_transitions_on_start).inspect_value(path, offset, limit))
                .map(|v| ("m_EvaluateTransitionsOnStart".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
