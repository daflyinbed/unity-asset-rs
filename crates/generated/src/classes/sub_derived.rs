//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SubDerived {
}

impl crate::Parse for SubDerived {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {

        Ok(SubDerived {
        })
    }
}
