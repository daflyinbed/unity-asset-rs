//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LeafInfoConstant {
    pub m_id_array: Vec<u32>,
    pub m_index_offset: u32,
}

impl crate::Parse for LeafInfoConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_id_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_index_offset = rdr.read_u32()?;

        Ok(LeafInfoConstant {
            m_id_array,
            m_index_offset,
        })
    }
}

impl crate::ToValue for LeafInfoConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_IDArray".to_string(), self.m_id_array.to_value())),
            Some(("m_IndexOffset".to_string(), self.m_index_offset.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_IDArray" => (self.m_id_array).inspect_value(path, offset, limit),
                "m_IndexOffset" => (self.m_index_offset).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_id_array).inspect_value(path, offset, limit))
                .map(|v| ("m_IDArray".to_string(), v)),
            ((self.m_index_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_IndexOffset".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
