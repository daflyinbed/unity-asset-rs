//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetDefaultTextureCompressionFormat {
    pub m_build_target: String,
    pub m_format: i32,
}

impl crate::Parse for BuildTargetDefaultTextureCompressionFormat {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_format = rdr.read_i32()?;

        Ok(BuildTargetDefaultTextureCompressionFormat {
            m_build_target,
            m_format,
        })
    }
}
