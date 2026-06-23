//! 自动生成。请勿手动修改。

use crate::subclasses::float4::float4;

#[derive(Debug, Clone)]
pub struct xform {
    pub t: float4,
    pub q: float4,
    pub s: float4,
}

impl crate::Parse for xform {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let t = float4::parse(r, ver)?;
        let q = float4::parse(r, ver)?;
        let s = float4::parse(r, ver)?;

        Ok(xform {
            t,
            q,
            s,
        })
    }
}
