//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SubEmitterData {
    pub emitter: PPtr,
    pub r#type: i32,
    pub properties: i32,
}

impl crate::Parse for SubEmitterData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let emitter = crate::PPtr::parse(r, ver)?;
        let r#type = r.read_i32()?;
        let properties = r.read_i32()?;

        Ok(SubEmitterData {
            emitter,
            r#type,
            properties,
        })
    }
}
