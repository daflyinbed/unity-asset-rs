//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildReportFile {
    pub path: String,
    pub role: String,
    pub id: u32,
    pub total_size: u64,
}

impl crate::Parse for BuildReportFile {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let path = r.read_aligned_string()?;
        r.align(4)?;
        let role = r.read_aligned_string()?;
        r.align(4)?;
        let id = r.read_u32()?;
        let total_size = r.read_u64()?;

        Ok(BuildReportFile {
            path,
            role,
            id,
            total_size,
        })
    }
}
