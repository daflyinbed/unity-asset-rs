//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;
use crate::subclasses::limit::Limit;


/// 版本枚举：AxesMSgnVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AxesMSgnVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for AxesMSgnVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { Ok(Box::new(float3::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_4(v))
        } else {
            let v = ( { Ok(Box::new(float4::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Axes {
    pub m_pre_q: float4,
    pub m_post_q: float4,
    pub m_sgn: AxesMSgnVariant,
    pub m_limit: Limit,
    pub m_length: f32,
    pub m_type: u32,
}

impl crate::Parse for Axes {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_pre_q = float4::parse(rdr, ver)?;
        let m_post_q = float4::parse(rdr, ver)?;
        let m_sgn = AxesMSgnVariant::parse(rdr, ver)?;
        let m_limit = Limit::parse(rdr, ver)?;
        let m_length = rdr.read_f32()?;
        let m_type = rdr.read_u32()?;

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
