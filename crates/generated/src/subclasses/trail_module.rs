//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::min_max_gradient::MinMaxGradient;
use crate::subclasses::vector2f::Vector2f;


/// 版本枚举：TrailModuleInheritParticleColorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrailModuleInheritParticleColorVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for TrailModuleInheritParticleColorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TrailModuleGenerateLightingDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrailModuleGenerateLightingDataVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for TrailModuleGenerateLightingDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TrailModuleSplitSubEmitterRibbonsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TrailModuleSplitSubEmitterRibbonsVariant {
    Default(bool),
    V2018_3(bool),
}

impl crate::Parse for TrailModuleSplitSubEmitterRibbonsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TrailModule {
    pub enabled: bool,
    pub ratio: f32,
    pub mode: Option<i32>,
    pub lifetime: MinMaxCurve,
    pub min_vertex_distance: f32,
    pub texture_mode: i32,
    pub world_space: bool,
    pub die_with_particles: bool,
    pub ribbon_count: Option<i32>,
    pub texture_scale: Option<Vector2f>,
    pub size_affects_width: bool,
    pub shadow_bias: Option<f32>,
    pub size_affects_lifetime: bool,
    pub inherit_particle_color: TrailModuleInheritParticleColorVariant,
    pub color_over_lifetime: MinMaxGradient,
    pub generate_lighting_data: Option<TrailModuleGenerateLightingDataVariant>,
    pub width_over_trail: MinMaxCurve,
    pub color_over_trail: MinMaxGradient,
    pub split_sub_emitter_ribbons: Option<TrailModuleSplitSubEmitterRibbonsVariant>,
    pub attach_ribbons_to_transform: Option<bool>,
}

impl crate::Parse for TrailModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut ratio = None;
        let mut mode = None;
        let mut lifetime = None;
        let mut min_vertex_distance = None;
        let mut texture_mode = None;
        let mut world_space = None;
        let mut die_with_particles = None;
        let mut ribbon_count = None;
        let mut texture_scale = None;
        let mut size_affects_width = None;
        let mut shadow_bias = None;
        let mut size_affects_lifetime = None;
        let mut inherit_particle_color = None;
        let mut color_over_lifetime = None;
        let mut generate_lighting_data = None;
        let mut width_over_trail = None;
        let mut color_over_trail = None;
        let mut split_sub_emitter_ribbons = None;
        let mut attach_ribbons_to_transform = None;
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            mode = Some(rdr.read_i32()?);
        } else {
        }
            ratio = Some(rdr.read_f32()?);
            lifetime = Some(MinMaxCurve::parse(rdr, ver)?);
            min_vertex_distance = Some(rdr.read_f32()?);
            texture_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            texture_scale = Some(Vector2f::parse(rdr, ver)?);
            ribbon_count = Some(rdr.read_i32()?);
            shadow_bias = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            ribbon_count = Some(rdr.read_i32()?);
            shadow_bias = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            ribbon_count = Some(rdr.read_i32()?);
        } else {
        }
            world_space = Some(rdr.read_bool()?);
            die_with_particles = Some(rdr.read_bool()?);
            size_affects_width = Some(rdr.read_bool()?);
            size_affects_lifetime = Some(rdr.read_bool()?);
            inherit_particle_color = Some(TrailModuleInheritParticleColorVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            generate_lighting_data = Some(TrailModuleGenerateLightingDataVariant::parse(rdr, ver)?);
            split_sub_emitter_ribbons = Some(TrailModuleSplitSubEmitterRibbonsVariant::parse(rdr, ver)?);
            attach_ribbons_to_transform = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            generate_lighting_data = Some(TrailModuleGenerateLightingDataVariant::parse(rdr, ver)?);
            split_sub_emitter_ribbons = Some(TrailModuleSplitSubEmitterRibbonsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            generate_lighting_data = Some(TrailModuleGenerateLightingDataVariant::parse(rdr, ver)?);
        } else {
        }
            color_over_lifetime = Some(MinMaxGradient::parse(rdr, ver)?);
            width_over_trail = Some(MinMaxCurve::parse(rdr, ver)?);
            color_over_trail = Some(MinMaxGradient::parse(rdr, ver)?);

        Ok(TrailModule {
            enabled: enabled.unwrap(),
            ratio: ratio.unwrap(),
            mode,
            lifetime: lifetime.unwrap(),
            min_vertex_distance: min_vertex_distance.unwrap(),
            texture_mode: texture_mode.unwrap(),
            world_space: world_space.unwrap(),
            die_with_particles: die_with_particles.unwrap(),
            ribbon_count,
            texture_scale,
            size_affects_width: size_affects_width.unwrap(),
            shadow_bias,
            size_affects_lifetime: size_affects_lifetime.unwrap(),
            inherit_particle_color: inherit_particle_color.unwrap(),
            color_over_lifetime: color_over_lifetime.unwrap(),
            generate_lighting_data,
            width_over_trail: width_over_trail.unwrap(),
            color_over_trail: color_over_trail.unwrap(),
            split_sub_emitter_ribbons,
            attach_ribbons_to_transform,
        })
    }
}
