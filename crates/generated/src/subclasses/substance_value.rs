//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SubstanceValue {
    pub scalar_0: f32,
    pub scalar_1: f32,
    pub scalar_2: f32,
    pub scalar_3: f32,
    pub texture: PPtr,
    pub stringvalue: Option<String>,
}

impl crate::Parse for SubstanceValue {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let scalar_0 = rdr.read_f32()?;
        let scalar_1 = rdr.read_f32()?;
        let scalar_2 = rdr.read_f32()?;
        let scalar_3 = rdr.read_f32()?;
        let texture = crate::PPtr::parse(rdr, ver)?;
        let stringvalue = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SubstanceValue {
            scalar_0,
            scalar_1,
            scalar_2,
            scalar_3,
            texture,
            stringvalue,
        })
    }
}
