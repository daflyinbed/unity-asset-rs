//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedShaderDependency {
    pub from: String,
    pub to: String,
}

impl crate::Parse for SerializedShaderDependency {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let from = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let to = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(SerializedShaderDependency {
            from,
            to,
        })
    }
}
