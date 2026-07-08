//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_float {
    pub m_expression_index: u32,
    pub m_value: f32,
}

impl crate::Parse for VfxEntryExpressionValue_float {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = rdr.read_f32()?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_float {
            m_expression_index,
            m_value,
        })
    }
}
