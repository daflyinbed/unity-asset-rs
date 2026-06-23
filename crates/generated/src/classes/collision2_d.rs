//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Collision2D {
}

impl crate::Parse for Collision2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {

        Ok(Collision2D {
        })
    }
}
