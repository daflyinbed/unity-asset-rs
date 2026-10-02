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

impl crate::ToValue for AssetDatabaseMetrics {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "totalAssetCount".to_string(),
                self.total_asset_count.to_value(),
            )),
            crate::opt_field("nonProAssetCount", &self.non_pro_asset_count),
            crate::opt_field(
                "nonProAssetsCreatedAfterProLicense",
                &self.non_pro_assets_created_after_pro_license,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "totalAssetCount" => (self.total_asset_count).inspect_value(path, offset, limit),
                "nonProAssetCount" => (self.non_pro_asset_count).inspect_value(path, offset, limit),
                "nonProAssetsCreatedAfterProLicense" => (self
                    .non_pro_assets_created_after_pro_license)
                    .inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.total_asset_count).inspect_value(path, offset, limit))
                .map(|v| ("totalAssetCount".to_string(), v)),
            ((self.non_pro_asset_count).inspect_value(path, offset, limit))
                .map(|v| ("nonProAssetCount".to_string(), v)),
            ((self.non_pro_assets_created_after_pro_license).inspect_value(path, offset, limit))
                .map(|v| ("nonProAssetsCreatedAfterProLicense".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
