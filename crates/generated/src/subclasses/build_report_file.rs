//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildReportFile {
    pub path: String,
    pub role: String,
    pub id: u32,
    pub total_size: u64,
    pub flags: Option<i32>,
}

impl crate::Parse for BuildReportFile {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let path = rdr.read_aligned_string()?;
        let role = rdr.read_aligned_string()?;
        let id = rdr.read_u32()?;
        let total_size = rdr.read_u64()?;
        let flags = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(BuildReportFile {
            path,
            role,
            id,
            total_size,
            flags,
        })
    }
}
