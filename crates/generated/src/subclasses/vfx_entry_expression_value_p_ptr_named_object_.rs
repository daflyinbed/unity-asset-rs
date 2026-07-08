//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_PPtr_NamedObject_ {
    pub m_expression_index: u32,
    pub m_value: PPtr,
}

impl crate::Parse for VfxEntryExpressionValue_PPtr_NamedObject_ {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = crate::PPtr::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_PPtr_NamedObject_ {
            m_expression_index,
            m_value,
        })
    }
}
