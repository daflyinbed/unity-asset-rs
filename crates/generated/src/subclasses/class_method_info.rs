//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClassMethodInfo {
    pub m_class_index: i32,
    pub m_method_name: String,
    pub m_order_number: i32,
}

impl crate::Parse for ClassMethodInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_class_index = rdr.read_i32()?;
        let m_method_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_order_number = rdr.read_i32()?;
        rdr.align(4)?;

        Ok(ClassMethodInfo {
            m_class_index,
            m_method_name,
            m_order_number,
        })
    }
}

impl crate::ToValue for ClassMethodInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_ClassIndex".to_string(), self.m_class_index.to_value())),
            Some(("m_MethodName".to_string(), self.m_method_name.to_value())),
            Some(("m_OrderNumber".to_string(), self.m_order_number.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ClassIndex" => (self.m_class_index).inspect_value(path, offset, limit),
                "m_MethodName" => (self.m_method_name).inspect_value(path, offset, limit),
                "m_OrderNumber" => (self.m_order_number).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_class_index).inspect_value(path, offset, limit))
                .map(|v| ("m_ClassIndex".to_string(), v)),
            ((self.m_method_name).inspect_value(path, offset, limit))
                .map(|v| ("m_MethodName".to_string(), v)),
            ((self.m_order_number).inspect_value(path, offset, limit))
                .map(|v| ("m_OrderNumber".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
