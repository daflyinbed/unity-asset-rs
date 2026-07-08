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
        let asset_bundle_dependencies = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(AssetBundleInfo {
            asset_bundle_hash,
            asset_bundle_dependencies,
        })
    }
}
