//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BlockShaderSourceArtifact {
    pub m_object_hide_flags: u32,
    pub shader_name: String,
    pub shader_source: String,
}

impl crate::Parse for BlockShaderSourceArtifact {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let shader_name = r.read_aligned_string()?;
        r.align(4)?;
        let shader_source = r.read_aligned_string()?;
        r.align(4)?;

        Ok(BlockShaderSourceArtifact {
            m_object_hide_flags,
            shader_name,
            shader_source,
        })
    }
}
