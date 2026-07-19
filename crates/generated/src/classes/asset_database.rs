//! 自动生成。请勿手动修改。

use crate::subclasses::asset::Asset;
use crate::subclasses::asset_bundle_full_name::AssetBundleFullName;
use crate::subclasses::asset_database_metrics::AssetDatabaseMetrics;
use crate::subclasses::asset_importer_hash_key::AssetImporterHashKey;
use crate::subclasses::asset_time_stamp::AssetTimeStamp;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AssetDatabase {
    pub m_assets: Vec<(Guid, Asset)>,
    pub m_asset_time_stamps: Option<Vec<(String, AssetTimeStamp)>>,
    pub m_unity_shaders_version: Option<i32>,
    pub m_last_valid_version_hashes: Option<Vec<(i32, u32)>>,
    pub m_last_valid_versions: Option<Vec<(AssetImporterHashKey, u32)>>,
    pub m_asset_bundle_names: Option<Vec<(i32, AssetBundleFullName)>>,
    pub m_metrics: Option<AssetDatabaseMetrics>,
}

impl crate::Parse for AssetDatabase {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_assets = None;
        let mut m_asset_time_stamps = None;
        let mut m_unity_shaders_version = None;
        let mut m_last_valid_version_hashes = None;
        let mut m_last_valid_versions = None;
        let mut m_asset_bundle_names = None;
        let mut m_metrics = None;
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_asset_time_stamps = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = AssetTimeStamp::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }
        m_assets = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = Guid::parse(rdr, ver)?;
                            let b = Asset::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_unity_shaders_version = Some(rdr.read_i32()?);
            m_last_valid_versions = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = AssetImporterHashKey::parse(rdr, ver)?;
                                let b = rdr.read_u32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_asset_bundle_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = AssetBundleFullName::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_metrics = Some(AssetDatabaseMetrics::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_unity_shaders_version = Some(rdr.read_i32()?);
            m_last_valid_version_hashes = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_u32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_asset_bundle_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = AssetBundleFullName::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_metrics = Some(AssetDatabaseMetrics::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_unity_shaders_version = Some(rdr.read_i32()?);
            m_last_valid_version_hashes = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_u32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_unity_shaders_version = Some(rdr.read_i32()?);
        } else {
        }

        Ok(AssetDatabase {
            m_assets: m_assets.unwrap(),
            m_asset_time_stamps,
            m_unity_shaders_version,
            m_last_valid_version_hashes,
            m_last_valid_versions,
            m_asset_bundle_names,
            m_metrics,
        })
    }
}
