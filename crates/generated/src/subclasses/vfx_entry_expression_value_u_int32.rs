//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_UInt32 {
    pub m_expression_index: u32,
    pub m_value: u32,
}

impl crate::Parse for VfxEntryExpressionValue_UInt32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = rdr.read_u32()?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_UInt32 {
            m_expression_index,
            m_value,
        })
    }
}
