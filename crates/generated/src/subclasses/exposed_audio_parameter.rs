//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct ExposedAudioParameter {
    pub guid: Guid,
    pub name: String,
}

impl crate::Parse for ExposedAudioParameter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(r, ver)?;
        let name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(ExposedAudioParameter {
            guid,
            name,
        })
    }
}
