//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_SInt32 {
    pub m_expression_index: u32,
    pub m_value: i32,
}

impl crate::Parse for VfxEntryExpressionValue_SInt32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = rdr.read_i32()?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_SInt32 {
            m_expression_index,
            m_value,
        })
    }
}

impl crate::ToValue for VfxEntryExpressionValue_SInt32 {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_ExpressionIndex".to_string(),
                self.m_expression_index.to_value(),
            )),
            Some(("m_Value".to_string(), self.m_value.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ExpressionIndex" => (self.m_expression_index).inspect_value(path, offset, limit),
                "m_Value" => (self.m_value).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_expression_index).inspect_value(path, offset, limit))
                .map(|v| ("m_ExpressionIndex".to_string(), v)),
            ((self.m_value).inspect_value(path, offset, limit)).map(|v| ("m_Value".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
