//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TestObjectWithSerializedArray {
    pub m_integer_array: Vec<i32>,
    pub m_clamp_test_value: f32,
}

impl crate::Parse for TestObjectWithSerializedArray {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_integer_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_clamp_test_value = rdr.read_f32()?;

        Ok(TestObjectWithSerializedArray {
            m_integer_array,
            m_clamp_test_value,
        })
    }
}

impl crate::ToValue for TestObjectWithSerializedArray {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_IntegerArray".to_string(),
                self.m_integer_array.to_value(),
            )),
            Some((
                "m_ClampTestValue".to_string(),
                self.m_clamp_test_value.to_value(),
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
                "m_IntegerArray" => (self.m_integer_array).inspect_value(path, offset, limit),
                "m_ClampTestValue" => (self.m_clamp_test_value).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_integer_array).inspect_value(path, offset, limit))
                .map(|v| ("m_IntegerArray".to_string(), v)),
            ((self.m_clamp_test_value).inspect_value(path, offset, limit))
                .map(|v| ("m_ClampTestValue".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
