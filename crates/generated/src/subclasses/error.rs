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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let message = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let file_path = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let start_line = rdr.read_u32()?;
        let start_char = rdr.read_u32()?;
        let severity = rdr.read_i32()?;

        Ok(Error {
            message,
            file_path,
            start_line,
            start_char,
            severity,
        })
    }
}
