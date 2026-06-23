//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VariantInfo {
    pub keywords: String,
    pub pass_type: i32,
}

impl crate::Parse for VariantInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let keywords = r.read_aligned_string()?;
        r.align(4)?;
        let pass_type = r.read_i32()?;

        Ok(VariantInfo {
            keywords,
            pass_type,
        })
    }
}
