//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StyleSheetImporter {
    pub m_name: String,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for StyleSheetImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_user_data = rdr.read_aligned_string()?;
        let m_asset_bundle_name = rdr.read_aligned_string()?;
        let m_asset_bundle_variant = rdr.read_aligned_string()?;

        Ok(StyleSheetImporter {
            m_name,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
