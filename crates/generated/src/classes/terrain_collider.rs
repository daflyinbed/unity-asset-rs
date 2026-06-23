//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use super::collider::Collider;

#[derive(Debug, Clone)]
pub struct TerrainCollider {
    pub parent: Collider,
    pub m_game_object: PPtr,
    pub m_material: Option<PPtr>,
    pub m_is_trigger: Option<bool>,
    pub m_enabled: bool,
    pub m_include_layers: Option<BitField>,
    pub m_terrain_data: PPtr,
    pub m_exclude_layers: Option<BitField>,
    pub m_create_tree_colliders: Option<bool>,
    pub m_enable_tree_colliders: Option<bool>,
    pub m_layer_override_priority: Option<i32>,
    pub m_provides_contacts: Option<bool>,
}

impl crate::Parse for TerrainCollider {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Collider::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_material = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_is_trigger = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enabled = r.read_bool()?;
        r.align(4)?;
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_terrain_data = crate::PPtr::parse(r, ver)?;
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_create_tree_colliders = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_tree_colliders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_layer_override_priority = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_provides_contacts = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(TerrainCollider {
            parent,
            m_game_object,
            m_material,
            m_is_trigger,
            m_enabled,
            m_include_layers,
            m_terrain_data,
            m_exclude_layers,
            m_create_tree_colliders,
            m_enable_tree_colliders,
            m_layer_override_priority,
            m_provides_contacts,
        })
    }
}
