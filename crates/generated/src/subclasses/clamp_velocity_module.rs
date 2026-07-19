//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

/// 版本枚举：ClampVelocityModuleSeparateAxisVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClampVelocityModuleSeparateAxisVariant {
    Default(bool),
    V4_0(bool),
}

impl crate::Parse for ClampVelocityModuleSeparateAxisVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ClampVelocityModuleInWorldSpaceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClampVelocityModuleInWorldSpaceVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for ClampVelocityModuleInWorldSpaceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClampVelocityModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub magnitude: MinMaxCurve,
    pub separate_axis: ClampVelocityModuleSeparateAxisVariant,
    pub dampen: f32,
    pub in_world_space: Option<ClampVelocityModuleInWorldSpaceVariant>,
    pub multiply_drag_by_particle_size: Option<bool>,
    pub multiply_drag_by_particle_velocity: Option<bool>,
    pub drag: Option<MinMaxCurve>,
}

impl crate::Parse for ClampVelocityModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut x = None;
        let mut y = None;
        let mut z = None;
        let mut magnitude = None;
        let mut separate_axis = None;
        let mut dampen = None;
        let mut in_world_space = None;
        let mut multiply_drag_by_particle_size = None;
        let mut multiply_drag_by_particle_velocity = None;
        let mut drag = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        x = Some(MinMaxCurve::parse(rdr, ver)?);
        y = Some(MinMaxCurve::parse(rdr, ver)?);
        z = Some(MinMaxCurve::parse(rdr, ver)?);
        magnitude = Some(MinMaxCurve::parse(rdr, ver)?);
        separate_axis = Some(ClampVelocityModuleSeparateAxisVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            in_world_space = Some(ClampVelocityModuleInWorldSpaceVariant::parse(rdr, ver)?);
            multiply_drag_by_particle_size = Some(rdr.read_bool()?);
            multiply_drag_by_particle_velocity = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            in_world_space = Some(ClampVelocityModuleInWorldSpaceVariant::parse(rdr, ver)?);
        } else {
        }
        dampen = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            drag = Some(MinMaxCurve::parse(rdr, ver)?);
        } else {
        }

        Ok(ClampVelocityModule {
            enabled: enabled.unwrap(),
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
            magnitude: magnitude.unwrap(),
            separate_axis: separate_axis.unwrap(),
            dampen: dampen.unwrap(),
            in_world_space,
            multiply_drag_by_particle_size,
            multiply_drag_by_particle_velocity,
            drag,
        })
    }
}
