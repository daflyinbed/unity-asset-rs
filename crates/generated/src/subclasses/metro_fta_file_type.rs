//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MetroFtaFileType {
    pub content_type: String,
    pub file_type: String,
}

impl crate::Parse for MetroFtaFileType {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let content_type = rdr.read_aligned_string()?;
        let file_type = rdr.read_aligned_string()?;

        Ok(MetroFtaFileType {
            content_type,
            file_type,
        })
    }
}
