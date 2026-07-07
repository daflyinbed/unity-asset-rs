//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::classes::vector3f::Vector3f;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::quaternionf::Quaternionf;

#[derive(Debug, Clone)]
pub struct Rigidbody {
    pub m_game_object: PPtr,
    pub m_mass: f32,
    pub m_drag: Option<f32>,
    pub m_linear_damping: Option<f32>,
    pub m_angular_drag: Option<f32>,
    pub m_angular_damping: Option<f32>,
    pub m_use_gravity: bool,
    pub m_center_of_mass: Option<Vector3f>,
    pub m_is_kinematic: bool,
    pub m_inertia_tensor: Option<Vector3f>,
    pub m_interpolate: u8,
    pub m_inertia_rotation: Option<Quaternionf>,
    pub m_constraints: i32,
    pub m_implicit_com: Option<bool>,
    pub m_include_layers: Option<BitField>,
    pub m_collision_detection: i32,
    pub m_implicit_tensor: Option<bool>,
    pub m_exclude_layers: Option<BitField>,
}

impl crate::Parse for Rigidbody {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_mass = r.read_f32()?;
        let m_drag = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_linear_damping = if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_angular_drag = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_angular_damping = if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use_gravity = r.read_bool()?;
        let m_center_of_mass = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_is_kinematic = r.read_bool()?;
        let m_inertia_tensor = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_interpolate = r.read_u8()?;
        r.align(4)?;
        let m_inertia_rotation = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(Quaternionf::parse(r, ver)?)
        } else {
            None
        };
        let m_constraints = r.read_i32()?;
        let m_implicit_com = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_collision_detection = r.read_i32()?;
        let m_implicit_tensor = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };

        Ok(Rigidbody {
            m_game_object,
            m_mass,
            m_drag,
            m_linear_damping,
            m_angular_drag,
            m_angular_damping,
            m_use_gravity,
            m_center_of_mass,
            m_is_kinematic,
            m_inertia_tensor,
            m_interpolate,
            m_inertia_rotation,
            m_constraints,
            m_implicit_com,
            m_include_layers,
            m_collision_detection,
            m_implicit_tensor,
            m_exclude_layers,
        })
    }
}
