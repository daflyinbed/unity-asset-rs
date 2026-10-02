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

impl crate::ToValue for NoiseModuleRemapEnabledVariant {
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

impl crate::ToValue for NoiseModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("strength".to_string(), self.strength.to_value())),
            Some(("strengthY".to_string(), self.strength_y.to_value())),
            Some(("strengthZ".to_string(), self.strength_z.to_value())),
            Some(("separateAxes".to_string(), self.separate_axes.to_value())),
            Some(("frequency".to_string(), self.frequency.to_value())),
            Some(("damping".to_string(), self.damping.to_value())),
            Some(("octaves".to_string(), self.octaves.to_value())),
            Some((
                "octaveMultiplier".to_string(),
                self.octave_multiplier.to_value(),
            )),
            Some(("octaveScale".to_string(), self.octave_scale.to_value())),
            Some(("quality".to_string(), self.quality.to_value())),
            Some(("scrollSpeed".to_string(), self.scroll_speed.to_value())),
            Some(("remap".to_string(), self.remap.to_value())),
            Some(("remapY".to_string(), self.remap_y.to_value())),
            Some(("remapZ".to_string(), self.remap_z.to_value())),
            Some(("remapEnabled".to_string(), self.remap_enabled.to_value())),
            crate::opt_field("positionAmount", &self.position_amount),
            crate::opt_field("rotationAmount", &self.rotation_amount),
            crate::opt_field("sizeAmount", &self.size_amount),
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
                "strength" => (self.strength).inspect_value(path, offset, limit),
                "strengthY" => (self.strength_y).inspect_value(path, offset, limit),
                "strengthZ" => (self.strength_z).inspect_value(path, offset, limit),
                "separateAxes" => (self.separate_axes).inspect_value(path, offset, limit),
                "frequency" => (self.frequency).inspect_value(path, offset, limit),
                "damping" => (self.damping).inspect_value(path, offset, limit),
                "octaves" => (self.octaves).inspect_value(path, offset, limit),
                "octaveMultiplier" => (self.octave_multiplier).inspect_value(path, offset, limit),
                "octaveScale" => (self.octave_scale).inspect_value(path, offset, limit),
                "quality" => (self.quality).inspect_value(path, offset, limit),
                "scrollSpeed" => (self.scroll_speed).inspect_value(path, offset, limit),
                "remap" => (self.remap).inspect_value(path, offset, limit),
                "remapY" => (self.remap_y).inspect_value(path, offset, limit),
                "remapZ" => (self.remap_z).inspect_value(path, offset, limit),
                "remapEnabled" => (self.remap_enabled).inspect_value(path, offset, limit),
                "positionAmount" => (self.position_amount).inspect_value(path, offset, limit),
                "rotationAmount" => (self.rotation_amount).inspect_value(path, offset, limit),
                "sizeAmount" => (self.size_amount).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.strength).inspect_value(path, offset, limit))
                .map(|v| ("strength".to_string(), v)),
            ((self.strength_y).inspect_value(path, offset, limit))
                .map(|v| ("strengthY".to_string(), v)),
            ((self.strength_z).inspect_value(path, offset, limit))
                .map(|v| ("strengthZ".to_string(), v)),
            ((self.separate_axes).inspect_value(path, offset, limit))
                .map(|v| ("separateAxes".to_string(), v)),
            ((self.frequency).inspect_value(path, offset, limit))
                .map(|v| ("frequency".to_string(), v)),
            ((self.damping).inspect_value(path, offset, limit)).map(|v| ("damping".to_string(), v)),
            ((self.octaves).inspect_value(path, offset, limit)).map(|v| ("octaves".to_string(), v)),
            ((self.octave_multiplier).inspect_value(path, offset, limit))
                .map(|v| ("octaveMultiplier".to_string(), v)),
            ((self.octave_scale).inspect_value(path, offset, limit))
                .map(|v| ("octaveScale".to_string(), v)),
            ((self.quality).inspect_value(path, offset, limit)).map(|v| ("quality".to_string(), v)),
            ((self.scroll_speed).inspect_value(path, offset, limit))
                .map(|v| ("scrollSpeed".to_string(), v)),
            ((self.remap).inspect_value(path, offset, limit)).map(|v| ("remap".to_string(), v)),
            ((self.remap_y).inspect_value(path, offset, limit)).map(|v| ("remapY".to_string(), v)),
            ((self.remap_z).inspect_value(path, offset, limit)).map(|v| ("remapZ".to_string(), v)),
            ((self.remap_enabled).inspect_value(path, offset, limit))
                .map(|v| ("remapEnabled".to_string(), v)),
            ((self.position_amount).inspect_value(path, offset, limit))
                .map(|v| ("positionAmount".to_string(), v)),
            ((self.rotation_amount).inspect_value(path, offset, limit))
                .map(|v| ("rotationAmount".to_string(), v)),
            ((self.size_amount).inspect_value(path, offset, limit))
                .map(|v| ("sizeAmount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
