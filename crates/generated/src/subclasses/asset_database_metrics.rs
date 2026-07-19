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
        let mut total_asset_count = None;
        let mut non_pro_asset_count = None;
        let mut non_pro_assets_created_after_pro_license = None;
        total_asset_count = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2017, 4, 6) {
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            non_pro_asset_count = Some(rdr.read_i32()?);
            non_pro_assets_created_after_pro_license = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 3) {
        } else {
            non_pro_asset_count = Some(rdr.read_i32()?);
            non_pro_assets_created_after_pro_license = Some(rdr.read_i32()?);
        }

        Ok(AssetDatabaseMetrics {
            total_asset_count: total_asset_count.unwrap(),
            non_pro_asset_count,
            non_pro_assets_created_after_pro_license,
        })
    }
}
