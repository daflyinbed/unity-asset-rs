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
