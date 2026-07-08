//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEntryExposed_UInt32 {
    pub m_value: u32,
    pub m_name: String,
    pub m_overridden: bool,
}

impl crate::Parse for VfxEntryExposed_UInt32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value = rdr.read_u32()?;
        rdr.align(4)?;
        let m_name = rdr.read_aligned_string()?;
        let m_overridden = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxEntryExposed_UInt32 {
            m_value,
            m_name,
            m_overridden,
        })
    }
}
