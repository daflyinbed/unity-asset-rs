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
