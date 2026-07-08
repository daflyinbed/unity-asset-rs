//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LodRenderer {
    pub renderer: PPtr,
}

impl crate::Parse for LodRenderer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let renderer = crate::PPtr::parse(rdr, ver)?;

        Ok(LodRenderer {
            renderer,
        })
    }
}
