//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Blend1dDataConstant {
    pub m_child_threshold_array: Vec<f32>,
}

impl crate::Parse for Blend1dDataConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_child_threshold_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(Blend1dDataConstant {
            m_child_threshold_array,
        })
    }
}

impl crate::ToValue for Blend1dDataConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_ChildThresholdArray".to_string(),
            self.m_child_threshold_array.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ChildThresholdArray" => {
                    (self.m_child_threshold_array).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_child_threshold_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ChildThresholdArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
