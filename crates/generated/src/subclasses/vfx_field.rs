//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_expression_value::VfxEntryExpressionValue;

#[derive(Debug, Clone)]
pub struct VfxField {
    pub m_array: Vec<VfxEntryExpressionValue>,
}

impl crate::Parse for VfxField {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxEntryExpressionValue::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(VfxField {
            m_array,
        })
    }
}
