//! 自动生成。请勿手动修改。

use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_Vector4f {
    pub m_expression_index: u32,
    pub m_value: Vector4f,
}

impl crate::Parse for VfxEntryExpressionValue_Vector4f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = Vector4f::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_Vector4f {
            m_expression_index,
            m_value,
        })
    }
}
