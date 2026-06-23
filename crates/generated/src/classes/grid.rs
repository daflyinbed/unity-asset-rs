//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;
use super::grid_layout::GridLayout;

#[derive(Debug, Clone)]
pub struct Grid {
    pub parent: GridLayout,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_cell_size: Vector3f,
    pub m_cell_gap: Vector3f,
    pub m_cell_layout: i32,
    pub m_cell_swizzle: i32,
}

impl crate::Parse for Grid {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GridLayout::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_cell_size = Vector3f::parse(r, ver)?;
        let m_cell_gap = Vector3f::parse(r, ver)?;
        let m_cell_layout = r.read_i32()?;
        let m_cell_swizzle = r.read_i32()?;

        Ok(Grid {
            parent,
            m_game_object,
            m_enabled,
            m_cell_size,
            m_cell_gap,
            m_cell_layout,
            m_cell_swizzle,
        })
    }
}
