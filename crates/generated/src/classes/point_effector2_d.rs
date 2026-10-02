//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

/// 版本枚举：PointEffector2DMForceSourceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PointEffector2DMForceSourceVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for PointEffector2DMForceSourceVariant {
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

impl crate::ToValue for PointEffector2DMForceSourceVariant {
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

/// 版本枚举：PointEffector2DMForceTargetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PointEffector2DMForceTargetVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for PointEffector2DMForceTargetVariant {
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

impl crate::ToValue for PointEffector2DMForceTargetVariant {
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

/// 版本枚举：PointEffector2DMForceModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PointEffector2DMForceModeVariant {
    Default(u8),
    V5_5(i32),
}

impl crate::Parse for PointEffector2DMForceModeVariant {
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

impl crate::ToValue for PointEffector2DMForceModeVariant {
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
pub struct PointEffector2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_force_magnitude: f32,
    pub m_force_variation: f32,
    pub m_distance_scale: f32,
    pub m_drag: Option<f32>,
    pub m_angular_drag: Option<f32>,
    pub m_force_source: PointEffector2DMForceSourceVariant,
    pub m_force_target: PointEffector2DMForceTargetVariant,
    pub m_force_mode: PointEffector2DMForceModeVariant,
    pub m_linear_damping: Option<f32>,
    pub m_angular_damping: Option<f32>,
}

impl crate::Parse for PointEffector2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_collider_mask = None;
        let mut m_use_collider_mask = None;
        let mut m_force_magnitude = None;
        let mut m_force_variation = None;
        let mut m_distance_scale = None;
        let mut m_drag = None;
        let mut m_angular_drag = None;
        let mut m_force_source = None;
        let mut m_force_target = None;
        let mut m_force_mode = None;
        let mut m_linear_damping = None;
        let mut m_angular_damping = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 2) {
            m_use_collider_mask = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_collider_mask = Some(BitField::parse(rdr, ver)?);
        m_force_magnitude = Some(rdr.read_f32()?);
        m_force_variation = Some(rdr.read_f32()?);
        m_distance_scale = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_drag = Some(rdr.read_f32()?);
            m_angular_drag = Some(rdr.read_f32()?);
        }
        m_force_source = Some(PointEffector2DMForceSourceVariant::parse(rdr, ver)?);
        m_force_target = Some(PointEffector2DMForceTargetVariant::parse(rdr, ver)?);
        m_force_mode = Some(PointEffector2DMForceModeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_linear_damping = Some(rdr.read_f32()?);
            m_angular_damping = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_drag = Some(rdr.read_f32()?);
            m_angular_drag = Some(rdr.read_f32()?);
        } else {
        }

        Ok(PointEffector2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collider_mask: m_collider_mask.unwrap(),
            m_use_collider_mask,
            m_force_magnitude: m_force_magnitude.unwrap(),
            m_force_variation: m_force_variation.unwrap(),
            m_distance_scale: m_distance_scale.unwrap(),
            m_drag,
            m_angular_drag,
            m_force_source: m_force_source.unwrap(),
            m_force_target: m_force_target.unwrap(),
            m_force_mode: m_force_mode.unwrap(),
            m_linear_damping,
            m_angular_damping,
        })
    }
}

impl crate::ToValue for PointEffector2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_ColliderMask".to_string(),
                self.m_collider_mask.to_value(),
            )),
            crate::opt_field("m_UseColliderMask", &self.m_use_collider_mask),
            Some((
                "m_ForceMagnitude".to_string(),
                self.m_force_magnitude.to_value(),
            )),
            Some((
                "m_ForceVariation".to_string(),
                self.m_force_variation.to_value(),
            )),
            Some((
                "m_DistanceScale".to_string(),
                self.m_distance_scale.to_value(),
            )),
            crate::opt_field("m_Drag", &self.m_drag),
            crate::opt_field("m_AngularDrag", &self.m_angular_drag),
            Some(("m_ForceSource".to_string(), self.m_force_source.to_value())),
            Some(("m_ForceTarget".to_string(), self.m_force_target.to_value())),
            Some(("m_ForceMode".to_string(), self.m_force_mode.to_value())),
            crate::opt_field("m_LinearDamping", &self.m_linear_damping),
            crate::opt_field("m_AngularDamping", &self.m_angular_damping),
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
                "m_ColliderMask" => (self.m_collider_mask).inspect_value(path, offset, limit),
                "m_UseColliderMask" => {
                    (self.m_use_collider_mask).inspect_value(path, offset, limit)
                }
                "m_ForceMagnitude" => (self.m_force_magnitude).inspect_value(path, offset, limit),
                "m_ForceVariation" => (self.m_force_variation).inspect_value(path, offset, limit),
                "m_DistanceScale" => (self.m_distance_scale).inspect_value(path, offset, limit),
                "m_Drag" => (self.m_drag).inspect_value(path, offset, limit),
                "m_AngularDrag" => (self.m_angular_drag).inspect_value(path, offset, limit),
                "m_ForceSource" => (self.m_force_source).inspect_value(path, offset, limit),
                "m_ForceTarget" => (self.m_force_target).inspect_value(path, offset, limit),
                "m_ForceMode" => (self.m_force_mode).inspect_value(path, offset, limit),
                "m_LinearDamping" => (self.m_linear_damping).inspect_value(path, offset, limit),
                "m_AngularDamping" => (self.m_angular_damping).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_collider_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderMask".to_string(), v)),
            ((self.m_use_collider_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_UseColliderMask".to_string(), v)),
            ((self.m_force_magnitude).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceMagnitude".to_string(), v)),
            ((self.m_force_variation).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceVariation".to_string(), v)),
            ((self.m_distance_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_DistanceScale".to_string(), v)),
            ((self.m_drag).inspect_value(path, offset, limit)).map(|v| ("m_Drag".to_string(), v)),
            ((self.m_angular_drag).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularDrag".to_string(), v)),
            ((self.m_force_source).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceSource".to_string(), v)),
            ((self.m_force_target).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceTarget".to_string(), v)),
            ((self.m_force_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceMode".to_string(), v)),
            ((self.m_linear_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearDamping".to_string(), v)),
            ((self.m_angular_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularDamping".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
