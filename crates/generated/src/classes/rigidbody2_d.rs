//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

/// 版本枚举：Rigidbody2DMInterpolateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Rigidbody2DMInterpolateVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for Rigidbody2DMInterpolateVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Rigidbody2DMInterpolateVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Rigidbody2DMSleepingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Rigidbody2DMSleepingModeVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for Rigidbody2DMSleepingModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Rigidbody2DMSleepingModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Rigidbody2DMCollisionDetectionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Rigidbody2DMCollisionDetectionVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for Rigidbody2DMCollisionDetectionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Rigidbody2DMCollisionDetectionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Rigidbody2D {
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
    pub m_interpolate: Rigidbody2DMInterpolateVariant,
    pub m_linear_damping: Option<f32>,
    pub m_sleeping_mode: Rigidbody2DMSleepingModeVariant,
    pub m_angular_damping: Option<f32>,
    pub m_collision_detection: Rigidbody2DMCollisionDetectionVariant,
    pub m_constraints: Option<i32>,
    pub m_material: Option<PPtr>,
    pub m_include_layers: Option<BitField>,
    pub m_exclude_layers: Option<BitField>,
}

impl crate::Parse for Rigidbody2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_mass = None;
        let mut m_use_auto_mass = None;
        let mut m_body_type = None;
        let mut m_linear_drag = None;
        let mut m_simulated = None;
        let mut m_angular_drag = None;
        let mut m_use_full_kinematic_contacts = None;
        let mut m_gravity_scale = None;
        let mut m_fixed_angle = None;
        let mut m_is_kinematic = None;
        let mut m_interpolate = None;
        let mut m_linear_damping = None;
        let mut m_sleeping_mode = None;
        let mut m_angular_damping = None;
        let mut m_collision_detection = None;
        let mut m_constraints = None;
        let mut m_material = None;
        let mut m_include_layers = None;
        let mut m_exclude_layers = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_body_type = Some(rdr.read_i32()?);
            m_simulated = Some(rdr.read_bool()?);
            m_use_full_kinematic_contacts = Some(rdr.read_bool()?);
            m_use_auto_mass = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_use_auto_mass = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_mass = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(6000, 0, 11) {
            m_linear_damping = Some(rdr.read_f32()?);
            m_angular_damping = Some(rdr.read_f32()?);
        } else {
            m_linear_drag = Some(rdr.read_f32()?);
            m_angular_drag = Some(rdr.read_f32()?);
        }
        m_gravity_scale = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_material = Some(crate::PPtr::parse(rdr, ver)?);
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_material = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_is_kinematic = Some(rdr.read_bool()?);
        } else {
            m_fixed_angle = Some(rdr.read_bool()?);
            m_is_kinematic = Some(rdr.read_bool()?);
        }
        m_interpolate = Some(Rigidbody2DMInterpolateVariant::parse(rdr, ver)?);
        m_sleeping_mode = Some(Rigidbody2DMSleepingModeVariant::parse(rdr, ver)?);
        m_collision_detection = Some(Rigidbody2DMCollisionDetectionVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_constraints = Some(rdr.read_i32()?);
        } else {
        }

        Ok(Rigidbody2D {
            m_game_object: m_game_object.unwrap(),
            m_mass: m_mass.unwrap(),
            m_use_auto_mass,
            m_body_type,
            m_linear_drag,
            m_simulated,
            m_angular_drag,
            m_use_full_kinematic_contacts,
            m_gravity_scale: m_gravity_scale.unwrap(),
            m_fixed_angle,
            m_is_kinematic,
            m_interpolate: m_interpolate.unwrap(),
            m_linear_damping,
            m_sleeping_mode: m_sleeping_mode.unwrap(),
            m_angular_damping,
            m_collision_detection: m_collision_detection.unwrap(),
            m_constraints,
            m_material,
            m_include_layers,
            m_exclude_layers,
        })
    }
}

impl crate::ToValue for Rigidbody2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Mass".to_string(), self.m_mass.to_value())),
            crate::opt_field("m_UseAutoMass", &self.m_use_auto_mass),
            crate::opt_field("m_BodyType", &self.m_body_type),
            crate::opt_field("m_LinearDrag", &self.m_linear_drag),
            crate::opt_field("m_Simulated", &self.m_simulated),
            crate::opt_field("m_AngularDrag", &self.m_angular_drag),
            crate::opt_field(
                "m_UseFullKinematicContacts",
                &self.m_use_full_kinematic_contacts,
            ),
            Some((
                "m_GravityScale".to_string(),
                self.m_gravity_scale.to_value(),
            )),
            crate::opt_field("m_FixedAngle", &self.m_fixed_angle),
            crate::opt_field("m_IsKinematic", &self.m_is_kinematic),
            Some(("m_Interpolate".to_string(), self.m_interpolate.to_value())),
            crate::opt_field("m_LinearDamping", &self.m_linear_damping),
            Some((
                "m_SleepingMode".to_string(),
                self.m_sleeping_mode.to_value(),
            )),
            crate::opt_field("m_AngularDamping", &self.m_angular_damping),
            Some((
                "m_CollisionDetection".to_string(),
                self.m_collision_detection.to_value(),
            )),
            crate::opt_field("m_Constraints", &self.m_constraints),
            crate::opt_field("m_Material", &self.m_material),
            crate::opt_field("m_IncludeLayers", &self.m_include_layers),
            crate::opt_field("m_ExcludeLayers", &self.m_exclude_layers),
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
                "m_Mass" => (self.m_mass).inspect_value(path, offset, limit),
                "m_UseAutoMass" => (self.m_use_auto_mass).inspect_value(path, offset, limit),
                "m_BodyType" => (self.m_body_type).inspect_value(path, offset, limit),
                "m_LinearDrag" => (self.m_linear_drag).inspect_value(path, offset, limit),
                "m_Simulated" => (self.m_simulated).inspect_value(path, offset, limit),
                "m_AngularDrag" => (self.m_angular_drag).inspect_value(path, offset, limit),
                "m_UseFullKinematicContacts" => {
                    (self.m_use_full_kinematic_contacts).inspect_value(path, offset, limit)
                }
                "m_GravityScale" => (self.m_gravity_scale).inspect_value(path, offset, limit),
                "m_FixedAngle" => (self.m_fixed_angle).inspect_value(path, offset, limit),
                "m_IsKinematic" => (self.m_is_kinematic).inspect_value(path, offset, limit),
                "m_Interpolate" => (self.m_interpolate).inspect_value(path, offset, limit),
                "m_LinearDamping" => (self.m_linear_damping).inspect_value(path, offset, limit),
                "m_SleepingMode" => (self.m_sleeping_mode).inspect_value(path, offset, limit),
                "m_AngularDamping" => (self.m_angular_damping).inspect_value(path, offset, limit),
                "m_CollisionDetection" => {
                    (self.m_collision_detection).inspect_value(path, offset, limit)
                }
                "m_Constraints" => (self.m_constraints).inspect_value(path, offset, limit),
                "m_Material" => (self.m_material).inspect_value(path, offset, limit),
                "m_IncludeLayers" => (self.m_include_layers).inspect_value(path, offset, limit),
                "m_ExcludeLayers" => (self.m_exclude_layers).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_mass).inspect_value(path, offset, limit)).map(|v| ("m_Mass".to_string(), v)),
            ((self.m_use_auto_mass).inspect_value(path, offset, limit))
                .map(|v| ("m_UseAutoMass".to_string(), v)),
            ((self.m_body_type).inspect_value(path, offset, limit))
                .map(|v| ("m_BodyType".to_string(), v)),
            ((self.m_linear_drag).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearDrag".to_string(), v)),
            ((self.m_simulated).inspect_value(path, offset, limit))
                .map(|v| ("m_Simulated".to_string(), v)),
            ((self.m_angular_drag).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularDrag".to_string(), v)),
            ((self.m_use_full_kinematic_contacts).inspect_value(path, offset, limit))
                .map(|v| ("m_UseFullKinematicContacts".to_string(), v)),
            ((self.m_gravity_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_GravityScale".to_string(), v)),
            ((self.m_fixed_angle).inspect_value(path, offset, limit))
                .map(|v| ("m_FixedAngle".to_string(), v)),
            ((self.m_is_kinematic).inspect_value(path, offset, limit))
                .map(|v| ("m_IsKinematic".to_string(), v)),
            ((self.m_interpolate).inspect_value(path, offset, limit))
                .map(|v| ("m_Interpolate".to_string(), v)),
            ((self.m_linear_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearDamping".to_string(), v)),
            ((self.m_sleeping_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_SleepingMode".to_string(), v)),
            ((self.m_angular_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularDamping".to_string(), v)),
            ((self.m_collision_detection).inspect_value(path, offset, limit))
                .map(|v| ("m_CollisionDetection".to_string(), v)),
            ((self.m_constraints).inspect_value(path, offset, limit))
                .map(|v| ("m_Constraints".to_string(), v)),
            ((self.m_material).inspect_value(path, offset, limit))
                .map(|v| ("m_Material".to_string(), v)),
            ((self.m_include_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_IncludeLayers".to_string(), v)),
            ((self.m_exclude_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ExcludeLayers".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
