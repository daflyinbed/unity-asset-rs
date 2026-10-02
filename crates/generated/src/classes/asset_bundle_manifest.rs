//! 自动生成。请勿手动修改。

use crate::subclasses::asset_bundle_info::AssetBundleInfo;

#[derive(Debug, Clone)]
pub struct AssetBundleManifest {
    pub m_name: String,
    pub asset_bundle_names: Vec<(i32, String)>,
    pub asset_bundles_with_variant: Vec<i32>,
    pub asset_bundle_infos: Vec<(i32, AssetBundleInfo)>,
}

impl crate::Parse for AssetBundleManifest {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let asset_bundle_names = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_i32()?;
                        let b = rdr.read_aligned_string()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let asset_bundles_with_variant = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let asset_bundle_infos = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_i32()?;
                        let b = AssetBundleInfo::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(AssetBundleManifest {
            m_name,
            asset_bundle_names,
            asset_bundles_with_variant,
            asset_bundle_infos,
        })
    }
}

impl crate::ToValue for AssetBundleManifest {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "AssetBundleNames".to_string(),
                self.asset_bundle_names.to_value(),
            )),
            Some((
                "AssetBundlesWithVariant".to_string(),
                self.asset_bundles_with_variant.to_value(),
            )),
            Some((
                "AssetBundleInfos".to_string(),
                self.asset_bundle_infos.to_value(),
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
                "AssetBundleNames" => (self.asset_bundle_names).inspect_value(path, offset, limit),
                "AssetBundlesWithVariant" => {
                    (self.asset_bundles_with_variant).inspect_value(path, offset, limit)
                }
                "AssetBundleInfos" => (self.asset_bundle_infos).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.asset_bundle_names).inspect_value(path, offset, limit))
                .map(|v| ("AssetBundleNames".to_string(), v)),
            ((self.asset_bundles_with_variant).inspect_value(path, offset, limit))
                .map(|v| ("AssetBundlesWithVariant".to_string(), v)),
            ((self.asset_bundle_infos).inspect_value(path, offset, limit))
                .map(|v| ("AssetBundleInfos".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
