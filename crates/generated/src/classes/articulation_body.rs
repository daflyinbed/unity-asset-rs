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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 3, 5) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2020_3_5(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ArticulationBodyMUseGravityVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_3_5(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
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
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_mass = Some(rdr.read_f32()?);
        m_parent_anchor_position = Some(Vector3f::parse(rdr, ver)?);
        m_parent_anchor_rotation = Some(Quaternionf::parse(rdr, ver)?);
        m_anchor_position = Some(Vector3f::parse(rdr, ver)?);
        m_anchor_rotation = Some(Quaternionf::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_match_anchors = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_compute_parent_anchor = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_articulation_joint_type = Some(rdr.read_i32()?);
        m_linear_x = Some(rdr.read_i32()?);
        m_linear_y = Some(rdr.read_i32()?);
        m_linear_z = Some(rdr.read_i32()?);
        m_swing_y = Some(rdr.read_i32()?);
        m_swing_z = Some(rdr.read_i32()?);
        m_twist = Some(rdr.read_i32()?);
        m_x_drive = Some(ArticulationDrive::parse(rdr, ver)?);
        m_y_drive = Some(ArticulationDrive::parse(rdr, ver)?);
        m_z_drive = Some(ArticulationDrive::parse(rdr, ver)?);
        m_linear_damping = Some(rdr.read_f32()?);
        m_angular_damping = Some(rdr.read_f32()?);
        m_joint_friction = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_center_of_mass = Some(Vector3f::parse(rdr, ver)?);
            m_inertia_tensor = Some(Vector3f::parse(rdr, ver)?);
            m_inertia_rotation = Some(Quaternionf::parse(rdr, ver)?);
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_implicit_com = Some(rdr.read_bool()?);
            m_implicit_tensor = Some(rdr.read_bool()?);
        } else {
        }
        m_immovable = Some(rdr.read_bool()?);
        m_use_gravity = Some(ArticulationBodyMUseGravityVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 3, 5) {
            m_collision_detection_mode = Some(rdr.read_i32()?);
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

impl crate::ToValue for ArticulationBody {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Mass".to_string(), self.m_mass.to_value())),
            Some((
                "m_ParentAnchorPosition".to_string(),
                self.m_parent_anchor_position.to_value(),
            )),
            Some((
                "m_ParentAnchorRotation".to_string(),
                self.m_parent_anchor_rotation.to_value(),
            )),
            Some((
                "m_AnchorPosition".to_string(),
                self.m_anchor_position.to_value(),
            )),
            Some((
                "m_AnchorRotation".to_string(),
                self.m_anchor_rotation.to_value(),
            )),
            crate::opt_field("m_ComputeParentAnchor", &self.m_compute_parent_anchor),
            crate::opt_field("m_MatchAnchors", &self.m_match_anchors),
            Some((
                "m_ArticulationJointType".to_string(),
                self.m_articulation_joint_type.to_value(),
            )),
            Some(("m_LinearX".to_string(), self.m_linear_x.to_value())),
            Some(("m_LinearY".to_string(), self.m_linear_y.to_value())),
            Some(("m_LinearZ".to_string(), self.m_linear_z.to_value())),
            Some(("m_SwingY".to_string(), self.m_swing_y.to_value())),
            Some(("m_SwingZ".to_string(), self.m_swing_z.to_value())),
            Some(("m_Twist".to_string(), self.m_twist.to_value())),
            Some(("m_XDrive".to_string(), self.m_x_drive.to_value())),
            Some(("m_YDrive".to_string(), self.m_y_drive.to_value())),
            Some(("m_ZDrive".to_string(), self.m_z_drive.to_value())),
            Some((
                "m_LinearDamping".to_string(),
                self.m_linear_damping.to_value(),
            )),
            Some((
                "m_AngularDamping".to_string(),
                self.m_angular_damping.to_value(),
            )),
            Some((
                "m_JointFriction".to_string(),
                self.m_joint_friction.to_value(),
            )),
            Some(("m_Immovable".to_string(), self.m_immovable.to_value())),
            crate::opt_field("m_CenterOfMass", &self.m_center_of_mass),
            Some(("m_UseGravity".to_string(), self.m_use_gravity.to_value())),
            crate::opt_field("m_InertiaTensor", &self.m_inertia_tensor),
            crate::opt_field("m_CollisionDetectionMode", &self.m_collision_detection_mode),
            crate::opt_field("m_InertiaRotation", &self.m_inertia_rotation),
            crate::opt_field("m_IncludeLayers", &self.m_include_layers),
            crate::opt_field("m_ExcludeLayers", &self.m_exclude_layers),
            crate::opt_field("m_ImplicitCom", &self.m_implicit_com),
            crate::opt_field("m_ImplicitTensor", &self.m_implicit_tensor),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Mass" => (self.m_mass).inspect_value(path, offset, limit),
                "m_ParentAnchorPosition" => {
                    (self.m_parent_anchor_position).inspect_value(path, offset, limit)
                }
                "m_ParentAnchorRotation" => {
                    (self.m_parent_anchor_rotation).inspect_value(path, offset, limit)
                }
                "m_AnchorPosition" => (self.m_anchor_position).inspect_value(path, offset, limit),
                "m_AnchorRotation" => (self.m_anchor_rotation).inspect_value(path, offset, limit),
                "m_ComputeParentAnchor" => {
                    (self.m_compute_parent_anchor).inspect_value(path, offset, limit)
                }
                "m_MatchAnchors" => (self.m_match_anchors).inspect_value(path, offset, limit),
                "m_ArticulationJointType" => {
                    (self.m_articulation_joint_type).inspect_value(path, offset, limit)
                }
                "m_LinearX" => (self.m_linear_x).inspect_value(path, offset, limit),
                "m_LinearY" => (self.m_linear_y).inspect_value(path, offset, limit),
                "m_LinearZ" => (self.m_linear_z).inspect_value(path, offset, limit),
                "m_SwingY" => (self.m_swing_y).inspect_value(path, offset, limit),
                "m_SwingZ" => (self.m_swing_z).inspect_value(path, offset, limit),
                "m_Twist" => (self.m_twist).inspect_value(path, offset, limit),
                "m_XDrive" => (self.m_x_drive).inspect_value(path, offset, limit),
                "m_YDrive" => (self.m_y_drive).inspect_value(path, offset, limit),
                "m_ZDrive" => (self.m_z_drive).inspect_value(path, offset, limit),
                "m_LinearDamping" => (self.m_linear_damping).inspect_value(path, offset, limit),
                "m_AngularDamping" => (self.m_angular_damping).inspect_value(path, offset, limit),
                "m_JointFriction" => (self.m_joint_friction).inspect_value(path, offset, limit),
                "m_Immovable" => (self.m_immovable).inspect_value(path, offset, limit),
                "m_CenterOfMass" => (self.m_center_of_mass).inspect_value(path, offset, limit),
                "m_UseGravity" => (self.m_use_gravity).inspect_value(path, offset, limit),
                "m_InertiaTensor" => (self.m_inertia_tensor).inspect_value(path, offset, limit),
                "m_CollisionDetectionMode" => {
                    (self.m_collision_detection_mode).inspect_value(path, offset, limit)
                }
                "m_InertiaRotation" => (self.m_inertia_rotation).inspect_value(path, offset, limit),
                "m_IncludeLayers" => (self.m_include_layers).inspect_value(path, offset, limit),
                "m_ExcludeLayers" => (self.m_exclude_layers).inspect_value(path, offset, limit),
                "m_ImplicitCom" => (self.m_implicit_com).inspect_value(path, offset, limit),
                "m_ImplicitTensor" => (self.m_implicit_tensor).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_mass).inspect_value(path, offset, limit)).map(|v| ("m_Mass".to_string(), v)),
            ((self.m_parent_anchor_position).inspect_value(path, offset, limit))
                .map(|v| ("m_ParentAnchorPosition".to_string(), v)),
            ((self.m_parent_anchor_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_ParentAnchorRotation".to_string(), v)),
            ((self.m_anchor_position).inspect_value(path, offset, limit))
                .map(|v| ("m_AnchorPosition".to_string(), v)),
            ((self.m_anchor_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_AnchorRotation".to_string(), v)),
            ((self.m_compute_parent_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ComputeParentAnchor".to_string(), v)),
            ((self.m_match_anchors).inspect_value(path, offset, limit))
                .map(|v| ("m_MatchAnchors".to_string(), v)),
            ((self.m_articulation_joint_type).inspect_value(path, offset, limit))
                .map(|v| ("m_ArticulationJointType".to_string(), v)),
            ((self.m_linear_x).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearX".to_string(), v)),
            ((self.m_linear_y).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearY".to_string(), v)),
            ((self.m_linear_z).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearZ".to_string(), v)),
            ((self.m_swing_y).inspect_value(path, offset, limit))
                .map(|v| ("m_SwingY".to_string(), v)),
            ((self.m_swing_z).inspect_value(path, offset, limit))
                .map(|v| ("m_SwingZ".to_string(), v)),
            ((self.m_twist).inspect_value(path, offset, limit)).map(|v| ("m_Twist".to_string(), v)),
            ((self.m_x_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_XDrive".to_string(), v)),
            ((self.m_y_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_YDrive".to_string(), v)),
            ((self.m_z_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_ZDrive".to_string(), v)),
            ((self.m_linear_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearDamping".to_string(), v)),
            ((self.m_angular_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularDamping".to_string(), v)),
            ((self.m_joint_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_JointFriction".to_string(), v)),
            ((self.m_immovable).inspect_value(path, offset, limit))
                .map(|v| ("m_Immovable".to_string(), v)),
            ((self.m_center_of_mass).inspect_value(path, offset, limit))
                .map(|v| ("m_CenterOfMass".to_string(), v)),
            ((self.m_use_gravity).inspect_value(path, offset, limit))
                .map(|v| ("m_UseGravity".to_string(), v)),
            ((self.m_inertia_tensor).inspect_value(path, offset, limit))
                .map(|v| ("m_InertiaTensor".to_string(), v)),
            ((self.m_collision_detection_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_CollisionDetectionMode".to_string(), v)),
            ((self.m_inertia_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_InertiaRotation".to_string(), v)),
            ((self.m_include_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_IncludeLayers".to_string(), v)),
            ((self.m_exclude_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ExcludeLayers".to_string(), v)),
            ((self.m_implicit_com).inspect_value(path, offset, limit))
                .map(|v| ("m_ImplicitCom".to_string(), v)),
            ((self.m_implicit_tensor).inspect_value(path, offset, limit))
                .map(|v| ("m_ImplicitTensor".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
