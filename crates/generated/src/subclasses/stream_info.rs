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
        let channel_mask = rdr.read_u32()?;
        let offset = rdr.read_u32()?;
        let stride = StreamInfoStrideVariant::parse(rdr, ver)?;
        let align = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let divider_op = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(rdr.read_u8()?)
        } else {
            None
        };
        let frequency = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(rdr.read_u16()?)
        } else {
            None
        };

        Ok(StreamInfo {
            channel_mask,
            offset,
            stride,
            align,
            divider_op,
            frequency,
        })
    }
}
