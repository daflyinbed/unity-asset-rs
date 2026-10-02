//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct SelectorTransitionConstant {
    pub m_destination: u32,
    pub m_condition_constant_array: Vec<OffsetPtr>,
}

impl crate::Parse for SelectorTransitionConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_destination = rdr.read_u32()?;
        let m_condition_constant_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| OffsetPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SelectorTransitionConstant {
            m_destination,
            m_condition_constant_array,
        })
    }
}

impl crate::ToValue for SelectorTransitionConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Destination".to_string(), self.m_destination.to_value())),
            Some((
                "m_ConditionConstantArray".to_string(),
                self.m_condition_constant_array.to_value(),
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
                "m_Destination" => (self.m_destination).inspect_value(path, offset, limit),
                "m_ConditionConstantArray" => {
                    (self.m_condition_constant_array).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_destination).inspect_value(path, offset, limit))
                .map(|v| ("m_Destination".to_string(), v)),
            ((self.m_condition_constant_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ConditionConstantArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
