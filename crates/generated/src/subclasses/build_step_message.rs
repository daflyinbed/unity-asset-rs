//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildStepMessage {
    pub r#type: i32,
    pub content: String,
}

impl crate::Parse for BuildStepMessage {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = r.read_i32()?;
        let content = r.read_aligned_string()?;
        r.align(4)?;

        Ok(BuildStepMessage {
            r#type,
            content,
        })
    }
}
