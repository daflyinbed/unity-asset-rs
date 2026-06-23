//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MetroFtaFileType {
    pub content_type: String,
    pub file_type: String,
}

impl crate::Parse for MetroFtaFileType {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let content_type = r.read_aligned_string()?;
        r.align(4)?;
        let file_type = r.read_aligned_string()?;
        r.align(4)?;

        Ok(MetroFtaFileType {
            content_type,
            file_type,
        })
    }
}
