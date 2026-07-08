//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct VfxEntryExposed_PPtr_NamedObject_ {
    pub m_value: PPtr,
    pub m_name: String,
    pub m_overridden: bool,
}

impl crate::Parse for VfxEntryExposed_PPtr_NamedObject_ {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value = crate::PPtr::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_name = rdr.read_aligned_string()?;
        let m_overridden = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxEntryExposed_PPtr_NamedObject_ {
            m_value,
            m_name,
            m_overridden,
        })
    }
}
