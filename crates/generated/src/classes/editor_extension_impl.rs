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
        let m_object = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };
        let m_template_father = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };
        let m_data_template = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };
        let m_override_variable = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = bitset::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let g_flattened_type_tree = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(EditorExtensionImpl {
            m_object,
            m_template_father,
            m_data_template,
            m_override_variable,
            g_flattened_type_tree,
        })
    }
}
