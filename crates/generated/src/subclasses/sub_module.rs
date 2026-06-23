//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SubModule {
    pub enabled: bool,
    pub sub_emitter_birth: PPtr,
    pub sub_emitter_death: PPtr,
    pub sub_emitter_collision: PPtr,
}

impl crate::Parse for SubModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let sub_emitter_birth = crate::PPtr::parse(r, ver)?;
        let sub_emitter_death = crate::PPtr::parse(r, ver)?;
        let sub_emitter_collision = crate::PPtr::parse(r, ver)?;

        Ok(SubModule {
            enabled,
            sub_emitter_birth,
            sub_emitter_death,
            sub_emitter_collision,
        })
    }
}
