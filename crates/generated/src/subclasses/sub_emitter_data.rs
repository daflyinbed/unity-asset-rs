//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SubEmitterData {
    pub emitter: PPtr,
    pub r#type: i32,
    pub properties: i32,
    pub emit_probability: Option<f32>,
}

impl crate::Parse for SubEmitterData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let emitter = crate::PPtr::parse(rdr, ver)?;
        let r#type = rdr.read_i32()?;
        let properties = rdr.read_i32()?;
        let emit_probability = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(SubEmitterData {
            emitter,
            r#type,
            properties,
            emit_probability,
        })
    }
}
