//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ComponentPair {
    pub component: PPtr,
}

impl crate::Parse for ComponentPair {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let component = crate::PPtr::parse(rdr, ver)?;

        Ok(ComponentPair {
            component,
        })
    }
}
