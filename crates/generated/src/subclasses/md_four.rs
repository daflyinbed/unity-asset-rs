//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MdFour {
    pub md4_hash: Vec<u8>,
}

impl crate::Parse for MdFour {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let md4_hash = ({
            let n = rdr.read_i32()? as usize;
            rdr.read_bytes(n)
        })?;
        rdr.align(4)?;

        Ok(MdFour { md4_hash })
    }
}

impl crate::ToValue for MdFour {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "md4 hash".to_string(),
            crate::Value::Bytes(self.md4_hash.clone()),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "md4 hash" => crate::inspect_bytes(&(self.md4_hash), path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            (crate::inspect_bytes(&(self.md4_hash), path, offset, limit))
                .map(|v| ("md4 hash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
