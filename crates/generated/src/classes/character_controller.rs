//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::vector3f::Vector3f;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct CharacterController {
    pub m_game_object: PPtr,
    pub m_height: f32,
    pub m_material: Option<PPtr>,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_height = r.read_f32()?;
        let m_material = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_radius = r.read_f32()?;
        let m_is_trigger = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_slope_limit = r.read_f32()?;
        let m_enabled = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_step_offset = r.read_f32()?;
        let m_layer_override_priority = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_skin_width = r.read_f32()?;
        let m_provides_contacts = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_min_move_distance = r.read_f32()?;
        let m_center = Vector3f::parse(r, ver)?;

        Ok(CharacterController {
            m_game_object,
            m_height,
            m_material,
            m_radius,
            m_is_trigger,
            m_include_layers,
            m_slope_limit,
            m_enabled,
            m_exclude_layers,
            m_step_offset,
            m_layer_override_priority,
            m_skin_width,
            m_provides_contacts,
            m_min_move_distance,
            m_center,
        })
    }
}
