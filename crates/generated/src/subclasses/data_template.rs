//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct DataTemplate {
    pub m_last_merge_identifier: Guid,
    pub m_objects: Vec<PPtr>,
    pub m_father: PPtr,
    pub m_is_data_template: bool,
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
}

impl crate::Parse for DataTemplate {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_last_merge_identifier = Guid::parse(r, ver)?;
        let m_objects = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_father = crate::PPtr::parse(r, ver)?;
        let m_is_data_template = r.read_bool()?;
        r.align(4)?;
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(DataTemplate {
            m_last_merge_identifier,
            m_objects,
            m_father,
            m_is_data_template,
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
        })
    }
}
