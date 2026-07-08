//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UnityPurchasingSettings {
    pub m_enabled: bool,
    pub m_test_mode: bool,
}

impl crate::Parse for UnityPurchasingSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_enabled = rdr.read_bool()?;
        let m_test_mode = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(UnityPurchasingSettings {
            m_enabled,
            m_test_mode,
        })
    }
}
