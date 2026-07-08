//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;


/// 版本枚举：xformTVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum xformTVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for xformTVariant {
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

/// 版本枚举：xformSVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum xformSVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for xformSVariant {
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
pub struct xform {
    pub t: xformTVariant,
    pub q: float4,
    pub s: xformSVariant,
}

impl crate::Parse for xform {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let t = xformTVariant::parse(rdr, ver)?;
        let q = float4::parse(rdr, ver)?;
        let s = xformSVariant::parse(rdr, ver)?;

        Ok(xform {
            t,
            q,
            s,
        })
    }
}
