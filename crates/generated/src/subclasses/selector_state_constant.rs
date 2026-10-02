//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct SelectorStateConstant {
    pub m_transition_constant_array: Vec<OffsetPtr>,
    pub m_full_path_id: u32,
    pub m_is_entry: bool,
}

impl crate::Parse for SelectorStateConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_transition_constant_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| OffsetPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_full_path_id = rdr.read_u32()?;
        let m_is_entry = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(SelectorStateConstant {
            m_transition_constant_array,
            m_full_path_id,
            m_is_entry,
        })
    }
}

impl crate::ToValue for SelectorStateConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_TransitionConstantArray".to_string(),
                self.m_transition_constant_array.to_value(),
            )),
            Some(("m_FullPathID".to_string(), self.m_full_path_id.to_value())),
            Some(("m_IsEntry".to_string(), self.m_is_entry.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_TransitionConstantArray" => {
                    (self.m_transition_constant_array).inspect_value(path, offset, limit)
                }
                "m_FullPathID" => (self.m_full_path_id).inspect_value(path, offset, limit),
                "m_IsEntry" => (self.m_is_entry).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_transition_constant_array).inspect_value(path, offset, limit))
                .map(|v| ("m_TransitionConstantArray".to_string(), v)),
            ((self.m_full_path_id).inspect_value(path, offset, limit))
                .map(|v| ("m_FullPathID".to_string(), v)),
            ((self.m_is_entry).inspect_value(path, offset, limit))
                .map(|v| ("m_IsEntry".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
