//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SubstanceEnumItem {
    pub value: i32,
    pub text: String,
}

impl crate::Parse for SubstanceEnumItem {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = r.read_i32()?;
        let text = r.read_aligned_string()?;
        r.align(4)?;

        Ok(SubstanceEnumItem {
            value,
            text,
        })
    }
}
