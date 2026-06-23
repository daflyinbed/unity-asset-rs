//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildReportScenesUsingAsset {
    pub asset_path: String,
    pub scene_paths: Vec<String>,
}

impl crate::Parse for BuildReportScenesUsingAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let asset_path = r.read_aligned_string()?;
        r.align(4)?;
        let scene_paths = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(BuildReportScenesUsingAsset {
            asset_path,
            scene_paths,
        })
    }
}
