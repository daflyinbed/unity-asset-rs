//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ShaderError {
    pub message: String,
    pub line: i32,
    pub warning: bool,
    pub program_error: bool,
}

impl crate::Parse for ShaderError {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let message = rdr.read_aligned_string()?;
        let line = rdr.read_i32()?;
        let warning = rdr.read_bool()?;
        let program_error = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(ShaderError {
            message,
            line,
            warning,
            program_error,
        })
    }
}
