//! 自动生成。请勿手动修改。

use crate::subclasses::asset::Asset;
use crate::subclasses::asset_bundle_full_name::AssetBundleFullName;
use crate::subclasses::asset_database_metrics::AssetDatabaseMetrics;
use crate::subclasses::asset_importer_hash_key::AssetImporterHashKey;
use crate::subclasses::asset_time_stamp::AssetTimeStamp;
use crate::subclasses::guid::Guid;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct AssetDatabase {
    pub parent: Object,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_assets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = Asset::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_asset_time_stamps = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = AssetTimeStamp::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_unity_shaders_version = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_last_valid_version_hashes = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_u32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_last_valid_versions = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = AssetImporterHashKey::parse(r, ver)?; let b = r.read_u32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_names = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = AssetBundleFullName::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_metrics = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(AssetDatabaseMetrics::parse(r, ver)?)
        } else {
            None
        };

        Ok(AssetDatabase {
            parent,
            m_assets,
            m_asset_time_stamps,
            m_unity_shaders_version,
            m_last_valid_version_hashes,
            m_last_valid_versions,
            m_asset_bundle_names,
            m_metrics,
        })
    }
}
