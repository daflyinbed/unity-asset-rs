//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ShaderBindChannel {
    pub source: i8,
    pub target: i8,
}

impl crate::Parse for ShaderBindChannel {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let source = rdr.read_i8()?;
        let target = rdr.read_i8()?;

        Ok(ShaderBindChannel {
            source,
            target,
        })
    }
}
