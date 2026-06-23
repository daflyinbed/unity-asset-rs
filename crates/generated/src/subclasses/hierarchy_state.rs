//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct HierarchyState {
    pub m_object_hide_flags: u32,
    pub expanded: Vec<PPtr>,
    pub selection: Vec<PPtr>,
    pub scrollposition_x: f32,
    pub scrollposition_y: f32,
}

impl crate::Parse for HierarchyState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let expanded = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let selection = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let scrollposition_x = r.read_f32()?;
        let scrollposition_y = r.read_f32()?;

        Ok(HierarchyState {
            m_object_hide_flags,
            expanded,
            selection,
            scrollposition_x,
            scrollposition_y,
        })
    }
}
