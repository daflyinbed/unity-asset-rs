//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_bool {
    pub m_expression_index: u32,
    pub m_value: bool,
}

impl crate::Parse for VfxEntryExpressionValue_bool {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_bool {
            m_expression_index,
            m_value,
        })
    }
}
