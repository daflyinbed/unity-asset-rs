//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct float_ {
}

impl crate::Parse for float_ {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {

        Ok(float_ {
        })
    }
}
