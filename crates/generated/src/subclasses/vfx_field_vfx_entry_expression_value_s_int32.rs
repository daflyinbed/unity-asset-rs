//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_expression_value_s_int32::VfxEntryExpressionValue_SInt32;

#[derive(Debug, Clone)]
pub struct VfxField_VfxEntryExpressionValue_SInt32 {
    pub m_array: Vec<VfxEntryExpressionValue_SInt32>,
}

impl crate::Parse for VfxField_VfxEntryExpressionValue_SInt32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VfxEntryExpressionValue_SInt32::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(VfxField_VfxEntryExpressionValue_SInt32 { m_array })
    }
}
