//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedTagMap {
    pub tags: Vec<(String, String)>,
}

impl crate::Parse for SerializedTagMap {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let tags = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_aligned_string()?;
                        let b = rdr.read_aligned_string()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SerializedTagMap { tags })
    }
}

impl crate::ToValue for SerializedTagMap {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("tags".to_string(), self.tags.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "tags" => (self.tags).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> =
            vec![((self.tags).inspect_value(path, offset, limit)).map(|v| ("tags".to_string(), v))]
                .into_iter()
                .flatten()
                .collect();
        Some(crate::Value::Struct(fields))
    }
}
