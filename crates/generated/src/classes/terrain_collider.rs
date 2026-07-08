//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;


/// 版本枚举：TerrainColliderMMaterialVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TerrainColliderMMaterialVariant {
    Default(PPtr),
    V2023_3(PPtr),
}

impl crate::Parse for TerrainColliderMMaterialVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::V2023_3(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TerrainCollider {
    pub m_game_object: PPtr,
    pub m_material: Option<TerrainColliderMMaterialVariant>,
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
        let mut m_game_object = None;
        let mut m_material = None;
        let mut m_is_trigger = None;
        let mut m_enabled = None;
        let mut m_include_layers = None;
        let mut m_terrain_data = None;
        let mut m_exclude_layers = None;
        let mut m_create_tree_colliders = None;
        let mut m_enable_tree_colliders = None;
        let mut m_layer_override_priority = None;
        let mut m_provides_contacts = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_material = Some(TerrainColliderMMaterialVariant::parse(r, ver)?);
            m_include_layers = Some(BitField::parse(r, ver)?);
            m_exclude_layers = Some(BitField::parse(r, ver)?);
            m_layer_override_priority = Some(r.read_i32()?);
            m_provides_contacts = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_material = Some(TerrainColliderMMaterialVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_is_trigger = Some(r.read_bool()?);
        } else {
            m_material = Some(TerrainColliderMMaterialVariant::parse(r, ver)?);
            m_is_trigger = Some(r.read_bool()?);
        }
            m_enabled = Some(r.read_bool()?);
            r.align(4)?;
            m_terrain_data = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_tree_colliders = Some(r.read_bool()?);
        } else {
            m_create_tree_colliders = Some(r.read_bool()?);
        }

        Ok(TerrainCollider {
            m_game_object: m_game_object.unwrap(),
            m_material,
            m_is_trigger,
            m_enabled: m_enabled.unwrap(),
            m_include_layers,
            m_terrain_data: m_terrain_data.unwrap(),
            m_exclude_layers,
            m_create_tree_colliders,
            m_enable_tree_colliders,
            m_layer_override_priority,
            m_provides_contacts,
        })
    }
}
