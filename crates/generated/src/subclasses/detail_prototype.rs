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
