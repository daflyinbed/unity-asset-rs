//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PPtrKeyframe {
    pub time: f32,
    pub value: PPtr,
}

impl crate::Parse for PPtrKeyframe {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = r.read_f32()?;
        let value = crate::PPtr::parse(r, ver)?;

        Ok(PPtrKeyframe {
            time,
            value,
        })
    }
}
