//! 自动生成。请勿手动修改。

use crate::subclasses::build_report_scenes_using_asset::BuildReportScenesUsingAsset;

#[derive(Debug, Clone)]
pub struct ScenesUsingAssets {
    pub m_scenes_using_assets: Vec<BuildReportScenesUsingAsset>,
    pub m_list_of_scenes_using_each_asset: Vec<(String, Vec<String>)>,
}

impl crate::Parse for ScenesUsingAssets {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_scenes_using_assets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildReportScenesUsingAsset::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_list_of_scenes_using_each_asset = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ScenesUsingAssets {
            m_scenes_using_assets,
            m_list_of_scenes_using_each_asset,
        })
    }
}
