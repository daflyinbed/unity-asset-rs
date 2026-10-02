//! 自动生成。请勿手动修改。

/// 版本枚举：StreamingInfoOffsetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StreamingInfoOffsetVariant {
    Default(u32),
    V2020_1(u64),
}

impl crate::Parse for StreamingInfoOffsetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u64()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for StreamingInfoOffsetVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StreamingInfo {
    pub offset: StreamingInfoOffsetVariant,
    pub size: u32,
    pub path: String,
}

impl crate::Parse for StreamingInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let offset = StreamingInfoOffsetVariant::parse(rdr, ver)?;
        let size = rdr.read_u32()?;
        let path = rdr.read_aligned_string()?;

        Ok(StreamingInfo { offset, size, path })
    }
}

impl crate::ToValue for StreamingInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("offset".to_string(), self.offset.to_value())),
            Some(("size".to_string(), self.size.to_value())),
            Some(("path".to_string(), self.path.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "offset" => (self.offset).inspect_value(path, offset, limit),
                "size" => (self.size).inspect_value(path, offset, limit),
                "path" => (self.path).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.offset).inspect_value(path, offset, limit)).map(|v| ("offset".to_string(), v)),
            ((self.size).inspect_value(path, offset, limit)).map(|v| ("size".to_string(), v)),
            ((self.path).inspect_value(path, offset, limit)).map(|v| ("path".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
