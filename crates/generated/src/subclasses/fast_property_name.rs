//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct FastPropertyName {
    pub name: String,
}

impl crate::Parse for FastPropertyName {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(FastPropertyName {
            name,
        })
    }
}
