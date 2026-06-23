//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PresetType {
    pub m_native_type_id: i32,
    pub m_managed_type_p_ptr: PPtr,
    pub m_managed_type_fallback: String,
}

impl crate::Parse for PresetType {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_native_type_id = r.read_i32()?;
        let m_managed_type_p_ptr = crate::PPtr::parse(r, ver)?;
        let m_managed_type_fallback = r.read_aligned_string()?;
        r.align(4)?;

        Ok(PresetType {
            m_native_type_id,
            m_managed_type_p_ptr,
            m_managed_type_fallback,
        })
    }
}
