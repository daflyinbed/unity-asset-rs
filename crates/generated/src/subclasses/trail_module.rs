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

impl crate::ToValue for TrailModuleInheritParticleColorVariant {
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

impl crate::ToValue for TrailModuleGenerateLightingDataVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for TrailModuleSplitSubEmitterRibbonsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
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
            split_sub_emitter_ribbons =
                Some(TrailModuleSplitSubEmitterRibbonsVariant::parse(rdr, ver)?);
            attach_ribbons_to_transform = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            generate_lighting_data = Some(TrailModuleGenerateLightingDataVariant::parse(rdr, ver)?);
            split_sub_emitter_ribbons =
                Some(TrailModuleSplitSubEmitterRibbonsVariant::parse(rdr, ver)?);
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

impl crate::ToValue for TrailModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("ratio".to_string(), self.ratio.to_value())),
            crate::opt_field("mode", &self.mode),
            Some(("lifetime".to_string(), self.lifetime.to_value())),
            Some((
                "minVertexDistance".to_string(),
                self.min_vertex_distance.to_value(),
            )),
            Some(("textureMode".to_string(), self.texture_mode.to_value())),
            Some(("worldSpace".to_string(), self.world_space.to_value())),
            Some((
                "dieWithParticles".to_string(),
                self.die_with_particles.to_value(),
            )),
            crate::opt_field("ribbonCount", &self.ribbon_count),
            crate::opt_field("textureScale", &self.texture_scale),
            Some((
                "sizeAffectsWidth".to_string(),
                self.size_affects_width.to_value(),
            )),
            crate::opt_field("shadowBias", &self.shadow_bias),
            Some((
                "sizeAffectsLifetime".to_string(),
                self.size_affects_lifetime.to_value(),
            )),
            Some((
                "inheritParticleColor".to_string(),
                self.inherit_particle_color.to_value(),
            )),
            Some((
                "colorOverLifetime".to_string(),
                self.color_over_lifetime.to_value(),
            )),
            crate::opt_field("generateLightingData", &self.generate_lighting_data),
            Some((
                "widthOverTrail".to_string(),
                self.width_over_trail.to_value(),
            )),
            Some((
                "colorOverTrail".to_string(),
                self.color_over_trail.to_value(),
            )),
            crate::opt_field("splitSubEmitterRibbons", &self.split_sub_emitter_ribbons),
            crate::opt_field(
                "attachRibbonsToTransform",
                &self.attach_ribbons_to_transform,
            ),
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
                "ratio" => (self.ratio).inspect_value(path, offset, limit),
                "mode" => (self.mode).inspect_value(path, offset, limit),
                "lifetime" => (self.lifetime).inspect_value(path, offset, limit),
                "minVertexDistance" => {
                    (self.min_vertex_distance).inspect_value(path, offset, limit)
                }
                "textureMode" => (self.texture_mode).inspect_value(path, offset, limit),
                "worldSpace" => (self.world_space).inspect_value(path, offset, limit),
                "dieWithParticles" => (self.die_with_particles).inspect_value(path, offset, limit),
                "ribbonCount" => (self.ribbon_count).inspect_value(path, offset, limit),
                "textureScale" => (self.texture_scale).inspect_value(path, offset, limit),
                "sizeAffectsWidth" => (self.size_affects_width).inspect_value(path, offset, limit),
                "shadowBias" => (self.shadow_bias).inspect_value(path, offset, limit),
                "sizeAffectsLifetime" => {
                    (self.size_affects_lifetime).inspect_value(path, offset, limit)
                }
                "inheritParticleColor" => {
                    (self.inherit_particle_color).inspect_value(path, offset, limit)
                }
                "colorOverLifetime" => {
                    (self.color_over_lifetime).inspect_value(path, offset, limit)
                }
                "generateLightingData" => {
                    (self.generate_lighting_data).inspect_value(path, offset, limit)
                }
                "widthOverTrail" => (self.width_over_trail).inspect_value(path, offset, limit),
                "colorOverTrail" => (self.color_over_trail).inspect_value(path, offset, limit),
                "splitSubEmitterRibbons" => {
                    (self.split_sub_emitter_ribbons).inspect_value(path, offset, limit)
                }
                "attachRibbonsToTransform" => {
                    (self.attach_ribbons_to_transform).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.ratio).inspect_value(path, offset, limit)).map(|v| ("ratio".to_string(), v)),
            ((self.mode).inspect_value(path, offset, limit)).map(|v| ("mode".to_string(), v)),
            ((self.lifetime).inspect_value(path, offset, limit))
                .map(|v| ("lifetime".to_string(), v)),
            ((self.min_vertex_distance).inspect_value(path, offset, limit))
                .map(|v| ("minVertexDistance".to_string(), v)),
            ((self.texture_mode).inspect_value(path, offset, limit))
                .map(|v| ("textureMode".to_string(), v)),
            ((self.world_space).inspect_value(path, offset, limit))
                .map(|v| ("worldSpace".to_string(), v)),
            ((self.die_with_particles).inspect_value(path, offset, limit))
                .map(|v| ("dieWithParticles".to_string(), v)),
            ((self.ribbon_count).inspect_value(path, offset, limit))
                .map(|v| ("ribbonCount".to_string(), v)),
            ((self.texture_scale).inspect_value(path, offset, limit))
                .map(|v| ("textureScale".to_string(), v)),
            ((self.size_affects_width).inspect_value(path, offset, limit))
                .map(|v| ("sizeAffectsWidth".to_string(), v)),
            ((self.shadow_bias).inspect_value(path, offset, limit))
                .map(|v| ("shadowBias".to_string(), v)),
            ((self.size_affects_lifetime).inspect_value(path, offset, limit))
                .map(|v| ("sizeAffectsLifetime".to_string(), v)),
            ((self.inherit_particle_color).inspect_value(path, offset, limit))
                .map(|v| ("inheritParticleColor".to_string(), v)),
            ((self.color_over_lifetime).inspect_value(path, offset, limit))
                .map(|v| ("colorOverLifetime".to_string(), v)),
            ((self.generate_lighting_data).inspect_value(path, offset, limit))
                .map(|v| ("generateLightingData".to_string(), v)),
            ((self.width_over_trail).inspect_value(path, offset, limit))
                .map(|v| ("widthOverTrail".to_string(), v)),
            ((self.color_over_trail).inspect_value(path, offset, limit))
                .map(|v| ("colorOverTrail".to_string(), v)),
            ((self.split_sub_emitter_ribbons).inspect_value(path, offset, limit))
                .map(|v| ("splitSubEmitterRibbons".to_string(), v)),
            ((self.attach_ribbons_to_transform).inspect_value(path, offset, limit))
                .map(|v| ("attachRibbonsToTransform".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
