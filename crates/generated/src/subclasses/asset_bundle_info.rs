//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct AssetBundleInfo {
    pub asset_bundle_hash: Hash128,
    pub asset_bundle_dependencies: Vec<i32>,
}

impl crate::Parse for AssetBundleInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let asset_bundle_hash = Hash128::parse(rdr, ver)?;
        let asset_bundle_dependencies = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(AssetBundleInfo {
            asset_bundle_hash,
            asset_bundle_dependencies,
        })
    }
}

impl crate::ToValue for AssetBundleInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "AssetBundleHash".to_string(),
                self.asset_bundle_hash.to_value(),
            )),
            Some((
                "AssetBundleDependencies".to_string(),
                self.asset_bundle_dependencies.to_value(),
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
                "AssetBundleHash" => (self.asset_bundle_hash).inspect_value(path, offset, limit),
                "AssetBundleDependencies" => {
                    (self.asset_bundle_dependencies).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.asset_bundle_hash).inspect_value(path, offset, limit))
                .map(|v| ("AssetBundleHash".to_string(), v)),
            ((self.asset_bundle_dependencies).inspect_value(path, offset, limit))
                .map(|v| ("AssetBundleDependencies".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
