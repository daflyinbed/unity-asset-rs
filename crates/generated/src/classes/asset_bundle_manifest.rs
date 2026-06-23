//! 自动生成。请勿手动修改。

use crate::subclasses::asset_bundle_info::AssetBundleInfo;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct AssetBundleManifest {
    pub parent: NamedObject,
    pub m_name: String,
    pub asset_bundle_names: Vec<(i32, String)>,
    pub asset_bundles_with_variant: Vec<i32>,
    pub asset_bundle_infos: Vec<(i32, AssetBundleInfo)>,
}

impl crate::Parse for AssetBundleManifest {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let asset_bundle_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let asset_bundles_with_variant = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let asset_bundle_infos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = AssetBundleInfo::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(AssetBundleManifest {
            parent,
            m_name,
            asset_bundle_names,
            asset_bundles_with_variant,
            asset_bundle_infos,
        })
    }
}
