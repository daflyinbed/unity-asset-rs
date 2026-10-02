//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub stream: u8,
    pub offset: u8,
    pub format: u8,
    pub dimension: u8,
}

impl crate::Parse for ChannelInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let stream = rdr.read_u8()?;
        let offset = rdr.read_u8()?;
        let format = rdr.read_u8()?;
        let dimension = rdr.read_u8()?;

        Ok(ChannelInfo {
            stream,
            offset,
            format,
            dimension,
        })
    }
}

impl crate::ToValue for ChannelInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("stream".to_string(), self.stream.to_value())),
            Some(("offset".to_string(), self.offset.to_value())),
            Some(("format".to_string(), self.format.to_value())),
            Some(("dimension".to_string(), self.dimension.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "stream" => (self.stream).inspect_value(path, offset, limit),
                "offset" => (self.offset).inspect_value(path, offset, limit),
                "format" => (self.format).inspect_value(path, offset, limit),
                "dimension" => (self.dimension).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.stream).inspect_value(path, offset, limit)).map(|v| ("stream".to_string(), v)),
            ((self.offset).inspect_value(path, offset, limit)).map(|v| ("offset".to_string(), v)),
            ((self.format).inspect_value(path, offset, limit)).map(|v| ("format".to_string(), v)),
            ((self.dimension).inspect_value(path, offset, limit))
                .map(|v| ("dimension".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
