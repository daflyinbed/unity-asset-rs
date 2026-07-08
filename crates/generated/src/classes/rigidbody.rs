//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

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
    pub m_include_layers: Option<BitField>,
    pub m_collision_detection: i32,
    pub m_exclude_layers: Option<BitField>,
    pub m_implicit_com: Option<bool>,
    pub m_implicit_tensor: Option<bool>,
}

impl crate::Parse for Rigidbody {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_mass = None;
        let mut m_drag = None;
        let mut m_linear_damping = None;
        let mut m_angular_drag = None;
        let mut m_angular_damping = None;
        let mut m_use_gravity = None;
        let mut m_center_of_mass = None;
        let mut m_is_kinematic = None;
        let mut m_inertia_tensor = None;
        let mut m_interpolate = None;
        let mut m_inertia_rotation = None;
        let mut m_constraints = None;
        let mut m_include_layers = None;
        let mut m_collision_detection = None;
        let mut m_exclude_layers = None;
        let mut m_implicit_com = None;
        let mut m_implicit_tensor = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_mass = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_linear_damping = Some(r.read_f32()?);
            m_angular_damping = Some(r.read_f32()?);
            m_center_of_mass = Some(Vector3f::parse(r, ver)?);
            m_inertia_tensor = Some(Vector3f::parse(r, ver)?);
            m_inertia_rotation = Some(Quaternionf::parse(r, ver)?);
            m_include_layers = Some(BitField::parse(r, ver)?);
            m_exclude_layers = Some(BitField::parse(r, ver)?);
            m_implicit_com = Some(r.read_bool()?);
            m_implicit_tensor = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_drag = Some(r.read_f32()?);
            m_angular_drag = Some(r.read_f32()?);
            m_center_of_mass = Some(Vector3f::parse(r, ver)?);
            m_inertia_tensor = Some(Vector3f::parse(r, ver)?);
            m_inertia_rotation = Some(Quaternionf::parse(r, ver)?);
            m_include_layers = Some(BitField::parse(r, ver)?);
            m_exclude_layers = Some(BitField::parse(r, ver)?);
            m_implicit_com = Some(r.read_bool()?);
            m_implicit_tensor = Some(r.read_bool()?);
        } else {
            m_drag = Some(r.read_f32()?);
            m_angular_drag = Some(r.read_f32()?);
        }
            m_use_gravity = Some(r.read_bool()?);
            m_is_kinematic = Some(r.read_bool()?);
            m_interpolate = Some(r.read_u8()?);
            r.align(4)?;
            m_constraints = Some(r.read_i32()?);
            m_collision_detection = Some(r.read_i32()?);

        Ok(Rigidbody {
            m_game_object: m_game_object.unwrap(),
            m_mass: m_mass.unwrap(),
            m_drag,
            m_linear_damping,
            m_angular_drag,
            m_angular_damping,
            m_use_gravity: m_use_gravity.unwrap(),
            m_center_of_mass,
            m_is_kinematic: m_is_kinematic.unwrap(),
            m_inertia_tensor,
            m_interpolate: m_interpolate.unwrap(),
            m_inertia_rotation,
            m_constraints: m_constraints.unwrap(),
            m_include_layers,
            m_collision_detection: m_collision_detection.unwrap(),
            m_exclude_layers,
            m_implicit_com,
            m_implicit_tensor,
        })
    }
}
