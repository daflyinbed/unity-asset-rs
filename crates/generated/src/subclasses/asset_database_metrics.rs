//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetDatabaseMetrics {
    pub total_asset_count: i32,
    pub non_pro_asset_count: Option<i32>,
    pub non_pro_assets_created_after_pro_license: Option<i32>,
}

impl crate::Parse for AssetDatabaseMetrics {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let total_asset_count = rdr.read_i32()?;
        let non_pro_asset_count = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let non_pro_assets_created_after_pro_license = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(AssetDatabaseMetrics {
            total_asset_count,
            non_pro_asset_count,
            non_pro_assets_created_after_pro_license,
        })
    }
}
