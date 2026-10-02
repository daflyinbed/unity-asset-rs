//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct ControllerConstant {
    pub m_layer_array: Vec<OffsetPtr>,
    pub m_state_machine_array: Vec<OffsetPtr>,
    pub m_values: OffsetPtr,
    pub m_default_values: OffsetPtr,
}

impl crate::Parse for ControllerConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_layer_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| OffsetPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_state_machine_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| OffsetPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_values = OffsetPtr::parse(rdr, ver)?;
        let m_default_values = OffsetPtr::parse(rdr, ver)?;

        Ok(ControllerConstant {
            m_layer_array,
            m_state_machine_array,
            m_values,
            m_default_values,
        })
    }
}

impl crate::ToValue for ControllerConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_LayerArray".to_string(), self.m_layer_array.to_value())),
            Some((
                "m_StateMachineArray".to_string(),
                self.m_state_machine_array.to_value(),
            )),
            Some(("m_Values".to_string(), self.m_values.to_value())),
            Some((
                "m_DefaultValues".to_string(),
                self.m_default_values.to_value(),
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
                "m_LayerArray" => (self.m_layer_array).inspect_value(path, offset, limit),
                "m_StateMachineArray" => {
                    (self.m_state_machine_array).inspect_value(path, offset, limit)
                }
                "m_Values" => (self.m_values).inspect_value(path, offset, limit),
                "m_DefaultValues" => (self.m_default_values).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_layer_array).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerArray".to_string(), v)),
            ((self.m_state_machine_array).inspect_value(path, offset, limit))
                .map(|v| ("m_StateMachineArray".to_string(), v)),
            ((self.m_values).inspect_value(path, offset, limit))
                .map(|v| ("m_Values".to_string(), v)),
            ((self.m_default_values).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultValues".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
