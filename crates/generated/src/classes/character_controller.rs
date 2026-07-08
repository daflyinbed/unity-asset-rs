//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：CharacterControllerMGameObjectVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CharacterControllerMGameObjectVariant {
    Default(PPtr),
    V3_5(PPtr),
}

impl crate::Parse for CharacterControllerMGameObjectVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V3_5(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：CharacterControllerMMaterialVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CharacterControllerMMaterialVariant {
    Default(PPtr),
    V2023_3(PPtr),
}

impl crate::Parse for CharacterControllerMMaterialVariant {
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
pub struct CharacterController {
    pub m_game_object: CharacterControllerMGameObjectVariant,
    pub m_height: f32,
    pub m_material: Option<CharacterControllerMMaterialVariant>,
    pub m_radius: f32,
    pub m_is_trigger: Option<bool>,
    pub m_include_layers: Option<BitField>,
    pub m_slope_limit: f32,
    pub m_enabled: Option<bool>,
    pub m_exclude_layers: Option<BitField>,
    pub m_step_offset: f32,
    pub m_layer_override_priority: Option<i32>,
    pub m_skin_width: f32,
    pub m_provides_contacts: Option<bool>,
    pub m_min_move_distance: f32,
    pub m_center: Vector3f,
}

impl crate::Parse for CharacterController {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_height = None;
        let mut m_material = None;
        let mut m_radius = None;
        let mut m_is_trigger = None;
        let mut m_include_layers = None;
        let mut m_slope_limit = None;
        let mut m_enabled = None;
        let mut m_exclude_layers = None;
        let mut m_step_offset = None;
        let mut m_layer_override_priority = None;
        let mut m_skin_width = None;
        let mut m_provides_contacts = None;
        let mut m_min_move_distance = None;
        let mut m_center = None;
            m_game_object = Some(CharacterControllerMGameObjectVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_material = Some(CharacterControllerMMaterialVariant::parse(rdr, ver)?);
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_layer_override_priority = Some(rdr.read_i32()?);
            m_provides_contacts = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_material = Some(CharacterControllerMMaterialVariant::parse(rdr, ver)?);
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_layer_override_priority = Some(rdr.read_i32()?);
            m_is_trigger = Some(rdr.read_bool()?);
            m_provides_contacts = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_material = Some(CharacterControllerMMaterialVariant::parse(rdr, ver)?);
            m_is_trigger = Some(rdr.read_bool()?);
            m_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_height = Some(rdr.read_f32()?);
            m_radius = Some(rdr.read_f32()?);
            m_slope_limit = Some(rdr.read_f32()?);
            m_step_offset = Some(rdr.read_f32()?);
            m_skin_width = Some(rdr.read_f32()?);
            m_min_move_distance = Some(rdr.read_f32()?);
            m_center = Some(Vector3f::parse(rdr, ver)?);

        Ok(CharacterController {
            m_game_object: m_game_object.unwrap(),
            m_height: m_height.unwrap(),
            m_material,
            m_radius: m_radius.unwrap(),
            m_is_trigger,
            m_include_layers,
            m_slope_limit: m_slope_limit.unwrap(),
            m_enabled,
            m_exclude_layers,
            m_step_offset: m_step_offset.unwrap(),
            m_layer_override_priority,
            m_skin_width: m_skin_width.unwrap(),
            m_provides_contacts,
            m_min_move_distance: m_min_move_distance.unwrap(),
            m_center: m_center.unwrap(),
        })
    }
}
