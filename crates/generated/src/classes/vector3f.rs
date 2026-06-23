//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Vector3f {
}

impl crate::Parse for Vector3f {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {

        Ok(Vector3f {
        })
    }
}
