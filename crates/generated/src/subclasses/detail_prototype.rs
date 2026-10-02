//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;

#[derive(Debug, Clone)]
pub struct DetailPrototype {
    pub prototype: PPtr,
    pub prototype_texture: PPtr,
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub noise_spread: f32,
    pub noise_seed: Option<i32>,
    pub bend_factor: Option<f32>,
    pub hole_test_radius: Option<f32>,
    pub healthy_color: ColorRgbAf,
    pub dry_color: ColorRgbAf,
    pub density: Option<f32>,
    pub lightmap_factor: Option<f32>,
    pub render_mode: i32,
    pub use_prototype_mesh: i32,
    pub use_instancing: Option<i32>,
    pub use_density_scaling: Option<i32>,
    pub align_to_ground: Option<f32>,
    pub position_jitter: Option<f32>,
    pub target_coverage: Option<f32>,
}

impl crate::Parse for DetailPrototype {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut prototype = None;
        let mut prototype_texture = None;
        let mut min_width = None;
        let mut max_width = None;
        let mut min_height = None;
        let mut max_height = None;
        let mut noise_spread = None;
        let mut noise_seed = None;
        let mut bend_factor = None;
        let mut hole_test_radius = None;
        let mut healthy_color = None;
        let mut dry_color = None;
        let mut density = None;
        let mut lightmap_factor = None;
        let mut render_mode = None;
        let mut use_prototype_mesh = None;
        let mut use_instancing = None;
        let mut use_density_scaling = None;
        let mut align_to_ground = None;
        let mut position_jitter = None;
        let mut target_coverage = None;
        prototype = Some(crate::PPtr::parse(rdr, ver)?);
        prototype_texture = Some(crate::PPtr::parse(rdr, ver)?);
        min_width = Some(rdr.read_f32()?);
        max_width = Some(rdr.read_f32()?);
        min_height = Some(rdr.read_f32()?);
        max_height = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            noise_seed = Some(rdr.read_i32()?);
        } else {
        }
        noise_spread = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            hole_test_radius = Some(rdr.read_f32()?);
            density = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            hole_test_radius = Some(rdr.read_f32()?);
        } else {
            bend_factor = Some(rdr.read_f32()?);
        }
        healthy_color = Some(ColorRgbAf::parse(rdr, ver)?);
        dry_color = Some(ColorRgbAf::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
        } else {
            lightmap_factor = Some(rdr.read_f32()?);
        }
        render_mode = Some(rdr.read_i32()?);
        use_prototype_mesh = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            use_instancing = Some(rdr.read_i32()?);
            use_density_scaling = Some(rdr.read_i32()?);
            align_to_ground = Some(rdr.read_f32()?);
            position_jitter = Some(rdr.read_f32()?);
            target_coverage = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            use_instancing = Some(rdr.read_i32()?);
        } else {
        }

        Ok(DetailPrototype {
            prototype: prototype.unwrap(),
            prototype_texture: prototype_texture.unwrap(),
            min_width: min_width.unwrap(),
            max_width: max_width.unwrap(),
            min_height: min_height.unwrap(),
            max_height: max_height.unwrap(),
            noise_spread: noise_spread.unwrap(),
            noise_seed,
            bend_factor,
            hole_test_radius,
            healthy_color: healthy_color.unwrap(),
            dry_color: dry_color.unwrap(),
            density,
            lightmap_factor,
            render_mode: render_mode.unwrap(),
            use_prototype_mesh: use_prototype_mesh.unwrap(),
            use_instancing,
            use_density_scaling,
            align_to_ground,
            position_jitter,
            target_coverage,
        })
    }
}

impl crate::ToValue for DetailPrototype {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("prototype".to_string(), self.prototype.to_value())),
            Some((
                "prototypeTexture".to_string(),
                self.prototype_texture.to_value(),
            )),
            Some(("minWidth".to_string(), self.min_width.to_value())),
            Some(("maxWidth".to_string(), self.max_width.to_value())),
            Some(("minHeight".to_string(), self.min_height.to_value())),
            Some(("maxHeight".to_string(), self.max_height.to_value())),
            Some(("noiseSpread".to_string(), self.noise_spread.to_value())),
            crate::opt_field("noiseSeed", &self.noise_seed),
            crate::opt_field("bendFactor", &self.bend_factor),
            crate::opt_field("holeTestRadius", &self.hole_test_radius),
            Some(("healthyColor".to_string(), self.healthy_color.to_value())),
            Some(("dryColor".to_string(), self.dry_color.to_value())),
            crate::opt_field("density", &self.density),
            crate::opt_field("lightmapFactor", &self.lightmap_factor),
            Some(("renderMode".to_string(), self.render_mode.to_value())),
            Some((
                "usePrototypeMesh".to_string(),
                self.use_prototype_mesh.to_value(),
            )),
            crate::opt_field("useInstancing", &self.use_instancing),
            crate::opt_field("useDensityScaling", &self.use_density_scaling),
            crate::opt_field("alignToGround", &self.align_to_ground),
            crate::opt_field("positionJitter", &self.position_jitter),
            crate::opt_field("targetCoverage", &self.target_coverage),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "prototype" => (self.prototype).inspect_value(path, offset, limit),
                "prototypeTexture" => (self.prototype_texture).inspect_value(path, offset, limit),
                "minWidth" => (self.min_width).inspect_value(path, offset, limit),
                "maxWidth" => (self.max_width).inspect_value(path, offset, limit),
                "minHeight" => (self.min_height).inspect_value(path, offset, limit),
                "maxHeight" => (self.max_height).inspect_value(path, offset, limit),
                "noiseSpread" => (self.noise_spread).inspect_value(path, offset, limit),
                "noiseSeed" => (self.noise_seed).inspect_value(path, offset, limit),
                "bendFactor" => (self.bend_factor).inspect_value(path, offset, limit),
                "holeTestRadius" => (self.hole_test_radius).inspect_value(path, offset, limit),
                "healthyColor" => (self.healthy_color).inspect_value(path, offset, limit),
                "dryColor" => (self.dry_color).inspect_value(path, offset, limit),
                "density" => (self.density).inspect_value(path, offset, limit),
                "lightmapFactor" => (self.lightmap_factor).inspect_value(path, offset, limit),
                "renderMode" => (self.render_mode).inspect_value(path, offset, limit),
                "usePrototypeMesh" => (self.use_prototype_mesh).inspect_value(path, offset, limit),
                "useInstancing" => (self.use_instancing).inspect_value(path, offset, limit),
                "useDensityScaling" => {
                    (self.use_density_scaling).inspect_value(path, offset, limit)
                }
                "alignToGround" => (self.align_to_ground).inspect_value(path, offset, limit),
                "positionJitter" => (self.position_jitter).inspect_value(path, offset, limit),
                "targetCoverage" => (self.target_coverage).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.prototype).inspect_value(path, offset, limit))
                .map(|v| ("prototype".to_string(), v)),
            ((self.prototype_texture).inspect_value(path, offset, limit))
                .map(|v| ("prototypeTexture".to_string(), v)),
            ((self.min_width).inspect_value(path, offset, limit))
                .map(|v| ("minWidth".to_string(), v)),
            ((self.max_width).inspect_value(path, offset, limit))
                .map(|v| ("maxWidth".to_string(), v)),
            ((self.min_height).inspect_value(path, offset, limit))
                .map(|v| ("minHeight".to_string(), v)),
            ((self.max_height).inspect_value(path, offset, limit))
                .map(|v| ("maxHeight".to_string(), v)),
            ((self.noise_spread).inspect_value(path, offset, limit))
                .map(|v| ("noiseSpread".to_string(), v)),
            ((self.noise_seed).inspect_value(path, offset, limit))
                .map(|v| ("noiseSeed".to_string(), v)),
            ((self.bend_factor).inspect_value(path, offset, limit))
                .map(|v| ("bendFactor".to_string(), v)),
            ((self.hole_test_radius).inspect_value(path, offset, limit))
                .map(|v| ("holeTestRadius".to_string(), v)),
            ((self.healthy_color).inspect_value(path, offset, limit))
                .map(|v| ("healthyColor".to_string(), v)),
            ((self.dry_color).inspect_value(path, offset, limit))
                .map(|v| ("dryColor".to_string(), v)),
            ((self.density).inspect_value(path, offset, limit)).map(|v| ("density".to_string(), v)),
            ((self.lightmap_factor).inspect_value(path, offset, limit))
                .map(|v| ("lightmapFactor".to_string(), v)),
            ((self.render_mode).inspect_value(path, offset, limit))
                .map(|v| ("renderMode".to_string(), v)),
            ((self.use_prototype_mesh).inspect_value(path, offset, limit))
                .map(|v| ("usePrototypeMesh".to_string(), v)),
            ((self.use_instancing).inspect_value(path, offset, limit))
                .map(|v| ("useInstancing".to_string(), v)),
            ((self.use_density_scaling).inspect_value(path, offset, limit))
                .map(|v| ("useDensityScaling".to_string(), v)),
            ((self.align_to_ground).inspect_value(path, offset, limit))
                .map(|v| ("alignToGround".to_string(), v)),
            ((self.position_jitter).inspect_value(path, offset, limit))
                .map(|v| ("positionJitter".to_string(), v)),
            ((self.target_coverage).inspect_value(path, offset, limit))
                .map(|v| ("targetCoverage".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
