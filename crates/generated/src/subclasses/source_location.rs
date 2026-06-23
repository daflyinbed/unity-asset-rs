//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub m_file: String,
    pub m_position: u32,
}

impl crate::Parse for SourceLocation {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_file = r.read_aligned_string()?;
        r.align(4)?;
        let m_position = r.read_u32()?;

        Ok(SourceLocation {
            m_file,
            m_position,
        })
    }
}
