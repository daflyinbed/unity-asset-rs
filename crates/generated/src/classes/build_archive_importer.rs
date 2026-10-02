//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct BuildArchiveImporter {
    pub m_name: String,
    pub m_external_objects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_used_file_i_ds: Vec<i64>,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for BuildArchiveImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_external_objects = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = SourceAssetIdentifier::parse(rdr, ver)?;
                        let b = crate::PPtr::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_used_file_i_ds = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i64())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_user_data = rdr.read_aligned_string()?;
        let m_asset_bundle_name = rdr.read_aligned_string()?;
        let m_asset_bundle_variant = rdr.read_aligned_string()?;

        Ok(BuildArchiveImporter {
            m_name,
            m_external_objects,
            m_used_file_i_ds,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}

impl crate::ToValue for BuildArchiveImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_ExternalObjects".to_string(),
                self.m_external_objects.to_value(),
            )),
            Some((
                "m_UsedFileIDs".to_string(),
                self.m_used_file_i_ds.to_value(),
            )),
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
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
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
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
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
