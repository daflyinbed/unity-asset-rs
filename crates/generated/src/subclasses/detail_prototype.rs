//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct DetailPrototype {
    pub prototype: PPtr,
    pub prototype_texture: PPtr,
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub noise_spread: f32,
    pub bend_factor: f32,
    pub healthy_color: ColorRgba,
    pub dry_color: ColorRgba,
    pub lightmap_factor: f32,
    pub render_mode: i32,
    pub use_prototype_mesh: i32,
}

impl crate::Parse for DetailPrototype {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let prototype = crate::PPtr::parse(r, ver)?;
        let prototype_texture = crate::PPtr::parse(r, ver)?;
        let min_width = r.read_f32()?;
        let max_width = r.read_f32()?;
        let min_height = r.read_f32()?;
        let max_height = r.read_f32()?;
        let noise_spread = r.read_f32()?;
        let bend_factor = r.read_f32()?;
        let healthy_color = ColorRgba::parse(r, ver)?;
        let dry_color = ColorRgba::parse(r, ver)?;
        let lightmap_factor = r.read_f32()?;
        let render_mode = r.read_i32()?;
        let use_prototype_mesh = r.read_i32()?;

        Ok(DetailPrototype {
            prototype,
            prototype_texture,
            min_width,
            max_width,
            min_height,
            max_height,
            noise_spread,
            bend_factor,
            healthy_color,
            dry_color,
            lightmap_factor,
            render_mode,
            use_prototype_mesh,
        })
    }
}
