//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub file_path: String,
    pub start_line: u32,
    pub start_char: u32,
    pub severity: i32,
}

impl crate::Parse for Error {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let message = r.read_aligned_string()?;
        r.align(4)?;
        let file_path = r.read_aligned_string()?;
        r.align(4)?;
        let start_line = r.read_u32()?;
        let start_char = r.read_u32()?;
        let severity = r.read_i32()?;

        Ok(Error {
            message,
            file_path,
            start_line,
            start_char,
            severity,
        })
    }
}
