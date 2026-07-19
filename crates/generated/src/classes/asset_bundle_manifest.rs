//! 自动生成。请勿手动修改。

use crate::subclasses::asset_bundle_info::AssetBundleInfo;

#[derive(Debug, Clone)]
pub struct AssetBundleManifest {
    pub m_name: String,
    pub asset_bundle_names: Vec<(i32, String)>,
    pub asset_bundles_with_variant: Vec<i32>,
    pub asset_bundle_infos: Vec<(i32, AssetBundleInfo)>,
}

impl crate::Parse for AssetBundleManifest {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let asset_bundle_names = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_i32()?;
                        let b = rdr.read_aligned_string()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let asset_bundles_with_variant = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let asset_bundle_infos = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_i32()?;
                        let b = AssetBundleInfo::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(AssetBundleManifest {
            m_name,
            asset_bundle_names,
            asset_bundles_with_variant,
            asset_bundle_infos,
        })
    }
}
