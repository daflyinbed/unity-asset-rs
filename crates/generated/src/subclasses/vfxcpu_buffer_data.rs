//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxcpuBufferData {
    pub data: Vec<u32>,
}

impl crate::Parse for VfxcpuBufferData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(VfxcpuBufferData { data })
    }
}

impl crate::ToValue for VfxcpuBufferData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("data".to_string(), self.data.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "data" => (self.data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> =
            vec![((self.data).inspect_value(path, offset, limit)).map(|v| ("data".to_string(), v))]
                .into_iter()
                .flatten()
                .collect();
        Some(crate::Value::Struct(fields))
    }
}
