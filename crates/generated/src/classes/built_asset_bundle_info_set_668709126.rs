//! 自动生成。请勿手动修改。

use crate::subclasses::built_asset_bundle_info::BuiltAssetBundleInfo;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfoSet_668709126 {
    pub parent: Object,
    pub bundle_infos: Vec<BuiltAssetBundleInfo>,
}

impl crate::Parse for BuiltAssetBundleInfoSet_668709126 {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let bundle_infos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuiltAssetBundleInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(BuiltAssetBundleInfoSet_668709126 {
            parent,
            bundle_infos,
        })
    }
}
