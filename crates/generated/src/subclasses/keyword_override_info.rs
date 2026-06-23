//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct KeywordOverrideInfo {
    pub name: String,
    pub keep_in_build: bool,
}

impl crate::Parse for KeywordOverrideInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let keep_in_build = r.read_bool()?;

        Ok(KeywordOverrideInfo {
            name,
            keep_in_build,
        })
    }
}
