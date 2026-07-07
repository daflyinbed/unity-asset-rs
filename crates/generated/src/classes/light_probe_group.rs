//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LightProbeGroup {
    pub m_game_object: PPtr,
    pub m_enabled: Option<u8>,
}

impl crate::Parse for LightProbeGroup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(LightProbeGroup {
            m_game_object,
            m_enabled,
        })
    }
}
