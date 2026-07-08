//! 自动生成。请勿手动修改。

use crate::subclasses::build_report_scenes_using_asset::BuildReportScenesUsingAsset;

#[derive(Debug, Clone)]
pub struct ScenesUsingAssets {
    pub m_scenes_using_assets: Vec<BuildReportScenesUsingAsset>,
    pub m_list_of_scenes_using_each_asset: Vec<(String, Vec<String>)>,
}

impl crate::Parse for ScenesUsingAssets {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_scenes_using_assets = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuildReportScenesUsingAsset::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_list_of_scenes_using_each_asset = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(ScenesUsingAssets {
            m_scenes_using_assets,
            m_list_of_scenes_using_each_asset,
        })
    }
}
