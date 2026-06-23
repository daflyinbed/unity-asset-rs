//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;

#[derive(Debug, Clone)]
pub struct Material {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_shader: PPtr,
    pub m_saved_properties: UnityPropertySheet,
}

impl crate::Parse for Material {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_shader = crate::PPtr::parse(r, ver)?;
        let m_saved_properties = UnityPropertySheet::parse(r, ver)?;
        r.align(4)?;

        Ok(Material {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_shader,
            m_saved_properties,
        })
    }
}
