//! 自动生成。请勿手动修改。

use crate::subclasses::gradient::Gradient;

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_Gradient {
    pub m_expression_index: u32,
    pub m_value: Gradient,
}

impl crate::Parse for VfxEntryExpressionValue_Gradient {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = Gradient::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_Gradient {
            m_expression_index,
            m_value,
        })
    }
}
