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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let message = r.read_aligned_string()?;
        r.align(4)?;
        let line = r.read_i32()?;
        let warning = r.read_bool()?;
        let program_error = r.read_bool()?;
        r.align(4)?;

        Ok(ShaderError {
            message,
            line,
            warning,
            program_error,
        })
    }
}
