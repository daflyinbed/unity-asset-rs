//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use super::component::Component;

#[derive(Debug, Clone)]
pub struct Rigidbody2D {
    pub parent: Component,
    pub m_game_object: PPtr,
    pub m_mass: f32,
    pub m_use_auto_mass: Option<bool>,
    pub m_body_type: Option<i32>,
    pub m_linear_drag: Option<f32>,
    pub m_simulated: Option<bool>,
    pub m_angular_drag: Option<f32>,
    pub m_use_full_kinematic_contacts: Option<bool>,
    pub m_gravity_scale: f32,
    pub m_fixed_angle: Option<bool>,
    pub m_is_kinematic: Option<bool>,
    pub m_interpolate: u8,
    pub m_linear_damping: Option<f32>,
    pub m_sleeping_mode: u8,
    pub m_angular_damping: Option<f32>,
    pub m_collision_detection: u8,
    pub m_constraints: Option<i32>,
    pub m_material: Option<PPtr>,
    pub m_include_layers: Option<BitField>,
    pub m_exclude_layers: Option<BitField>,
}

impl crate::Parse for Rigidbody2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Component::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_mass = r.read_f32()?;
        let m_use_auto_mass = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_body_type = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_linear_drag = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_simulated = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_angular_drag = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use_full_kinematic_contacts = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_gravity_scale = r.read_f32()?;
        let m_fixed_angle = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_is_kinematic = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_interpolate = r.read_u8()?;
        let m_linear_damping = if *ver >= crate::UnityVersion::new(6000, 0, 11) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_sleeping_mode = r.read_u8()?;
        let m_angular_damping = if *ver >= crate::UnityVersion::new(6000, 0, 11) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_collision_detection = r.read_u8()?;
        r.align(4)?;
        let m_constraints = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_material = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };

        Ok(Rigidbody2D {
            parent,
            m_game_object,
            m_mass,
            m_use_auto_mass,
            m_body_type,
            m_linear_drag,
            m_simulated,
            m_angular_drag,
            m_use_full_kinematic_contacts,
            m_gravity_scale,
            m_fixed_angle,
            m_is_kinematic,
            m_interpolate,
            m_linear_damping,
            m_sleeping_mode,
            m_angular_damping,
            m_collision_detection,
            m_constraints,
            m_material,
            m_include_layers,
            m_exclude_layers,
        })
    }
}
