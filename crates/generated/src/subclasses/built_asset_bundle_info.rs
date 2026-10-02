//! 自动生成。请勿手动修改。

/// 版本枚举：BuiltAssetBundleInfoPackagedFileIndicesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuiltAssetBundleInfoPackagedFileIndicesVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for BuiltAssetBundleInfoPackagedFileIndicesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BuiltAssetBundleInfoPackagedFileIndicesVariant {
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
pub struct BuiltAssetBundleInfo {
    pub bundle_name: String,
    pub bundle_archive_file: u32,
    pub packaged_file_indices: BuiltAssetBundleInfoPackagedFileIndicesVariant,
}

impl crate::Parse for BuiltAssetBundleInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bundle_name = rdr.read_aligned_string()?;
        let bundle_archive_file = rdr.read_u32()?;
        let packaged_file_indices =
            BuiltAssetBundleInfoPackagedFileIndicesVariant::parse(rdr, ver)?;

        Ok(BuiltAssetBundleInfo {
            bundle_name,
            bundle_archive_file,
            packaged_file_indices,
        })
    }
}

impl crate::ToValue for BuiltAssetBundleInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("bundleName".to_string(), self.bundle_name.to_value())),
            Some((
                "bundleArchiveFile".to_string(),
                self.bundle_archive_file.to_value(),
            )),
            Some((
                "packagedFileIndices".to_string(),
                self.packaged_file_indices.to_value(),
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
                "bundleName" => (self.bundle_name).inspect_value(path, offset, limit),
                "bundleArchiveFile" => {
                    (self.bundle_archive_file).inspect_value(path, offset, limit)
                }
                "packagedFileIndices" => {
                    (self.packaged_file_indices).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("bundleName".to_string(), v)),
            ((self.bundle_archive_file).inspect_value(path, offset, limit))
                .map(|v| ("bundleArchiveFile".to_string(), v)),
            ((self.packaged_file_indices).inspect_value(path, offset, limit))
                .map(|v| ("packagedFileIndices".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
