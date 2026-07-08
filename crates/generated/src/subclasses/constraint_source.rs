//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ConstraintSource {
    pub source_transform: PPtr,
    pub weight: f32,
}

impl crate::Parse for ConstraintSource {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let source_transform = crate::PPtr::parse(rdr, ver)?;
        let weight = rdr.read_f32()?;

        Ok(ConstraintSource {
            source_transform,
            weight,
        })
    }
}
