//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::min_max_gradient::MinMaxGradient;

#[derive(Debug, Clone)]
pub struct TrailModule {
    pub enabled: bool,
    pub ratio: f32,
    pub lifetime: MinMaxCurve,
    pub min_vertex_distance: f32,
    pub texture_mode: i32,
    pub world_space: bool,
    pub die_with_particles: bool,
    pub size_affects_width: bool,
    pub size_affects_lifetime: bool,
    pub inherit_particle_color: bool,
    pub color_over_lifetime: MinMaxGradient,
    pub width_over_trail: MinMaxCurve,
    pub color_over_trail: MinMaxGradient,
}

impl crate::Parse for TrailModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let ratio = r.read_f32()?;
        let lifetime = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let min_vertex_distance = r.read_f32()?;
        let texture_mode = r.read_i32()?;
        let world_space = r.read_bool()?;
        let die_with_particles = r.read_bool()?;
        let size_affects_width = r.read_bool()?;
        let size_affects_lifetime = r.read_bool()?;
        let inherit_particle_color = r.read_bool()?;
        r.align(4)?;
        let color_over_lifetime = MinMaxGradient::parse(r, ver)?;
        r.align(4)?;
        let width_over_trail = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let color_over_trail = MinMaxGradient::parse(r, ver)?;
        r.align(4)?;

        Ok(TrailModule {
            enabled,
            ratio,
            lifetime,
            min_vertex_distance,
            texture_mode,
            world_space,
            die_with_particles,
            size_affects_width,
            size_affects_lifetime,
            inherit_particle_color,
            color_over_lifetime,
            width_over_trail,
            color_over_trail,
        })
    }
}
