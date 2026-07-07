//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct MonoScript {
    pub m_name: String,
    pub m_execution_order: i32,
    pub m_properties_hash: Hash128,
    pub m_class_name: String,
    pub m_namespace: String,
    pub m_assembly_name: String,
    pub m_is_editor_script: Option<bool>,
}

impl crate::Parse for MonoScript {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_execution_order = r.read_i32()?;
        let m_properties_hash = Hash128::parse(r, ver)?;
        let m_class_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_namespace = r.read_aligned_string()?;
        r.align(4)?;
        let m_assembly_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_is_editor_script = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(MonoScript {
            m_name,
            m_execution_order,
            m_properties_hash,
            m_class_name,
            m_namespace,
            m_assembly_name,
            m_is_editor_script,
        })
    }
}
