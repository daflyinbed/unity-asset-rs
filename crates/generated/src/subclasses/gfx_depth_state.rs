//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GfxDepthState {
    pub depth_write: u8,
    pub depth_func: i8,
}

impl crate::Parse for GfxDepthState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_write = rdr.read_u8()?;
        let depth_func = rdr.read_i8()?;
        rdr.align(4)?;

        Ok(GfxDepthState {
            depth_write,
            depth_func,
        })
    }
}

impl crate::ToValue for GfxDepthState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("depthWrite".to_string(), self.depth_write.to_value())),
            Some(("depthFunc".to_string(), self.depth_func.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "depthWrite" => (self.depth_write).inspect_value(path, offset, limit),
                "depthFunc" => (self.depth_func).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.depth_write).inspect_value(path, offset, limit))
                .map(|v| ("depthWrite".to_string(), v)),
            ((self.depth_func).inspect_value(path, offset, limit))
                .map(|v| ("depthFunc".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
