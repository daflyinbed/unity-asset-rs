//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue {
    pub m_expression_index: u32,
    pub m_value: f32,
}

impl crate::Parse for VfxEntryExpressionValue {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = r.read_u32()?;
        let m_value = r.read_f32()?;
        r.align(4)?;

        Ok(VfxEntryExpressionValue {
            m_expression_index,
            m_value,
        })
    }
}
