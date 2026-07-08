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

        Ok(StreamingInfo {
            offset,
            size,
            path,
        })
    }
}
