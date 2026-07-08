//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;


/// 版本枚举：NoiseModuleRemapEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NoiseModuleRemapEnabledVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for NoiseModuleRemapEnabledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
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
    pub remap_enabled: NoiseModuleRemapEnabledVariant,
    pub position_amount: Option<MinMaxCurve>,
    pub rotation_amount: Option<MinMaxCurve>,
    pub size_amount: Option<MinMaxCurve>,
}

impl crate::Parse for NoiseModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let strength = MinMaxCurve::parse(rdr, ver)?;
        let strength_y = MinMaxCurve::parse(rdr, ver)?;
        let strength_z = MinMaxCurve::parse(rdr, ver)?;
        let separate_axes = rdr.read_bool()?;
        rdr.align(4)?;
        let frequency = rdr.read_f32()?;
        let damping = rdr.read_bool()?;
        rdr.align(4)?;
        let octaves = rdr.read_i32()?;
        let octave_multiplier = rdr.read_f32()?;
        let octave_scale = rdr.read_f32()?;
        let quality = rdr.read_i32()?;
        let scroll_speed = MinMaxCurve::parse(rdr, ver)?;
        let remap = MinMaxCurve::parse(rdr, ver)?;
        let remap_y = MinMaxCurve::parse(rdr, ver)?;
        let remap_z = MinMaxCurve::parse(rdr, ver)?;
        let remap_enabled = NoiseModuleRemapEnabledVariant::parse(rdr, ver)?;
        let position_amount = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(MinMaxCurve::parse(rdr, ver)?)
        } else {
            None
        };
        let rotation_amount = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(MinMaxCurve::parse(rdr, ver)?)
        } else {
            None
        };
        let size_amount = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(MinMaxCurve::parse(rdr, ver)?)
        } else {
            None
        };

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
            position_amount,
            rotation_amount,
            size_amount,
        })
    }
}
