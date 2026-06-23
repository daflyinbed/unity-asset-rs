//! 自动生成。请勿手动修改。

use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct StyleSheetImporter {
    pub parent: AssetImporter,
    pub m_name: String,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for StyleSheetImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;

        Ok(StyleSheetImporter {
            parent,
            m_name,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
