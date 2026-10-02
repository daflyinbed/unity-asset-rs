//! 自动生成。请勿手动修改。

/// 版本枚举：AttachmentInfoNeedsResolveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AttachmentInfoNeedsResolveVariant {
    Default(bool),
    V6000_2(bool),
}

impl crate::Parse for AttachmentInfoNeedsResolveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AttachmentInfoNeedsResolveVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AttachmentInfo {
    pub format: i32,
    pub needs_resolve: AttachmentInfoNeedsResolveVariant,
    pub can_multiview: Option<bool>,
    pub load_action: Option<u8>,
    pub store_action: Option<u8>,
    pub sample_count: Option<u8>,
}

impl crate::Parse for AttachmentInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let format = rdr.read_i32()?;
        let needs_resolve = AttachmentInfoNeedsResolveVariant::parse(rdr, ver)?;
        let can_multiview = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };
        let load_action = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(rdr.read_u8()?)
        } else {
            None
        };
        let store_action = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(rdr.read_u8()?)
        } else {
            None
        };
        let sample_count = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AttachmentInfo {
            format,
            needs_resolve,
            can_multiview,
            load_action,
            store_action,
            sample_count,
        })
    }
}

impl crate::ToValue for AttachmentInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("format".to_string(), self.format.to_value())),
            Some(("needsResolve".to_string(), self.needs_resolve.to_value())),
            crate::opt_field("canMultiview", &self.can_multiview),
            crate::opt_field("loadAction", &self.load_action),
            crate::opt_field("storeAction", &self.store_action),
            crate::opt_field("sampleCount", &self.sample_count),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "format" => (self.format).inspect_value(path, offset, limit),
                "needsResolve" => (self.needs_resolve).inspect_value(path, offset, limit),
                "canMultiview" => (self.can_multiview).inspect_value(path, offset, limit),
                "loadAction" => (self.load_action).inspect_value(path, offset, limit),
                "storeAction" => (self.store_action).inspect_value(path, offset, limit),
                "sampleCount" => (self.sample_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.format).inspect_value(path, offset, limit)).map(|v| ("format".to_string(), v)),
            ((self.needs_resolve).inspect_value(path, offset, limit))
                .map(|v| ("needsResolve".to_string(), v)),
            ((self.can_multiview).inspect_value(path, offset, limit))
                .map(|v| ("canMultiview".to_string(), v)),
            ((self.load_action).inspect_value(path, offset, limit))
                .map(|v| ("loadAction".to_string(), v)),
            ((self.store_action).inspect_value(path, offset, limit))
                .map(|v| ("storeAction".to_string(), v)),
            ((self.sample_count).inspect_value(path, offset, limit))
                .map(|v| ("sampleCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
