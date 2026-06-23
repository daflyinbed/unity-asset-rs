//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Tile {
    pub m_tile_index: u32,
    pub m_tile_sprite_index: u32,
    pub m_tile_matrix_index: u32,
    pub m_tile_color_index: u32,
    pub m_object_to_instantiate: PPtr,
    pub m_tile_flags: u32,
    pub m_collider_type: i32,
}

impl crate::Parse for Tile {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_tile_index = r.read_u32()?;
        let m_tile_sprite_index = r.read_u32()?;
        let m_tile_matrix_index = r.read_u32()?;
        let m_tile_color_index = r.read_u32()?;
        let m_object_to_instantiate = crate::PPtr::parse(r, ver)?;
        let m_tile_flags = r.read_u32()?;
        let m_collider_type = r.read_i32()?;

        Ok(Tile {
            m_tile_index,
            m_tile_sprite_index,
            m_tile_matrix_index,
            m_tile_color_index,
            m_object_to_instantiate,
            m_tile_flags,
            m_collider_type,
        })
    }
}
