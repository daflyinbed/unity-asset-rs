//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Grid {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_cell_size: Vector3f,
    pub m_cell_gap: Vector3f,
    pub m_cell_layout: i32,
    pub m_cell_swizzle: i32,
}

impl crate::Parse for Grid {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_cell_size = Vector3f::parse(rdr, ver)?;
        let m_cell_gap = Vector3f::parse(rdr, ver)?;
        let m_cell_layout = rdr.read_i32()?;
        let m_cell_swizzle = rdr.read_i32()?;

        Ok(Grid {
            m_game_object,
            m_enabled,
            m_cell_size,
            m_cell_gap,
            m_cell_layout,
            m_cell_swizzle,
        })
    }
}
