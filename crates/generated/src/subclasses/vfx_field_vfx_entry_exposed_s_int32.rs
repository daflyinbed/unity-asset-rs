//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_exposed_s_int32::VfxEntryExposed_SInt32;

#[derive(Debug, Clone)]
pub struct VfxField_VfxEntryExposed_SInt32 {
    pub m_array: Vec<VfxEntryExposed_SInt32>,
}

impl crate::Parse for VfxField_VfxEntryExposed_SInt32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExposed_SInt32::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(VfxField_VfxEntryExposed_SInt32 {
            m_array,
        })
    }
}
