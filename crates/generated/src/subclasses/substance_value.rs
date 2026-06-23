//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SubstanceValue {
    pub scalar_0: f32,
    pub scalar_1: f32,
    pub scalar_2: f32,
    pub scalar_3: f32,
    pub texture: PPtr,
}

impl crate::Parse for SubstanceValue {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let scalar_0 = r.read_f32()?;
        let scalar_1 = r.read_f32()?;
        let scalar_2 = r.read_f32()?;
        let scalar_3 = r.read_f32()?;
        let texture = crate::PPtr::parse(r, ver)?;

        Ok(SubstanceValue {
            scalar_0,
            scalar_1,
            scalar_2,
            scalar_3,
            texture,
        })
    }
}
