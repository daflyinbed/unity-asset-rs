//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct KtxImporter {
    pub m_name: String,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for KtxImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_user_data = rdr.read_aligned_string()?;
        let m_asset_bundle_name = rdr.read_aligned_string()?;
        let m_asset_bundle_variant = rdr.read_aligned_string()?;

        Ok(KtxImporter {
            m_name,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}

impl crate::ToValue for KtxImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_UserData".to_string(), self.m_user_data.to_value())),
            Some((
                "m_AssetBundleName".to_string(),
                self.m_asset_bundle_name.to_value(),
            )),
            Some((
                "m_AssetBundleVariant".to_string(),
                self.m_asset_bundle_variant.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
