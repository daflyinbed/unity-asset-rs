//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildSummary {
    pub name: String,
    pub platform_name: String,
    pub options: i32,
    pub asset_bundle_options: i32,
    pub output_path: String,
    pub crc: u32,
    pub total_size: u64,
    pub total_time_ms: u64,
    pub total_errors: i32,
    pub total_warnings: i32,
}

impl crate::Parse for BuildSummary {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let platform_name = r.read_aligned_string()?;
        r.align(4)?;
        let options = r.read_i32()?;
        let asset_bundle_options = r.read_i32()?;
        let output_path = r.read_aligned_string()?;
        r.align(4)?;
        let crc = r.read_u32()?;
        let total_size = r.read_u64()?;
        let total_time_ms = r.read_u64()?;
        let total_errors = r.read_i32()?;
        let total_warnings = r.read_i32()?;

        Ok(BuildSummary {
            name,
            platform_name,
            options,
            asset_bundle_options,
            output_path,
            crc,
            total_size,
            total_time_ms,
            total_errors,
            total_warnings,
        })
    }
}
