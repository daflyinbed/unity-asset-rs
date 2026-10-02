//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackedBitVector_Quaternionf {
    pub m_num_items: u32,
    pub m_data: Vec<u8>,
}

impl crate::Parse for PackedBitVector_Quaternionf {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_num_items = rdr.read_u32()?;
        let m_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(PackedBitVector_Quaternionf {
            m_num_items,
            m_data,
        })
    }
}

impl crate::ToValue for PackedBitVector_Quaternionf {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_NumItems".to_string(), self.m_num_items.to_value())),
            Some(("m_Data".to_string(), self.m_data.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_NumItems" => (self.m_num_items).inspect_value(path, offset, limit),
                "m_Data" => (self.m_data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_num_items).inspect_value(path, offset, limit))
                .map(|v| ("m_NumItems".to_string(), v)),
            ((self.m_data).inspect_value(path, offset, limit)).map(|v| ("m_Data".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
