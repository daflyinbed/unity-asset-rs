//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct bitset {
    pub bit_count: i32,
    pub bitblocks: Vec<u8>,
}

impl crate::Parse for bitset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bit_count = rdr.read_i32()?;
        let bitblocks = ({
            let n = rdr.read_i32()? as usize;
            rdr.read_bytes(n)
        })?;
        rdr.align(4)?;

        Ok(bitset {
            bit_count,
            bitblocks,
        })
    }
}

impl crate::ToValue for bitset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("bitCount".to_string(), self.bit_count.to_value())),
            Some((
                "bitblocks".to_string(),
                crate::Value::Bytes(self.bitblocks.clone()),
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
                "bitCount" => (self.bit_count).inspect_value(path, offset, limit),
                "bitblocks" => crate::inspect_bytes(&(self.bitblocks), path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.bit_count).inspect_value(path, offset, limit))
                .map(|v| ("bitCount".to_string(), v)),
            (crate::inspect_bytes(&(self.bitblocks), path, offset, limit))
                .map(|v| ("bitblocks".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
