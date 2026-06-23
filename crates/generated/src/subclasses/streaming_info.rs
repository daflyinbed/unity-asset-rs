//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StreamingInfo {
    pub offset: u32,
    pub size: u32,
    pub path: String,
}

impl crate::Parse for StreamingInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let offset = r.read_u32()?;
        let size = r.read_u32()?;
        let path = r.read_aligned_string()?;
        r.align(4)?;

        Ok(StreamingInfo {
            offset,
            size,
            path,
        })
    }
}
