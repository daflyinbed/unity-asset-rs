//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetDatabaseMetrics {
    pub total_asset_count: i32,
    pub non_pro_asset_count: i32,
    pub non_pro_assets_created_after_pro_license: i32,
}

impl crate::Parse for AssetDatabaseMetrics {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let total_asset_count = r.read_i32()?;
        let non_pro_asset_count = r.read_i32()?;
        let non_pro_assets_created_after_pro_license = r.read_i32()?;

        Ok(AssetDatabaseMetrics {
            total_asset_count,
            non_pro_asset_count,
            non_pro_assets_created_after_pro_license,
        })
    }
}
