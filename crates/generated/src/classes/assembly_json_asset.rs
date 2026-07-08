//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssemblyJsonAsset {
    pub m_name: String,
    pub m_script: String,
}

impl crate::Parse for AssemblyJsonAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_script = r.read_aligned_string()?;
        r.align(4)?;

        Ok(AssemblyJsonAsset {
            m_name,
            m_script,
        })
    }
}
