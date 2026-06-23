//! 自动生成。请勿手动修改。

use crate::subclasses::built_asset_bundle_info::BuiltAssetBundleInfo;

#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfoSet {
    pub bundle_infos: Vec<BuiltAssetBundleInfo>,
}

impl crate::Parse for BuiltAssetBundleInfoSet {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bundle_infos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuiltAssetBundleInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(BuiltAssetBundleInfoSet {
            bundle_infos,
        })
    }
}
