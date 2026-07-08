//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bitset::bitset;

#[derive(Debug, Clone)]
pub struct EditorExtensionImpl {
    pub m_object: Option<PPtr>,
    pub m_template_father: Option<PPtr>,
    pub m_data_template: Option<PPtr>,
    pub m_override_variable: Option<bitset>,
    pub g_flattened_type_tree: Option<Vec<u8>>,
}

impl crate::Parse for EditorExtensionImpl {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_object = None;
        let mut m_template_father = None;
        let mut m_data_template = None;
        let mut m_override_variable = None;
        let mut g_flattened_type_tree = None;
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_template_father = Some(crate::PPtr::parse(rdr, ver)?);
            m_data_template = Some(crate::PPtr::parse(rdr, ver)?);
            m_override_variable = Some(bitset::parse(rdr, ver)?);
            rdr.align(4)?;
            g_flattened_type_tree = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        }

        Ok(EditorExtensionImpl {
            m_object,
            m_template_father,
            m_data_template,
            m_override_variable,
            g_flattened_type_tree,
        })
    }
}
