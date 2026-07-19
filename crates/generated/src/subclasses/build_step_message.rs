//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildStepMessage {
    pub r#type: i32,
    pub content: String,
}

impl crate::Parse for BuildStepMessage {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = rdr.read_i32()?;
        let content = rdr.read_aligned_string()?;

        Ok(BuildStepMessage { r#type, content })
    }
}
