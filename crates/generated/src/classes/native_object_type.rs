//! 自动生成。请勿手动修改。

use crate::subclasses::native_type::NativeType;

#[derive(Debug, Clone)]
pub struct NativeObjectType {
    pub m_inner: NativeType,
}

impl crate::Parse for NativeObjectType {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_inner = NativeType::parse(rdr, ver)?;

        Ok(NativeObjectType {
            m_inner,
        })
    }
}
