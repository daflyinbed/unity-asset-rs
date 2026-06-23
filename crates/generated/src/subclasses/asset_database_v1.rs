//! 自动生成。请勿手动修改。

use crate::subclasses::asset::Asset;
use crate::subclasses::asset_bundle_full_name::AssetBundleFullName;
use crate::subclasses::asset_database_metrics::AssetDatabaseMetrics;
use crate::subclasses::asset_time_stamp::AssetTimeStamp;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AssetDatabaseV1 {
    pub m_object_hide_flags: u32,
    pub m_asset_time_stamps: Vec<(String, AssetTimeStamp)>,
    pub m_assets: Vec<(Guid, Asset)>,
    pub m_unity_shaders_version: i32,
    pub m_last_valid_version_hashes: Vec<(i32, u32)>,
    pub m_asset_bundle_names: Vec<(i32, AssetBundleFullName)>,
    pub m_metrics: AssetDatabaseMetrics,
}

impl crate::Parse for AssetDatabaseV1 {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_asset_time_stamps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = AssetTimeStamp::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_assets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = Asset::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_unity_shaders_version = r.read_i32()?;
        let m_last_valid_version_hashes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_u32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_asset_bundle_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = AssetBundleFullName::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_metrics = AssetDatabaseMetrics::parse(r, ver)?;

        Ok(AssetDatabaseV1 {
            m_object_hide_flags,
            m_asset_time_stamps,
            m_assets,
            m_unity_shaders_version,
            m_last_valid_version_hashes,
            m_asset_bundle_names,
            m_metrics,
        })
    }
}
