//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetShaderSettings {
    pub m_build_target: String,
    pub m_chunk_size_in_mb: u32,
    pub m_max_chunks: u32,
    pub m_override_default_settings: bool,
}

impl crate::Parse for BuildTargetShaderSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_chunk_size_in_mb = rdr.read_u32()?;
        let m_max_chunks = rdr.read_u32()?;
        let m_override_default_settings = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(BuildTargetShaderSettings {
            m_build_target,
            m_chunk_size_in_mb,
            m_max_chunks,
            m_override_default_settings,
        })
    }
}
