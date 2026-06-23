//! 自动生成。请勿手动修改。

use crate::subclasses::float4::float4;

#[derive(Debug, Clone)]
pub struct Limit {
    pub m_min: float4,
    pub m_max: float4,
}

impl crate::Parse for Limit {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_min = float4::parse(r, ver)?;
        let m_max = float4::parse(r, ver)?;

        Ok(Limit {
            m_min,
            m_max,
        })
    }
}
