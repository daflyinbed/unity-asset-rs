//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::min_max_gradient::MinMaxGradient;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：InitialModuleGravityModifierVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum InitialModuleGravityModifierVariant {
    Default(f32),
    V5_5(Box<MinMaxCurve>),
}

impl crate::Parse for InitialModuleGravityModifierVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ({ Ok(Box::new(MinMaxCurve::parse(rdr, ver)?)) })?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for InitialModuleGravityModifierVariant {
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
pub struct InitialModule {
    pub enabled: bool,
    pub start_lifetime: MinMaxCurve,
    pub start_speed: MinMaxCurve,
    pub start_color: MinMaxGradient,
    pub start_size: MinMaxCurve,
    pub start_rotation: MinMaxCurve,
    pub start_rotation_x: Option<MinMaxCurve>,
    pub start_size_y: Option<MinMaxCurve>,
    pub gravity_modifier: InitialModuleGravityModifierVariant,
    pub start_rotation_y: Option<MinMaxCurve>,
    pub start_size_z: Option<MinMaxCurve>,
    pub inherit_velocity: Option<f32>,
    pub max_num_particles: i32,
    pub randomize_rotation_direction: Option<f32>,
    pub rotation3_d: Option<bool>,
    pub gravity_source: Option<i32>,
    pub size3_d: Option<bool>,
    pub custom_emitter_velocity: Option<Vector3f>,
}

impl crate::Parse for InitialModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut start_lifetime = None;
        let mut start_speed = None;
        let mut start_color = None;
        let mut start_size = None;
        let mut start_rotation = None;
        let mut start_rotation_x = None;
        let mut start_size_y = None;
        let mut gravity_modifier = None;
        let mut start_rotation_y = None;
        let mut start_size_z = None;
        let mut inherit_velocity = None;
        let mut max_num_particles = None;
        let mut randomize_rotation_direction = None;
        let mut rotation3_d = None;
        let mut gravity_source = None;
        let mut size3_d = None;
        let mut custom_emitter_velocity = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        start_lifetime = Some(MinMaxCurve::parse(rdr, ver)?);
        start_speed = Some(MinMaxCurve::parse(rdr, ver)?);
        start_color = Some(MinMaxGradient::parse(rdr, ver)?);
        start_size = Some(MinMaxCurve::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            start_size_y = Some(MinMaxCurve::parse(rdr, ver)?);
            start_size_z = Some(MinMaxCurve::parse(rdr, ver)?);
            start_rotation_x = Some(MinMaxCurve::parse(rdr, ver)?);
            start_rotation_y = Some(MinMaxCurve::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            start_rotation_x = Some(MinMaxCurve::parse(rdr, ver)?);
            start_rotation_y = Some(MinMaxCurve::parse(rdr, ver)?);
        } else {
        }
        start_rotation = Some(MinMaxCurve::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            randomize_rotation_direction = Some(rdr.read_f32()?);
            gravity_source = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            randomize_rotation_direction = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            randomize_rotation_direction = Some(rdr.read_f32()?);
            gravity_modifier = Some(InitialModuleGravityModifierVariant::parse(rdr, ver)?);
        } else {
            gravity_modifier = Some(InitialModuleGravityModifierVariant::parse(rdr, ver)?);
            inherit_velocity = Some(rdr.read_f32()?);
        }
        max_num_particles = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            custom_emitter_velocity = Some(Vector3f::parse(rdr, ver)?);
            size3_d = Some(rdr.read_bool()?);
            rotation3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
            gravity_modifier = Some(InitialModuleGravityModifierVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            size3_d = Some(rdr.read_bool()?);
            rotation3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
            gravity_modifier = Some(InitialModuleGravityModifierVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            size3_d = Some(rdr.read_bool()?);
            rotation3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            rotation3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(InitialModule {
            enabled: enabled.unwrap(),
            start_lifetime: start_lifetime.unwrap(),
            start_speed: start_speed.unwrap(),
            start_color: start_color.unwrap(),
            start_size: start_size.unwrap(),
            start_rotation: start_rotation.unwrap(),
            start_rotation_x,
            start_size_y,
            gravity_modifier: gravity_modifier.unwrap(),
            start_rotation_y,
            start_size_z,
            inherit_velocity,
            max_num_particles: max_num_particles.unwrap(),
            randomize_rotation_direction,
            rotation3_d,
            gravity_source,
            size3_d,
            custom_emitter_velocity,
        })
    }
}

impl crate::ToValue for InitialModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("startLifetime".to_string(), self.start_lifetime.to_value())),
            Some(("startSpeed".to_string(), self.start_speed.to_value())),
            Some(("startColor".to_string(), self.start_color.to_value())),
            Some(("startSize".to_string(), self.start_size.to_value())),
            Some(("startRotation".to_string(), self.start_rotation.to_value())),
            crate::opt_field("startRotationX", &self.start_rotation_x),
            crate::opt_field("startSizeY", &self.start_size_y),
            Some((
                "gravityModifier".to_string(),
                self.gravity_modifier.to_value(),
            )),
            crate::opt_field("startRotationY", &self.start_rotation_y),
            crate::opt_field("startSizeZ", &self.start_size_z),
            crate::opt_field("inheritVelocity", &self.inherit_velocity),
            Some((
                "maxNumParticles".to_string(),
                self.max_num_particles.to_value(),
            )),
            crate::opt_field(
                "randomizeRotationDirection",
                &self.randomize_rotation_direction,
            ),
            crate::opt_field("rotation3D", &self.rotation3_d),
            crate::opt_field("gravitySource", &self.gravity_source),
            crate::opt_field("size3D", &self.size3_d),
            crate::opt_field("customEmitterVelocity", &self.custom_emitter_velocity),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "startLifetime" => (self.start_lifetime).inspect_value(path, offset, limit),
                "startSpeed" => (self.start_speed).inspect_value(path, offset, limit),
                "startColor" => (self.start_color).inspect_value(path, offset, limit),
                "startSize" => (self.start_size).inspect_value(path, offset, limit),
                "startRotation" => (self.start_rotation).inspect_value(path, offset, limit),
                "startRotationX" => (self.start_rotation_x).inspect_value(path, offset, limit),
                "startSizeY" => (self.start_size_y).inspect_value(path, offset, limit),
                "gravityModifier" => (self.gravity_modifier).inspect_value(path, offset, limit),
                "startRotationY" => (self.start_rotation_y).inspect_value(path, offset, limit),
                "startSizeZ" => (self.start_size_z).inspect_value(path, offset, limit),
                "inheritVelocity" => (self.inherit_velocity).inspect_value(path, offset, limit),
                "maxNumParticles" => (self.max_num_particles).inspect_value(path, offset, limit),
                "randomizeRotationDirection" => {
                    (self.randomize_rotation_direction).inspect_value(path, offset, limit)
                }
                "rotation3D" => (self.rotation3_d).inspect_value(path, offset, limit),
                "gravitySource" => (self.gravity_source).inspect_value(path, offset, limit),
                "size3D" => (self.size3_d).inspect_value(path, offset, limit),
                "customEmitterVelocity" => {
                    (self.custom_emitter_velocity).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.start_lifetime).inspect_value(path, offset, limit))
                .map(|v| ("startLifetime".to_string(), v)),
            ((self.start_speed).inspect_value(path, offset, limit))
                .map(|v| ("startSpeed".to_string(), v)),
            ((self.start_color).inspect_value(path, offset, limit))
                .map(|v| ("startColor".to_string(), v)),
            ((self.start_size).inspect_value(path, offset, limit))
                .map(|v| ("startSize".to_string(), v)),
            ((self.start_rotation).inspect_value(path, offset, limit))
                .map(|v| ("startRotation".to_string(), v)),
            ((self.start_rotation_x).inspect_value(path, offset, limit))
                .map(|v| ("startRotationX".to_string(), v)),
            ((self.start_size_y).inspect_value(path, offset, limit))
                .map(|v| ("startSizeY".to_string(), v)),
            ((self.gravity_modifier).inspect_value(path, offset, limit))
                .map(|v| ("gravityModifier".to_string(), v)),
            ((self.start_rotation_y).inspect_value(path, offset, limit))
                .map(|v| ("startRotationY".to_string(), v)),
            ((self.start_size_z).inspect_value(path, offset, limit))
                .map(|v| ("startSizeZ".to_string(), v)),
            ((self.inherit_velocity).inspect_value(path, offset, limit))
                .map(|v| ("inheritVelocity".to_string(), v)),
            ((self.max_num_particles).inspect_value(path, offset, limit))
                .map(|v| ("maxNumParticles".to_string(), v)),
            ((self.randomize_rotation_direction).inspect_value(path, offset, limit))
                .map(|v| ("randomizeRotationDirection".to_string(), v)),
            ((self.rotation3_d).inspect_value(path, offset, limit))
                .map(|v| ("rotation3D".to_string(), v)),
            ((self.gravity_source).inspect_value(path, offset, limit))
                .map(|v| ("gravitySource".to_string(), v)),
            ((self.size3_d).inspect_value(path, offset, limit)).map(|v| ("size3D".to_string(), v)),
            ((self.custom_emitter_velocity).inspect_value(path, offset, limit))
                .map(|v| ("customEmitterVelocity".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
