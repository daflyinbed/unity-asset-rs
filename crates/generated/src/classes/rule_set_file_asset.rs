//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RuleSetFileAsset {
    pub m_name: String,
    pub m_script: String,
}

impl crate::Parse for RuleSetFileAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_script = rdr.read_aligned_string()?;

        Ok(RuleSetFileAsset { m_name, m_script })
    }
}
