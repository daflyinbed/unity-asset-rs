//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct GenericBinding {
    pub path: u32,
    pub attribute: u32,
    pub script: PPtr,
    pub class_id: u16,
    pub custom_type: u8,
    pub is_p_ptr_curve: u8,
}

impl crate::Parse for GenericBinding {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let path = r.read_u32()?;
        let attribute = r.read_u32()?;
        let script = crate::PPtr::parse(r, ver)?;
        let class_id = r.read_u16()?;
        let custom_type = r.read_u8()?;
        let is_p_ptr_curve = r.read_u8()?;

        Ok(GenericBinding {
            path,
            attribute,
            script,
            class_id,
            custom_type,
            is_p_ptr_curve,
        })
    }
}
