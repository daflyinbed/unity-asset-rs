//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;

/// 版本枚举：LimitMMinVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LimitMMinVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for LimitMMinVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ({ Ok(Box::new(float3::parse(rdr, ver)?)) })?;
            Ok(Self::V5_4(v))
        } else {
            let v = ({ Ok(Box::new(float4::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LimitMMaxVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LimitMMaxVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for LimitMMaxVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ({ Ok(Box::new(float3::parse(rdr, ver)?)) })?;
            Ok(Self::V5_4(v))
        } else {
            let v = ({ Ok(Box::new(float4::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Limit {
    pub m_min: LimitMMinVariant,
    pub m_max: LimitMMaxVariant,
}

impl crate::Parse for Limit {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_min = LimitMMinVariant::parse(rdr, ver)?;
        let m_max = LimitMMaxVariant::parse(rdr, ver)?;

        Ok(Limit { m_min, m_max })
    }
}
