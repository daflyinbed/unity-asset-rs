//! 自动生成。请勿手动修改。

use crate::subclasses::native_type::NativeType;

#[derive(Debug, Clone)]
pub struct NativeObjectType {
    pub m_object_hide_flags: u32,
    pub m_inner: NativeType,
}

impl crate::Parse for NativeObjectType {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_inner = NativeType::parse(r, ver)?;
        r.align(4)?;

        Ok(NativeObjectType {
            m_object_hide_flags,
            m_inner,
        })
    }
}
