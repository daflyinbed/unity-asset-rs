//! 自动生成。请勿手动修改。

use crate::subclasses::embedded_native_type::EmbeddedNativeType;

#[derive(Debug, Clone)]
pub struct NativeType {
    pub a: i32,
    pub b: f32,
    pub embedded: EmbeddedNativeType,
}

impl crate::Parse for NativeType {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let a = rdr.read_i32()?;
        let b = rdr.read_f32()?;
        let embedded = EmbeddedNativeType::parse(rdr, ver)?;

        Ok(NativeType {
            a,
            b,
            embedded,
        })
    }
}
