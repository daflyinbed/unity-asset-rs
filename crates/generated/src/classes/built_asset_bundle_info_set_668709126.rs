//! 自动生成。请勿手动修改。

use crate::subclasses::built_asset_bundle_info::BuiltAssetBundleInfo;

/// 版本枚举：BuiltAssetBundleInfoSet_668709126BundleInfosVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuiltAssetBundleInfoSet_668709126BundleInfosVariant {
    Default(Vec<BuiltAssetBundleInfo>),
    V2017_1(Vec<BuiltAssetBundleInfo>),
}

impl crate::Parse for BuiltAssetBundleInfoSet_668709126BundleInfosVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuiltAssetBundleInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuiltAssetBundleInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BuiltAssetBundleInfoSet_668709126BundleInfosVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfoSet_668709126 {
    pub bundle_infos: BuiltAssetBundleInfoSet_668709126BundleInfosVariant,
}

impl crate::Parse for BuiltAssetBundleInfoSet_668709126 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bundle_infos = BuiltAssetBundleInfoSet_668709126BundleInfosVariant::parse(rdr, ver)?;

        Ok(BuiltAssetBundleInfoSet_668709126 { bundle_infos })
    }
}

impl crate::ToValue for BuiltAssetBundleInfoSet_668709126 {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "bundleInfos".to_string(),
            self.bundle_infos.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "bundleInfos" => (self.bundle_infos).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.bundle_infos).inspect_value(path, offset, limit))
                .map(|v| ("bundleInfos".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
