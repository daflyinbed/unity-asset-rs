//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：CapsuleColliderMMaterialVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CapsuleColliderMMaterialVariant {
    Default(PPtr),
    V2023_3(PPtr),
}

impl crate::Parse for CapsuleColliderMMaterialVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V2023_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct CapsuleCollider {
    pub m_game_object: PPtr,
    pub m_material: CapsuleColliderMMaterialVariant,
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_material = None;
        let mut m_is_trigger = None;
        let mut m_include_layers = None;
        let mut m_enabled = None;
        let mut m_exclude_layers = None;
        let mut m_radius = None;
        let mut m_layer_override_priority = None;
        let mut m_height = None;
        let mut m_direction = None;
        let mut m_provides_contacts = None;
        let mut m_center = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_material = Some(CapsuleColliderMMaterialVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_layer_override_priority = Some(rdr.read_i32()?);
        } else {
        }
            m_is_trigger = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_provides_contacts = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_radius = Some(rdr.read_f32()?);
            m_height = Some(rdr.read_f32()?);
            m_direction = Some(rdr.read_i32()?);
            m_center = Some(Vector3f::parse(rdr, ver)?);

        Ok(CapsuleCollider {
            m_game_object: m_game_object.unwrap(),
            m_material: m_material.unwrap(),
            m_is_trigger: m_is_trigger.unwrap(),
            m_include_layers,
            m_enabled: m_enabled.unwrap(),
            m_exclude_layers,
            m_radius: m_radius.unwrap(),
            m_layer_override_priority,
            m_height: m_height.unwrap(),
            m_direction: m_direction.unwrap(),
            m_provides_contacts,
            m_center: m_center.unwrap(),
        })
    }
}
