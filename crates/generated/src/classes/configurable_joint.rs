//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::joint_drive::JointDrive;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::soft_joint_limit::SoftJointLimit;
use crate::subclasses::soft_joint_limit_spring::SoftJointLimitSpring;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：ConfigurableJointMConfiguredInWorldSpaceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ConfigurableJointMConfiguredInWorldSpaceVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for ConfigurableJointMConfiguredInWorldSpaceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ConfigurableJointMConfiguredInWorldSpaceVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ConfigurableJointMEnablePreprocessingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ConfigurableJointMEnablePreprocessingVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for ConfigurableJointMEnablePreprocessingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ConfigurableJointMEnablePreprocessingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ConfigurableJoint {
    pub m_game_object: PPtr,
    pub m_connected_body: PPtr,
    pub m_anchor: Vector3f,
    pub m_connected_articulation_body: Option<PPtr>,
    pub m_axis: Vector3f,
    pub m_secondary_axis: Vector3f,
    pub m_auto_configure_connected_anchor: Option<bool>,
    pub m_x_motion: i32,
    pub m_connected_anchor: Option<Vector3f>,
    pub m_y_motion: i32,
    pub m_z_motion: i32,
    pub m_angular_x_motion: i32,
    pub m_angular_y_motion: i32,
    pub m_angular_z_motion: i32,
    pub m_linear_limit: SoftJointLimit,
    pub m_low_angular_x_limit: SoftJointLimit,
    pub m_high_angular_x_limit: SoftJointLimit,
    pub m_linear_limit_spring: Option<SoftJointLimitSpring>,
    pub m_angular_y_limit: SoftJointLimit,
    pub m_angular_z_limit: SoftJointLimit,
    pub m_angular_x_limit_spring: Option<SoftJointLimitSpring>,
    pub m_target_position: Vector3f,
    pub m_target_velocity: Vector3f,
    pub m_x_drive: JointDrive,
    pub m_angular_yz_limit_spring: Option<SoftJointLimitSpring>,
    pub m_y_drive: JointDrive,
    pub m_z_drive: JointDrive,
    pub m_target_rotation: Quaternionf,
    pub m_target_angular_velocity: Vector3f,
    pub m_rotation_drive_mode: i32,
    pub m_angular_x_drive: JointDrive,
    pub m_angular_yz_drive: JointDrive,
    pub m_slerp_drive: JointDrive,
    pub m_projection_mode: i32,
    pub m_projection_distance: f32,
    pub m_projection_angle: f32,
    pub m_configured_in_world_space: ConfigurableJointMConfiguredInWorldSpaceVariant,
    pub m_break_force: f32,
    pub m_swap_bodies: Option<bool>,
    pub m_break_torque: f32,
    pub m_enable_collision: Option<bool>,
    pub m_enable_preprocessing: Option<ConfigurableJointMEnablePreprocessingVariant>,
    pub m_mass_scale: Option<f32>,
    pub m_connected_mass_scale: Option<f32>,
}

impl crate::Parse for ConfigurableJoint {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_connected_body = None;
        let mut m_anchor = None;
        let mut m_connected_articulation_body = None;
        let mut m_axis = None;
        let mut m_secondary_axis = None;
        let mut m_auto_configure_connected_anchor = None;
        let mut m_x_motion = None;
        let mut m_connected_anchor = None;
        let mut m_y_motion = None;
        let mut m_z_motion = None;
        let mut m_angular_x_motion = None;
        let mut m_angular_y_motion = None;
        let mut m_angular_z_motion = None;
        let mut m_linear_limit = None;
        let mut m_low_angular_x_limit = None;
        let mut m_high_angular_x_limit = None;
        let mut m_linear_limit_spring = None;
        let mut m_angular_y_limit = None;
        let mut m_angular_z_limit = None;
        let mut m_angular_x_limit_spring = None;
        let mut m_target_position = None;
        let mut m_target_velocity = None;
        let mut m_x_drive = None;
        let mut m_angular_yz_limit_spring = None;
        let mut m_y_drive = None;
        let mut m_z_drive = None;
        let mut m_target_rotation = None;
        let mut m_target_angular_velocity = None;
        let mut m_rotation_drive_mode = None;
        let mut m_angular_x_drive = None;
        let mut m_angular_yz_drive = None;
        let mut m_slerp_drive = None;
        let mut m_projection_mode = None;
        let mut m_projection_distance = None;
        let mut m_projection_angle = None;
        let mut m_configured_in_world_space = None;
        let mut m_break_force = None;
        let mut m_swap_bodies = None;
        let mut m_break_torque = None;
        let mut m_enable_collision = None;
        let mut m_enable_preprocessing = None;
        let mut m_mass_scale = None;
        let mut m_connected_mass_scale = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_connected_body = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_connected_articulation_body = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_anchor = Some(Vector3f::parse(rdr, ver)?);
        m_axis = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_auto_configure_connected_anchor = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_connected_anchor = Some(Vector3f::parse(rdr, ver)?);
        } else {
        }
        m_secondary_axis = Some(Vector3f::parse(rdr, ver)?);
        m_x_motion = Some(rdr.read_i32()?);
        m_y_motion = Some(rdr.read_i32()?);
        m_z_motion = Some(rdr.read_i32()?);
        m_angular_x_motion = Some(rdr.read_i32()?);
        m_angular_y_motion = Some(rdr.read_i32()?);
        m_angular_z_motion = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_linear_limit_spring = Some(SoftJointLimitSpring::parse(rdr, ver)?);
        } else {
        }
        m_linear_limit = Some(SoftJointLimit::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_angular_x_limit_spring = Some(SoftJointLimitSpring::parse(rdr, ver)?);
        } else {
        }
        m_low_angular_x_limit = Some(SoftJointLimit::parse(rdr, ver)?);
        m_high_angular_x_limit = Some(SoftJointLimit::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_angular_yz_limit_spring = Some(SoftJointLimitSpring::parse(rdr, ver)?);
        } else {
        }
        m_angular_y_limit = Some(SoftJointLimit::parse(rdr, ver)?);
        m_angular_z_limit = Some(SoftJointLimit::parse(rdr, ver)?);
        m_target_position = Some(Vector3f::parse(rdr, ver)?);
        m_target_velocity = Some(Vector3f::parse(rdr, ver)?);
        m_x_drive = Some(JointDrive::parse(rdr, ver)?);
        m_y_drive = Some(JointDrive::parse(rdr, ver)?);
        m_z_drive = Some(JointDrive::parse(rdr, ver)?);
        m_target_rotation = Some(Quaternionf::parse(rdr, ver)?);
        m_target_angular_velocity = Some(Vector3f::parse(rdr, ver)?);
        m_rotation_drive_mode = Some(rdr.read_i32()?);
        m_angular_x_drive = Some(JointDrive::parse(rdr, ver)?);
        m_angular_yz_drive = Some(JointDrive::parse(rdr, ver)?);
        m_slerp_drive = Some(JointDrive::parse(rdr, ver)?);
        m_projection_mode = Some(rdr.read_i32()?);
        m_projection_distance = Some(rdr.read_f32()?);
        m_projection_angle = Some(rdr.read_f32()?);
        m_configured_in_world_space = Some(ConfigurableJointMConfiguredInWorldSpaceVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_swap_bodies = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_break_force = Some(rdr.read_f32()?);
        m_break_torque = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
            m_enable_preprocessing = Some(ConfigurableJointMEnablePreprocessingVariant::parse(
                rdr, ver,
            )?);
            m_mass_scale = Some(rdr.read_f32()?);
            m_connected_mass_scale = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
            m_enable_preprocessing = Some(ConfigurableJointMEnablePreprocessingVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_enable_collision = Some(rdr.read_bool()?);
        } else {
        }

        Ok(ConfigurableJoint {
            m_game_object: m_game_object.unwrap(),
            m_connected_body: m_connected_body.unwrap(),
            m_anchor: m_anchor.unwrap(),
            m_connected_articulation_body,
            m_axis: m_axis.unwrap(),
            m_secondary_axis: m_secondary_axis.unwrap(),
            m_auto_configure_connected_anchor,
            m_x_motion: m_x_motion.unwrap(),
            m_connected_anchor,
            m_y_motion: m_y_motion.unwrap(),
            m_z_motion: m_z_motion.unwrap(),
            m_angular_x_motion: m_angular_x_motion.unwrap(),
            m_angular_y_motion: m_angular_y_motion.unwrap(),
            m_angular_z_motion: m_angular_z_motion.unwrap(),
            m_linear_limit: m_linear_limit.unwrap(),
            m_low_angular_x_limit: m_low_angular_x_limit.unwrap(),
            m_high_angular_x_limit: m_high_angular_x_limit.unwrap(),
            m_linear_limit_spring,
            m_angular_y_limit: m_angular_y_limit.unwrap(),
            m_angular_z_limit: m_angular_z_limit.unwrap(),
            m_angular_x_limit_spring,
            m_target_position: m_target_position.unwrap(),
            m_target_velocity: m_target_velocity.unwrap(),
            m_x_drive: m_x_drive.unwrap(),
            m_angular_yz_limit_spring,
            m_y_drive: m_y_drive.unwrap(),
            m_z_drive: m_z_drive.unwrap(),
            m_target_rotation: m_target_rotation.unwrap(),
            m_target_angular_velocity: m_target_angular_velocity.unwrap(),
            m_rotation_drive_mode: m_rotation_drive_mode.unwrap(),
            m_angular_x_drive: m_angular_x_drive.unwrap(),
            m_angular_yz_drive: m_angular_yz_drive.unwrap(),
            m_slerp_drive: m_slerp_drive.unwrap(),
            m_projection_mode: m_projection_mode.unwrap(),
            m_projection_distance: m_projection_distance.unwrap(),
            m_projection_angle: m_projection_angle.unwrap(),
            m_configured_in_world_space: m_configured_in_world_space.unwrap(),
            m_break_force: m_break_force.unwrap(),
            m_swap_bodies,
            m_break_torque: m_break_torque.unwrap(),
            m_enable_collision,
            m_enable_preprocessing,
            m_mass_scale,
            m_connected_mass_scale,
        })
    }
}

impl crate::ToValue for ConfigurableJoint {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some((
                "m_ConnectedBody".to_string(),
                self.m_connected_body.to_value(),
            )),
            Some(("m_Anchor".to_string(), self.m_anchor.to_value())),
            crate::opt_field(
                "m_ConnectedArticulationBody",
                &self.m_connected_articulation_body,
            ),
            Some(("m_Axis".to_string(), self.m_axis.to_value())),
            Some((
                "m_SecondaryAxis".to_string(),
                self.m_secondary_axis.to_value(),
            )),
            crate::opt_field(
                "m_AutoConfigureConnectedAnchor",
                &self.m_auto_configure_connected_anchor,
            ),
            Some(("m_XMotion".to_string(), self.m_x_motion.to_value())),
            crate::opt_field("m_ConnectedAnchor", &self.m_connected_anchor),
            Some(("m_YMotion".to_string(), self.m_y_motion.to_value())),
            Some(("m_ZMotion".to_string(), self.m_z_motion.to_value())),
            Some((
                "m_AngularXMotion".to_string(),
                self.m_angular_x_motion.to_value(),
            )),
            Some((
                "m_AngularYMotion".to_string(),
                self.m_angular_y_motion.to_value(),
            )),
            Some((
                "m_AngularZMotion".to_string(),
                self.m_angular_z_motion.to_value(),
            )),
            Some(("m_LinearLimit".to_string(), self.m_linear_limit.to_value())),
            Some((
                "m_LowAngularXLimit".to_string(),
                self.m_low_angular_x_limit.to_value(),
            )),
            Some((
                "m_HighAngularXLimit".to_string(),
                self.m_high_angular_x_limit.to_value(),
            )),
            crate::opt_field("m_LinearLimitSpring", &self.m_linear_limit_spring),
            Some((
                "m_AngularYLimit".to_string(),
                self.m_angular_y_limit.to_value(),
            )),
            Some((
                "m_AngularZLimit".to_string(),
                self.m_angular_z_limit.to_value(),
            )),
            crate::opt_field("m_AngularXLimitSpring", &self.m_angular_x_limit_spring),
            Some((
                "m_TargetPosition".to_string(),
                self.m_target_position.to_value(),
            )),
            Some((
                "m_TargetVelocity".to_string(),
                self.m_target_velocity.to_value(),
            )),
            Some(("m_XDrive".to_string(), self.m_x_drive.to_value())),
            crate::opt_field("m_AngularYZLimitSpring", &self.m_angular_yz_limit_spring),
            Some(("m_YDrive".to_string(), self.m_y_drive.to_value())),
            Some(("m_ZDrive".to_string(), self.m_z_drive.to_value())),
            Some((
                "m_TargetRotation".to_string(),
                self.m_target_rotation.to_value(),
            )),
            Some((
                "m_TargetAngularVelocity".to_string(),
                self.m_target_angular_velocity.to_value(),
            )),
            Some((
                "m_RotationDriveMode".to_string(),
                self.m_rotation_drive_mode.to_value(),
            )),
            Some((
                "m_AngularXDrive".to_string(),
                self.m_angular_x_drive.to_value(),
            )),
            Some((
                "m_AngularYZDrive".to_string(),
                self.m_angular_yz_drive.to_value(),
            )),
            Some(("m_SlerpDrive".to_string(), self.m_slerp_drive.to_value())),
            Some((
                "m_ProjectionMode".to_string(),
                self.m_projection_mode.to_value(),
            )),
            Some((
                "m_ProjectionDistance".to_string(),
                self.m_projection_distance.to_value(),
            )),
            Some((
                "m_ProjectionAngle".to_string(),
                self.m_projection_angle.to_value(),
            )),
            Some((
                "m_ConfiguredInWorldSpace".to_string(),
                self.m_configured_in_world_space.to_value(),
            )),
            Some(("m_BreakForce".to_string(), self.m_break_force.to_value())),
            crate::opt_field("m_SwapBodies", &self.m_swap_bodies),
            Some(("m_BreakTorque".to_string(), self.m_break_torque.to_value())),
            crate::opt_field("m_EnableCollision", &self.m_enable_collision),
            crate::opt_field("m_EnablePreprocessing", &self.m_enable_preprocessing),
            crate::opt_field("m_MassScale", &self.m_mass_scale),
            crate::opt_field("m_ConnectedMassScale", &self.m_connected_mass_scale),
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
                "m_ConnectedBody" => (self.m_connected_body).inspect_value(path, offset, limit),
                "m_Anchor" => (self.m_anchor).inspect_value(path, offset, limit),
                "m_ConnectedArticulationBody" => {
                    (self.m_connected_articulation_body).inspect_value(path, offset, limit)
                }
                "m_Axis" => (self.m_axis).inspect_value(path, offset, limit),
                "m_SecondaryAxis" => (self.m_secondary_axis).inspect_value(path, offset, limit),
                "m_AutoConfigureConnectedAnchor" => {
                    (self.m_auto_configure_connected_anchor).inspect_value(path, offset, limit)
                }
                "m_XMotion" => (self.m_x_motion).inspect_value(path, offset, limit),
                "m_ConnectedAnchor" => (self.m_connected_anchor).inspect_value(path, offset, limit),
                "m_YMotion" => (self.m_y_motion).inspect_value(path, offset, limit),
                "m_ZMotion" => (self.m_z_motion).inspect_value(path, offset, limit),
                "m_AngularXMotion" => (self.m_angular_x_motion).inspect_value(path, offset, limit),
                "m_AngularYMotion" => (self.m_angular_y_motion).inspect_value(path, offset, limit),
                "m_AngularZMotion" => (self.m_angular_z_motion).inspect_value(path, offset, limit),
                "m_LinearLimit" => (self.m_linear_limit).inspect_value(path, offset, limit),
                "m_LowAngularXLimit" => {
                    (self.m_low_angular_x_limit).inspect_value(path, offset, limit)
                }
                "m_HighAngularXLimit" => {
                    (self.m_high_angular_x_limit).inspect_value(path, offset, limit)
                }
                "m_LinearLimitSpring" => {
                    (self.m_linear_limit_spring).inspect_value(path, offset, limit)
                }
                "m_AngularYLimit" => (self.m_angular_y_limit).inspect_value(path, offset, limit),
                "m_AngularZLimit" => (self.m_angular_z_limit).inspect_value(path, offset, limit),
                "m_AngularXLimitSpring" => {
                    (self.m_angular_x_limit_spring).inspect_value(path, offset, limit)
                }
                "m_TargetPosition" => (self.m_target_position).inspect_value(path, offset, limit),
                "m_TargetVelocity" => (self.m_target_velocity).inspect_value(path, offset, limit),
                "m_XDrive" => (self.m_x_drive).inspect_value(path, offset, limit),
                "m_AngularYZLimitSpring" => {
                    (self.m_angular_yz_limit_spring).inspect_value(path, offset, limit)
                }
                "m_YDrive" => (self.m_y_drive).inspect_value(path, offset, limit),
                "m_ZDrive" => (self.m_z_drive).inspect_value(path, offset, limit),
                "m_TargetRotation" => (self.m_target_rotation).inspect_value(path, offset, limit),
                "m_TargetAngularVelocity" => {
                    (self.m_target_angular_velocity).inspect_value(path, offset, limit)
                }
                "m_RotationDriveMode" => {
                    (self.m_rotation_drive_mode).inspect_value(path, offset, limit)
                }
                "m_AngularXDrive" => (self.m_angular_x_drive).inspect_value(path, offset, limit),
                "m_AngularYZDrive" => (self.m_angular_yz_drive).inspect_value(path, offset, limit),
                "m_SlerpDrive" => (self.m_slerp_drive).inspect_value(path, offset, limit),
                "m_ProjectionMode" => (self.m_projection_mode).inspect_value(path, offset, limit),
                "m_ProjectionDistance" => {
                    (self.m_projection_distance).inspect_value(path, offset, limit)
                }
                "m_ProjectionAngle" => (self.m_projection_angle).inspect_value(path, offset, limit),
                "m_ConfiguredInWorldSpace" => {
                    (self.m_configured_in_world_space).inspect_value(path, offset, limit)
                }
                "m_BreakForce" => (self.m_break_force).inspect_value(path, offset, limit),
                "m_SwapBodies" => (self.m_swap_bodies).inspect_value(path, offset, limit),
                "m_BreakTorque" => (self.m_break_torque).inspect_value(path, offset, limit),
                "m_EnableCollision" => (self.m_enable_collision).inspect_value(path, offset, limit),
                "m_EnablePreprocessing" => {
                    (self.m_enable_preprocessing).inspect_value(path, offset, limit)
                }
                "m_MassScale" => (self.m_mass_scale).inspect_value(path, offset, limit),
                "m_ConnectedMassScale" => {
                    (self.m_connected_mass_scale).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_connected_body).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedBody".to_string(), v)),
            ((self.m_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_Anchor".to_string(), v)),
            ((self.m_connected_articulation_body).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedArticulationBody".to_string(), v)),
            ((self.m_axis).inspect_value(path, offset, limit)).map(|v| ("m_Axis".to_string(), v)),
            ((self.m_secondary_axis).inspect_value(path, offset, limit))
                .map(|v| ("m_SecondaryAxis".to_string(), v)),
            ((self.m_auto_configure_connected_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoConfigureConnectedAnchor".to_string(), v)),
            ((self.m_x_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_XMotion".to_string(), v)),
            ((self.m_connected_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedAnchor".to_string(), v)),
            ((self.m_y_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_YMotion".to_string(), v)),
            ((self.m_z_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_ZMotion".to_string(), v)),
            ((self.m_angular_x_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularXMotion".to_string(), v)),
            ((self.m_angular_y_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularYMotion".to_string(), v)),
            ((self.m_angular_z_motion).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularZMotion".to_string(), v)),
            ((self.m_linear_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearLimit".to_string(), v)),
            ((self.m_low_angular_x_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_LowAngularXLimit".to_string(), v)),
            ((self.m_high_angular_x_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_HighAngularXLimit".to_string(), v)),
            ((self.m_linear_limit_spring).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearLimitSpring".to_string(), v)),
            ((self.m_angular_y_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularYLimit".to_string(), v)),
            ((self.m_angular_z_limit).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularZLimit".to_string(), v)),
            ((self.m_angular_x_limit_spring).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularXLimitSpring".to_string(), v)),
            ((self.m_target_position).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetPosition".to_string(), v)),
            ((self.m_target_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetVelocity".to_string(), v)),
            ((self.m_x_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_XDrive".to_string(), v)),
            ((self.m_angular_yz_limit_spring).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularYZLimitSpring".to_string(), v)),
            ((self.m_y_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_YDrive".to_string(), v)),
            ((self.m_z_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_ZDrive".to_string(), v)),
            ((self.m_target_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetRotation".to_string(), v)),
            ((self.m_target_angular_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetAngularVelocity".to_string(), v)),
            ((self.m_rotation_drive_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RotationDriveMode".to_string(), v)),
            ((self.m_angular_x_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularXDrive".to_string(), v)),
            ((self.m_angular_yz_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularYZDrive".to_string(), v)),
            ((self.m_slerp_drive).inspect_value(path, offset, limit))
                .map(|v| ("m_SlerpDrive".to_string(), v)),
            ((self.m_projection_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ProjectionMode".to_string(), v)),
            ((self.m_projection_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_ProjectionDistance".to_string(), v)),
            ((self.m_projection_angle).inspect_value(path, offset, limit))
                .map(|v| ("m_ProjectionAngle".to_string(), v)),
            ((self.m_configured_in_world_space).inspect_value(path, offset, limit))
                .map(|v| ("m_ConfiguredInWorldSpace".to_string(), v)),
            ((self.m_break_force).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakForce".to_string(), v)),
            ((self.m_swap_bodies).inspect_value(path, offset, limit))
                .map(|v| ("m_SwapBodies".to_string(), v)),
            ((self.m_break_torque).inspect_value(path, offset, limit))
                .map(|v| ("m_BreakTorque".to_string(), v)),
            ((self.m_enable_collision).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableCollision".to_string(), v)),
            ((self.m_enable_preprocessing).inspect_value(path, offset, limit))
                .map(|v| ("m_EnablePreprocessing".to_string(), v)),
            ((self.m_mass_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_MassScale".to_string(), v)),
            ((self.m_connected_mass_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_ConnectedMassScale".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
