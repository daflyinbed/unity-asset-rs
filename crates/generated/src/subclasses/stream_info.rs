//! 自动生成。请勿手动修改。

/// 版本枚举：StreamInfoStrideVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StreamInfoStrideVariant {
    Default(u32),
    V4_0(u8),
}

impl crate::Parse for StreamInfoStrideVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for StreamInfoStrideVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub channel_mask: u32,
    pub offset: u32,
    pub stride: StreamInfoStrideVariant,
    pub align: Option<u32>,
    pub divider_op: Option<u8>,
    pub frequency: Option<u16>,
}

impl crate::Parse for StreamInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut channel_mask = None;
        let mut offset = None;
        let mut stride = None;
        let mut align = None;
        let mut divider_op = None;
        let mut frequency = None;
        channel_mask = Some(rdr.read_u32()?);
        offset = Some(rdr.read_u32()?);
        stride = Some(StreamInfoStrideVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            divider_op = Some(rdr.read_u8()?);
            frequency = Some(rdr.read_u16()?);
        } else {
            align = Some(rdr.read_u32()?);
        }

        Ok(StreamInfo {
            channel_mask: channel_mask.unwrap(),
            offset: offset.unwrap(),
            stride: stride.unwrap(),
            align,
            divider_op,
            frequency,
        })
    }
}

impl crate::ToValue for StreamInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("channelMask".to_string(), self.channel_mask.to_value())),
            Some(("offset".to_string(), self.offset.to_value())),
            Some(("stride".to_string(), self.stride.to_value())),
            crate::opt_field("align", &self.align),
            crate::opt_field("dividerOp", &self.divider_op),
            crate::opt_field("frequency", &self.frequency),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "channelMask" => (self.channel_mask).inspect_value(path, offset, limit),
                "offset" => (self.offset).inspect_value(path, offset, limit),
                "stride" => (self.stride).inspect_value(path, offset, limit),
                "align" => (self.align).inspect_value(path, offset, limit),
                "dividerOp" => (self.divider_op).inspect_value(path, offset, limit),
                "frequency" => (self.frequency).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.channel_mask).inspect_value(path, offset, limit))
                .map(|v| ("channelMask".to_string(), v)),
            ((self.offset).inspect_value(path, offset, limit)).map(|v| ("offset".to_string(), v)),
            ((self.stride).inspect_value(path, offset, limit)).map(|v| ("stride".to_string(), v)),
            ((self.align).inspect_value(path, offset, limit)).map(|v| ("align".to_string(), v)),
            ((self.divider_op).inspect_value(path, offset, limit))
                .map(|v| ("dividerOp".to_string(), v)),
            ((self.frequency).inspect_value(path, offset, limit))
                .map(|v| ("frequency".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
