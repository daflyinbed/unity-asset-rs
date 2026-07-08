//! 自动生成。请勿手动修改。

use crate::subclasses::matrix4x4f::Matrix4x4f;

#[derive(Debug, Clone)]
pub struct VfxEntryExpressionValue_Matrix4x4f {
    pub m_expression_index: u32,
    pub m_value: Matrix4x4f,
}

impl crate::Parse for VfxEntryExpressionValue_Matrix4x4f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expression_index = rdr.read_u32()?;
        let m_value = Matrix4x4f::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(VfxEntryExpressionValue_Matrix4x4f {
            m_expression_index,
            m_value,
        })
    }
}
