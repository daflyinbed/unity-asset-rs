//! 自动生成。请勿手动修改。

use crate::subclasses::gradient::Gradient;

#[derive(Debug, Clone)]
pub struct VfxEntryExposed_Gradient {
    pub m_value: Gradient,
    pub m_name: String,
    pub m_overridden: bool,
}

impl crate::Parse for VfxEntryExposed_Gradient {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value = Gradient::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_name = rdr.read_aligned_string()?;
        let m_overridden = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxEntryExposed_Gradient {
            m_value,
            m_name,
            m_overridden,
        })
    }
}
