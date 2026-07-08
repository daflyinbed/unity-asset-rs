//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LuminSigningCert {
    pub m_cert_path: String,
    pub m_private_key_path: String,
}

impl crate::Parse for LuminSigningCert {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_cert_path = rdr.read_aligned_string()?;
        let m_private_key_path = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(LuminSigningCert {
            m_cert_path,
            m_private_key_path,
        })
    }
}
