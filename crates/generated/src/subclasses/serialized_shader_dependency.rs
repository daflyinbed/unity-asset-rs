//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedShaderDependency {
    pub from: String,
    pub to: String,
}

impl crate::Parse for SerializedShaderDependency {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let from = r.read_aligned_string()?;
        r.align(4)?;
        let to = r.read_aligned_string()?;
        r.align(4)?;

        Ok(SerializedShaderDependency {
            from,
            to,
        })
    }
}
