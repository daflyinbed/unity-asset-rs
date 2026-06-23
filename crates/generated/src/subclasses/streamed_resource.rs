//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StreamedResource {
    pub m_source: String,
    pub m_offset: u64,
    pub m_size: u64,
}

impl crate::Parse for StreamedResource {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_source = r.read_aligned_string()?;
        r.align(4)?;
        let m_offset = r.read_u64()?;
        let m_size = r.read_u64()?;

        Ok(StreamedResource {
            m_source,
            m_offset,
            m_size,
        })
    }
}
