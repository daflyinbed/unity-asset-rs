//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::min_max_gradient::MinMaxGradient;

#[derive(Debug, Clone)]
pub struct InitialModule {
    pub enabled: bool,
    pub start_lifetime: MinMaxCurve,
    pub start_speed: MinMaxCurve,
    pub start_color: MinMaxGradient,
    pub start_size: MinMaxCurve,
    pub start_rotation: MinMaxCurve,
    pub gravity_modifier: f32,
    pub inherit_velocity: f32,
    pub max_num_particles: i32,
}

impl crate::Parse for InitialModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let start_lifetime = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let start_speed = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let start_color = MinMaxGradient::parse(r, ver)?;
        r.align(4)?;
        let start_size = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let start_rotation = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let gravity_modifier = r.read_f32()?;
        let inherit_velocity = r.read_f32()?;
        let max_num_particles = r.read_i32()?;

        Ok(InitialModule {
            enabled,
            start_lifetime,
            start_speed,
            start_color,
            start_size,
            start_rotation,
            gravity_modifier,
            inherit_velocity,
            max_num_particles,
        })
    }
}
