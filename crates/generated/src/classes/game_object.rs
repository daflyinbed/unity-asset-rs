//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::editor_extension::EditorExtension;

#[derive(Debug, Clone)]
pub struct GameObject {
    pub parent: EditorExtension,
    pub m_component: Vec<(i32, PPtr)>,
    pub m_layer: u32,
    pub m_name: String,
    pub m_tag: u16,
    pub m_is_active: u8,
}

impl crate::Parse for GameObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = EditorExtension::parse(r, ver)?;
        let m_component = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_layer = r.read_u32()?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_tag = r.read_u16()?;
        let m_is_active = r.read_u8()?;

        Ok(GameObject {
            parent,
            m_component,
            m_layer,
            m_name,
            m_tag,
            m_is_active,
        })
    }
}
