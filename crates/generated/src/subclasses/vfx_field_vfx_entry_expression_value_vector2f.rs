//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_expression_value_vector2f::VfxEntryExpressionValue_Vector2f;

#[derive(Debug, Clone)]
pub struct VfxField_VfxEntryExpressionValue_Vector2f {
    pub m_array: Vec<VfxEntryExpressionValue_Vector2f>,
}

impl crate::Parse for VfxField_VfxEntryExpressionValue_Vector2f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VfxEntryExpressionValue_Vector2f::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(VfxField_VfxEntryExpressionValue_Vector2f { m_array })
    }
}
