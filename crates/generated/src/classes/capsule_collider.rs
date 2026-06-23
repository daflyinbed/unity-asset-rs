//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::vector3f::Vector3f;
use super::collider::Collider;

#[derive(Debug, Clone)]
pub struct CapsuleCollider {
    pub parent: Collider,
    pub m_game_object: PPtr,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_include_layers: Option<BitField>,
    pub m_enabled: bool,
    pub m_exclude_layers: Option<BitField>,
    pub m_radius: f32,
    pub m_layer_override_priority: Option<i32>,
    pub m_height: f32,
    pub m_direction: i32,
    pub m_provides_contacts: Option<bool>,
    pub m_center: Vector3f,
}

impl crate::Parse for CapsuleCollider {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Collider::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_is_trigger = r.read_bool()?;
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_enabled = r.read_bool()?;
        r.align(4)?;
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_radius = r.read_f32()?;
        let m_layer_override_priority = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_height = r.read_f32()?;
        let m_direction = r.read_i32()?;
        let m_provides_contacts = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_center = Vector3f::parse(r, ver)?;

        Ok(CapsuleCollider {
            parent,
            m_game_object,
            m_material,
            m_is_trigger,
            m_include_layers,
            m_enabled,
            m_exclude_layers,
            m_radius,
            m_layer_override_priority,
            m_height,
            m_direction,
            m_provides_contacts,
            m_center,
        })
    }
}
