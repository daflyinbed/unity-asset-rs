//! 自动生成。请勿手动修改。

use crate::subclasses::built_asset_bundle_info::BuiltAssetBundleInfo;

#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfoSet_668709126 {
    pub bundle_infos: Vec<BuiltAssetBundleInfo>,
}

impl crate::Parse for BuiltAssetBundleInfoSet_668709126 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bundle_infos = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuiltAssetBundleInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(BuiltAssetBundleInfoSet_668709126 {
            bundle_infos,
        })
    }
}
