//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BlockShaderSyntaxTree {
    pub m_name: String,
    pub file_path: String,
    pub source: String,
}

impl crate::Parse for BlockShaderSyntaxTree {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let file_path = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let source = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(BlockShaderSyntaxTree {
            m_name,
            file_path,
            source,
        })
    }
}
