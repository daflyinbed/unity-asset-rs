//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bitset::bitset;

#[derive(Debug, Clone)]
pub struct EditorExtensionImpl {
    pub m_object: PPtr,
    pub m_template_father: PPtr,
    pub m_data_template: PPtr,
    pub m_override_variable: bitset,
    pub g_flattened_type_tree: Vec<u8>,
}

impl crate::Parse for EditorExtensionImpl {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object = crate::PPtr::parse(r, ver)?;
        let m_template_father = crate::PPtr::parse(r, ver)?;
        let m_data_template = crate::PPtr::parse(r, ver)?;
        let m_override_variable = bitset::parse(r, ver)?;
        r.align(4)?;
        let g_flattened_type_tree = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(EditorExtensionImpl {
            m_object,
            m_template_father,
            m_data_template,
            m_override_variable,
            g_flattened_type_tree,
        })
    }
}
