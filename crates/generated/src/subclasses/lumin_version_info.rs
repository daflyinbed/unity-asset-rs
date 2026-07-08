//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LuminVersionInfo {
    pub m_version_code: i32,
    pub m_version_name: String,
}

impl crate::Parse for LuminVersionInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_version_code = rdr.read_i32()?;
        let m_version_name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(LuminVersionInfo {
            m_version_code,
            m_version_name,
        })
    }
}
