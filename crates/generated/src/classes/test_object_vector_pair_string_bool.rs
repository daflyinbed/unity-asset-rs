//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TestObjectVectorPairStringBool {
    pub m_map: Vec<(String, bool)>,
    pub m_string: String,
}

impl crate::Parse for TestObjectVectorPairStringBool {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_map = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_aligned_string()?;
                        let b = rdr.read_bool()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_string = rdr.read_aligned_string()?;

        Ok(TestObjectVectorPairStringBool { m_map, m_string })
    }
}

impl crate::ToValue for TestObjectVectorPairStringBool {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Map".to_string(), self.m_map.to_value())),
            Some(("m_String".to_string(), self.m_string.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Map" => (self.m_map).inspect_value(path, offset, limit),
                "m_String" => (self.m_string).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_map).inspect_value(path, offset, limit)).map(|v| ("m_Map".to_string(), v)),
            ((self.m_string).inspect_value(path, offset, limit))
                .map(|v| ("m_String".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
