//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackageManifest {
    pub m_name: String,
    pub m_script: String,
}

impl crate::Parse for PackageManifest {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_script = rdr.read_aligned_string()?;

        Ok(PackageManifest {
            m_name,
            m_script,
        })
    }
}
