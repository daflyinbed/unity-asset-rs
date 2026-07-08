//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BlockShaderSourceArtifact {
    pub shader_name: String,
    pub shader_source: String,
}

impl crate::Parse for BlockShaderSourceArtifact {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let shader_name = rdr.read_aligned_string()?;
        let shader_source = rdr.read_aligned_string()?;

        Ok(BlockShaderSourceArtifact {
            shader_name,
            shader_source,
        })
    }
}
