//! 自动生成。请勿手动修改。

use crate::subclasses::float4::float4;
use crate::subclasses::limit::Limit;

#[derive(Debug, Clone)]
pub struct Axes {
    pub m_pre_q: float4,
    pub m_post_q: float4,
    pub m_sgn: float4,
    pub m_limit: Limit,
    pub m_length: f32,
    pub m_type: u32,
}

impl crate::Parse for Axes {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_pre_q = float4::parse(r, ver)?;
        let m_post_q = float4::parse(r, ver)?;
        let m_sgn = float4::parse(r, ver)?;
        let m_limit = Limit::parse(r, ver)?;
        let m_length = r.read_f32()?;
        let m_type = r.read_u32()?;

        Ok(Axes {
            m_pre_q,
            m_post_q,
            m_sgn,
            m_limit,
            m_length,
            m_type,
        })
    }
}
