//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct KeywordTargetInfo {
    pub keyword_name: String,
    pub requirements: i32,
}

impl crate::Parse for KeywordTargetInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let keyword_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let requirements = rdr.read_i32()?;

        Ok(KeywordTargetInfo {
            keyword_name,
            requirements,
        })
    }
}
