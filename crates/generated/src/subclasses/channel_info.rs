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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let stream = r.read_u8()?;
        let offset = r.read_u8()?;
        let format = r.read_u8()?;
        let dimension = r.read_u8()?;

        Ok(ChannelInfo {
            stream,
            offset,
            format,
            dimension,
        })
    }
}
