//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VectorParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_offset_in_constant_buffer: Option<i32>,
    pub m_array_size: i32,
    pub m_type: i8,
    pub m_dim: i8,
}

impl crate::Parse for VectorParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_offset_in_constant_buffer = None;
        let mut m_array_size = None;
        let mut m_type = None;
        let mut m_dim = None;
        m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_offset_in_constant_buffer = Some(rdr.read_i32()?);
        } else {
            m_index = Some(rdr.read_i32()?);
        }
        m_array_size = Some(rdr.read_i32()?);
        m_type = Some(rdr.read_i8()?);
        m_dim = Some(rdr.read_i8()?);
        rdr.align(4)?;

        Ok(VectorParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_offset_in_constant_buffer,
            m_array_size: m_array_size.unwrap(),
            m_type: m_type.unwrap(),
            m_dim: m_dim.unwrap(),
        })
    }
}

impl crate::ToValue for VectorParameter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_NameIndex".to_string(), self.m_name_index.to_value())),
            crate::opt_field("m_Index", &self.m_index),
            crate::opt_field(
                "m_OffsetInConstantBuffer",
                &self.m_offset_in_constant_buffer,
            ),
            Some(("m_ArraySize".to_string(), self.m_array_size.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
            Some(("m_Dim".to_string(), self.m_dim.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_NameIndex" => (self.m_name_index).inspect_value(path, offset, limit),
                "m_Index" => (self.m_index).inspect_value(path, offset, limit),
                "m_OffsetInConstantBuffer" => {
                    (self.m_offset_in_constant_buffer).inspect_value(path, offset, limit)
                }
                "m_ArraySize" => (self.m_array_size).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_Dim" => (self.m_dim).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name_index).inspect_value(path, offset, limit))
                .map(|v| ("m_NameIndex".to_string(), v)),
            ((self.m_index).inspect_value(path, offset, limit)).map(|v| ("m_Index".to_string(), v)),
            ((self.m_offset_in_constant_buffer).inspect_value(path, offset, limit))
                .map(|v| ("m_OffsetInConstantBuffer".to_string(), v)),
            ((self.m_array_size).inspect_value(path, offset, limit))
                .map(|v| ("m_ArraySize".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_dim).inspect_value(path, offset, limit)).map(|v| ("m_Dim".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
