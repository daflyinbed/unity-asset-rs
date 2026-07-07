//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::articulation_drive::ArticulationDrive;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ArticulationBody {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_mass: f32,
    pub m_parent_anchor_position: Vector3f,
    pub m_parent_anchor_rotation: Quaternionf,
    pub m_anchor_position: Vector3f,
    pub m_anchor_rotation: Quaternionf,
    pub m_compute_parent_anchor: Option<bool>,
    pub m_match_anchors: Option<bool>,
    pub m_articulation_joint_type: i32,
    pub m_linear_x: i32,
    pub m_linear_y: i32,
    pub m_linear_z: i32,
    pub m_swing_y: i32,
    pub m_swing_z: i32,
    pub m_twist: i32,
    pub m_x_drive: ArticulationDrive,
    pub m_y_drive: ArticulationDrive,
    pub m_z_drive: ArticulationDrive,
    pub m_linear_damping: f32,
    pub m_angular_damping: f32,
    pub m_joint_friction: f32,
    pub m_immovable: bool,
    pub m_center_of_mass: Option<Vector3f>,
    pub m_use_gravity: Option<bool>,
    pub m_inertia_tensor: Option<Vector3f>,
    pub m_collision_detection_mode: Option<i32>,
    pub m_inertia_rotation: Option<Quaternionf>,
    pub m_implicit_com: Option<bool>,
    pub m_include_layers: Option<BitField>,
    pub m_implicit_tensor: Option<bool>,
    pub m_exclude_layers: Option<BitField>,
}

impl crate::Parse for ArticulationBody {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_mass = r.read_f32()?;
        let m_parent_anchor_position = Vector3f::parse(r, ver)?;
        let m_parent_anchor_rotation = Quaternionf::parse(r, ver)?;
        let m_anchor_position = Vector3f::parse(r, ver)?;
        let m_anchor_rotation = Quaternionf::parse(r, ver)?;
        let m_compute_parent_anchor = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_match_anchors = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_articulation_joint_type = r.read_i32()?;
        let m_linear_x = r.read_i32()?;
        let m_linear_y = r.read_i32()?;
        let m_linear_z = r.read_i32()?;
        let m_swing_y = r.read_i32()?;
        let m_swing_z = r.read_i32()?;
        let m_twist = r.read_i32()?;
        let m_x_drive = ArticulationDrive::parse(r, ver)?;
        let m_y_drive = ArticulationDrive::parse(r, ver)?;
        let m_z_drive = ArticulationDrive::parse(r, ver)?;
        let m_linear_damping = r.read_f32()?;
        let m_angular_damping = r.read_f32()?;
        let m_joint_friction = r.read_f32()?;
        let m_immovable = r.read_bool()?;
        let m_center_of_mass = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_use_gravity = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_inertia_tensor = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_collision_detection_mode = if *ver >= crate::UnityVersion::new(2020, 3, 5) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_inertia_rotation = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(Quaternionf::parse(r, ver)?)
        } else {
            None
        };
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

        Ok(ArticulationBody {
            m_game_object,
            m_enabled,
            m_mass,
            m_parent_anchor_position,
            m_parent_anchor_rotation,
            m_anchor_position,
            m_anchor_rotation,
            m_compute_parent_anchor,
            m_match_anchors,
            m_articulation_joint_type,
            m_linear_x,
            m_linear_y,
            m_linear_z,
            m_swing_y,
            m_swing_z,
            m_twist,
            m_x_drive,
            m_y_drive,
            m_z_drive,
            m_linear_damping,
            m_angular_damping,
            m_joint_friction,
            m_immovable,
            m_center_of_mass,
            m_use_gravity,
            m_inertia_tensor,
            m_collision_detection_mode,
            m_inertia_rotation,
            m_implicit_com,
            m_include_layers,
            m_implicit_tensor,
            m_exclude_layers,
        })
    }
}
