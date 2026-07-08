//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_exposed_bool::VfxEntryExposed_bool;

#[derive(Debug, Clone)]
pub struct VfxField_VfxEntryExposed_bool {
    pub m_array: Vec<VfxEntryExposed_bool>,
}

impl crate::Parse for VfxField_VfxEntryExposed_bool {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEntryExposed_bool::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(VfxField_VfxEntryExposed_bool {
            m_array,
        })
    }
}
