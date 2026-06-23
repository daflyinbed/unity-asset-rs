//! 自动生成。请勿手动修改。

use super::text_asset::TextAsset;

#[derive(Debug, Clone)]
pub struct MonoScript {
    pub parent: TextAsset,
    pub m_name: String,
    pub m_execution_order: i32,
    pub m_properties_hash: u32,
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
        let parent = TextAsset::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_execution_order = r.read_i32()?;
        let m_properties_hash = r.read_u32()?;
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
            parent,
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
