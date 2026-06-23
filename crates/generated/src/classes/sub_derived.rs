//! 自动生成。请勿手动修改。

use super::derived::Derived;

#[derive(Debug, Clone)]
pub struct SubDerived {
    pub parent: Derived,
}

impl crate::Parse for SubDerived {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Derived::parse(r, ver)?;

        Ok(SubDerived {
            parent,
        })
    }
}
