//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct DirectorGenericBinding {
    pub key: PPtr,
    pub value: PPtr,
}

impl crate::Parse for DirectorGenericBinding {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let key = crate::PPtr::parse(rdr, ver)?;
        let value = crate::PPtr::parse(rdr, ver)?;

        Ok(DirectorGenericBinding {
            key,
            value,
        })
    }
}
