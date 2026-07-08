//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct HierarchyState {
    pub expanded: Vec<PPtr>,
    pub selection: Vec<PPtr>,
    pub scrollposition_x: f32,
    pub scrollposition_y: f32,
}

impl crate::Parse for HierarchyState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let expanded = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let selection = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let scrollposition_x = rdr.read_f32()?;
        let scrollposition_y = rdr.read_f32()?;

        Ok(HierarchyState {
            expanded,
            selection,
            scrollposition_x,
            scrollposition_y,
        })
    }
}
