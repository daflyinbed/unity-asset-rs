//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct GameObject {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_component: Vec<(i32, PPtr)>,
    pub m_layer: u32,
    pub m_name: String,
    pub m_tag_string: String,
    pub m_is_active: u8,
    pub m_is_static: u8,
    pub m_icon: PPtr,
}

impl crate::Parse for GameObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_component = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_layer = r.read_u32()?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_tag_string = r.read_aligned_string()?;
        r.align(4)?;
        let m_is_active = r.read_u8()?;
        let m_is_static = r.read_u8()?;
        r.align(4)?;
        let m_icon = crate::PPtr::parse(r, ver)?;

        Ok(GameObject {
            m_object_hide_flags,
            m_extension_ptr,
            m_component,
            m_layer,
            m_name,
            m_tag_string,
            m_is_active,
            m_is_static,
            m_icon,
        })
    }
}
