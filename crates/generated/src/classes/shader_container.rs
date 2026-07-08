//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ShaderContainer {
}

impl crate::Parse for ShaderContainer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {

        Ok(ShaderContainer {
        })
    }
}
