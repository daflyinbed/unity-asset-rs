//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct MonoScript {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_script: String,
    pub m_default_references: Vec<(String, PPtr)>,
    pub m_icon: PPtr,
    pub m_editor_graph_data: PPtr,
    pub m_execution_order: i32,
    pub m_class_name: String,
    pub m_namespace: String,
    pub m_assembly_name: String,
    pub m_is_editor_script: bool,
}

impl crate::Parse for MonoScript {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_script = r.read_aligned_string()?;
        r.align(4)?;
        let m_default_references = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_icon = crate::PPtr::parse(r, ver)?;
        let m_editor_graph_data = crate::PPtr::parse(r, ver)?;
        let m_execution_order = r.read_i32()?;
        let m_class_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_namespace = r.read_aligned_string()?;
        r.align(4)?;
        let m_assembly_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_is_editor_script = r.read_bool()?;

        Ok(MonoScript {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_script,
            m_default_references,
            m_icon,
            m_editor_graph_data,
            m_execution_order,
            m_class_name,
            m_namespace,
            m_assembly_name,
            m_is_editor_script,
        })
    }
}
