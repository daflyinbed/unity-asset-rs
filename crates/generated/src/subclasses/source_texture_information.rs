//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SourceTextureInformation {
    pub width: i32,
    pub height: i32,
    pub does_texture_contain_alpha: bool,
}

impl crate::Parse for SourceTextureInformation {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let width = r.read_i32()?;
        let height = r.read_i32()?;
        let does_texture_contain_alpha = r.read_bool()?;

        Ok(SourceTextureInformation {
            width,
            height,
            does_texture_contain_alpha,
        })
    }
}
