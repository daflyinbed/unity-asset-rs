//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::articulation_drive::ArticulationDrive;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：ArticulationBodyMUseGravityVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ArticulationBodyMUseGravityVariant {
    Default(bool),
    V2020_3_5(bool),
}

impl crate::Parse for ArticulationBodyMUseGravityVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 3, 5) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2020_3_5(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
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
    pub m_use_gravity: ArticulationBodyMUseGravityVariant,
    pub m_inertia_tensor: Option<Vector3f>,
    pub m_collision_detection_mode: Option<i32>,
    pub m_inertia_rotation: Option<Quaternionf>,
    pub m_include_layers: Option<BitField>,
    pub m_exclude_layers: Option<BitField>,
    pub m_implicit_com: Option<bool>,
    pub m_implicit_tensor: Option<bool>,
}

impl crate::Parse for ArticulationBody {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_mass = None;
        let mut m_parent_anchor_position = None;
        let mut m_parent_anchor_rotation = None;
        let mut m_anchor_position = None;
        let mut m_anchor_rotation = None;
        let mut m_compute_parent_anchor = None;
        let mut m_match_anchors = None;
        let mut m_articulation_joint_type = None;
        let mut m_linear_x = None;
        let mut m_linear_y = None;
        let mut m_linear_z = None;
        let mut m_swing_y = None;
        let mut m_swing_z = None;
        let mut m_twist = None;
        let mut m_x_drive = None;
        let mut m_y_drive = None;
        let mut m_z_drive = None;
        let mut m_linear_damping = None;
        let mut m_angular_damping = None;
        let mut m_joint_friction = None;
        let mut m_immovable = None;
        let mut m_center_of_mass = None;
        let mut m_use_gravity = None;
        let mut m_inertia_tensor = None;
        let mut m_collision_detection_mode = None;
        let mut m_inertia_rotation = None;
        let mut m_include_layers = None;
        let mut m_exclude_layers = None;
        let mut m_implicit_com = None;
        let mut m_implicit_tensor = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
            m_mass = Some(r.read_f32()?);
            m_parent_anchor_position = Some(Vector3f::parse(r, ver)?);
            m_parent_anchor_rotation = Some(Quaternionf::parse(r, ver)?);
            m_anchor_position = Some(Vector3f::parse(r, ver)?);
            m_anchor_rotation = Some(Quaternionf::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_match_anchors = Some(r.read_bool()?);
            r.align(4)?;
        } else {
            m_compute_parent_anchor = Some(r.read_bool()?);
            r.align(4)?;
        }
            m_articulation_joint_type = Some(r.read_i32()?);
            m_linear_x = Some(r.read_i32()?);
            m_linear_y = Some(r.read_i32()?);
            m_linear_z = Some(r.read_i32()?);
            m_swing_y = Some(r.read_i32()?);
            m_swing_z = Some(r.read_i32()?);
            m_twist = Some(r.read_i32()?);
            m_x_drive = Some(ArticulationDrive::parse(r, ver)?);
            m_y_drive = Some(ArticulationDrive::parse(r, ver)?);
            m_z_drive = Some(ArticulationDrive::parse(r, ver)?);
            m_linear_damping = Some(r.read_f32()?);
            m_angular_damping = Some(r.read_f32()?);
            m_joint_friction = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_center_of_mass = Some(Vector3f::parse(r, ver)?);
            m_inertia_tensor = Some(Vector3f::parse(r, ver)?);
            m_inertia_rotation = Some(Quaternionf::parse(r, ver)?);
            m_include_layers = Some(BitField::parse(r, ver)?);
            m_exclude_layers = Some(BitField::parse(r, ver)?);
            m_implicit_com = Some(r.read_bool()?);
            m_implicit_tensor = Some(r.read_bool()?);
        } else {
        }
            m_immovable = Some(r.read_bool()?);
            m_use_gravity = Some(ArticulationBodyMUseGravityVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 3, 5) {
            m_collision_detection_mode = Some(r.read_i32()?);
        } else {
        }

        Ok(ArticulationBody {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_mass: m_mass.unwrap(),
            m_parent_anchor_position: m_parent_anchor_position.unwrap(),
            m_parent_anchor_rotation: m_parent_anchor_rotation.unwrap(),
            m_anchor_position: m_anchor_position.unwrap(),
            m_anchor_rotation: m_anchor_rotation.unwrap(),
            m_compute_parent_anchor,
            m_match_anchors,
            m_articulation_joint_type: m_articulation_joint_type.unwrap(),
            m_linear_x: m_linear_x.unwrap(),
            m_linear_y: m_linear_y.unwrap(),
            m_linear_z: m_linear_z.unwrap(),
            m_swing_y: m_swing_y.unwrap(),
            m_swing_z: m_swing_z.unwrap(),
            m_twist: m_twist.unwrap(),
            m_x_drive: m_x_drive.unwrap(),
            m_y_drive: m_y_drive.unwrap(),
            m_z_drive: m_z_drive.unwrap(),
            m_linear_damping: m_linear_damping.unwrap(),
            m_angular_damping: m_angular_damping.unwrap(),
            m_joint_friction: m_joint_friction.unwrap(),
            m_immovable: m_immovable.unwrap(),
            m_center_of_mass,
            m_use_gravity: m_use_gravity.unwrap(),
            m_inertia_tensor,
            m_collision_detection_mode,
            m_inertia_rotation,
            m_include_layers,
            m_exclude_layers,
            m_implicit_com,
            m_implicit_tensor,
        })
    }
}
