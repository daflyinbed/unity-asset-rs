//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfo {
    pub bundle_name: String,
    pub bundle_archive_file: u32,
    pub packaged_file_indices: Vec<u32>,
}

impl crate::Parse for BuiltAssetBundleInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let bundle_archive_file = r.read_u32()?;
        let packaged_file_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(BuiltAssetBundleInfo {
            bundle_name,
            bundle_archive_file,
            packaged_file_indices,
        })
    }
}
