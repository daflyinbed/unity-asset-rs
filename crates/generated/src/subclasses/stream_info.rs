//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub channel_mask: u32,
    pub offset: u32,
    pub stride: u32,
    pub align: u32,
}

impl crate::Parse for StreamInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let channel_mask = r.read_u32()?;
        let offset = r.read_u32()?;
        let stride = r.read_u32()?;
        let align = r.read_u32()?;

        Ok(StreamInfo {
            channel_mask,
            offset,
            stride,
            align,
        })
    }
}
