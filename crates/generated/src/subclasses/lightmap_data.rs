//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LightmapData {
    pub m_lightmap: PPtr,
    pub m_indirect_lightmap: PPtr,
}

impl crate::Parse for LightmapData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lightmap = crate::PPtr::parse(r, ver)?;
        let m_indirect_lightmap = crate::PPtr::parse(r, ver)?;

        Ok(LightmapData {
            m_lightmap,
            m_indirect_lightmap,
        })
    }
}
