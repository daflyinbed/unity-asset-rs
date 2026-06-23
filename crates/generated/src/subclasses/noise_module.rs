//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct NoiseModule {
    pub enabled: bool,
    pub strength: MinMaxCurve,
    pub strength_y: MinMaxCurve,
    pub strength_z: MinMaxCurve,
    pub separate_axes: bool,
    pub frequency: f32,
    pub damping: bool,
    pub octaves: i32,
    pub octave_multiplier: f32,
    pub octave_scale: f32,
    pub quality: i32,
    pub scroll_speed: MinMaxCurve,
    pub remap: MinMaxCurve,
    pub remap_y: MinMaxCurve,
    pub remap_z: MinMaxCurve,
    pub remap_enabled: bool,
}

impl crate::Parse for NoiseModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let strength = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let strength_y = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let strength_z = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let separate_axes = r.read_bool()?;
        r.align(4)?;
        let frequency = r.read_f32()?;
        let damping = r.read_bool()?;
        r.align(4)?;
        let octaves = r.read_i32()?;
        let octave_multiplier = r.read_f32()?;
        let octave_scale = r.read_f32()?;
        let quality = r.read_i32()?;
        let scroll_speed = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let remap = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let remap_y = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let remap_z = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let remap_enabled = r.read_bool()?;

        Ok(NoiseModule {
            enabled,
            strength,
            strength_y,
            strength_z,
            separate_axes,
            frequency,
            damping,
            octaves,
            octave_multiplier,
            octave_scale,
            quality,
            scroll_speed,
            remap,
            remap_y,
            remap_z,
            remap_enabled,
        })
    }
}
