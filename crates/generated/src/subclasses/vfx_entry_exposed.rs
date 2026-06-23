//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEntryExposed {
    pub m_value: f32,
    pub m_name: String,
    pub m_overridden: bool,
}

impl crate::Parse for VfxEntryExposed {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value = r.read_f32()?;
        r.align(4)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_overridden = r.read_bool()?;
        r.align(4)?;

        Ok(VfxEntryExposed {
            m_value,
            m_name,
            m_overridden,
        })
    }
}
