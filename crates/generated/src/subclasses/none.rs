//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct None {
}

impl crate::Parse for None {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {

        Ok(None {
        })
    }
}
