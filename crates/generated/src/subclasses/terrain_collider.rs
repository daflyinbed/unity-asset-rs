//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TerrainCollider {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_enabled: bool,
    pub m_terrain_data: PPtr,
    pub m_create_tree_colliders: bool,
}

impl crate::Parse for TerrainCollider {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_is_trigger = r.read_bool()?;
        let m_enabled = r.read_bool()?;
        r.align(4)?;
        let m_terrain_data = crate::PPtr::parse(r, ver)?;
        let m_create_tree_colliders = r.read_bool()?;

        Ok(TerrainCollider {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_material,
            m_is_trigger,
            m_enabled,
            m_terrain_data,
            m_create_tree_colliders,
        })
    }
}
