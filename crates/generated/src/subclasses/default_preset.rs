//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct DefaultPreset {
    pub m_preset: PPtr,
}

impl crate::Parse for DefaultPreset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_preset = crate::PPtr::parse(r, ver)?;

        Ok(DefaultPreset {
            m_preset,
        })
    }
}
