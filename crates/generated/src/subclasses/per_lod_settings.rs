//! 自动生成。请勿手动修改。

/// 版本枚举：PerLodSettingsEnableHueVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PerLodSettingsEnableHueVariant {
    Default(bool),
    V2018_3(bool),
}

impl crate::Parse for PerLodSettingsEnableHueVariant {
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

impl crate::ToValue for PerLodSettingsEnableHueVariant {
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
pub struct PerLodSettings {
    pub height: f32,
    pub enable_setting_override: Option<bool>,
    pub cast_shadows: bool,
    pub receive_shadows: bool,
    pub use_light_probes: bool,
    pub reflection_probe_usage: i32,
    pub enable_bump: bool,
    pub enable_hue: PerLodSettingsEnableHueVariant,
    pub wind_quality: i32,
    pub enable_subsurface: Option<bool>,
}

impl crate::Parse for PerLodSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut height = None;
        let mut enable_setting_override = None;
        let mut cast_shadows = None;
        let mut receive_shadows = None;
        let mut use_light_probes = None;
        let mut reflection_probe_usage = None;
        let mut enable_bump = None;
        let mut enable_hue = None;
        let mut wind_quality = None;
        let mut enable_subsurface = None;
        if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            enable_setting_override = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        height = Some(rdr.read_f32()?);
        cast_shadows = Some(rdr.read_bool()?);
        receive_shadows = Some(rdr.read_bool()?);
        use_light_probes = Some(rdr.read_bool()?);
        rdr.align(4)?;
        reflection_probe_usage = Some(rdr.read_i32()?);
        enable_bump = Some(rdr.read_bool()?);
        enable_hue = Some(PerLodSettingsEnableHueVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            enable_subsurface = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        wind_quality = Some(rdr.read_i32()?);

        Ok(PerLodSettings {
            height: height.unwrap(),
            enable_setting_override,
            cast_shadows: cast_shadows.unwrap(),
            receive_shadows: receive_shadows.unwrap(),
            use_light_probes: use_light_probes.unwrap(),
            reflection_probe_usage: reflection_probe_usage.unwrap(),
            enable_bump: enable_bump.unwrap(),
            enable_hue: enable_hue.unwrap(),
            wind_quality: wind_quality.unwrap(),
            enable_subsurface,
        })
    }
}

impl crate::ToValue for PerLodSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("height".to_string(), self.height.to_value())),
            crate::opt_field("enableSettingOverride", &self.enable_setting_override),
            Some(("castShadows".to_string(), self.cast_shadows.to_value())),
            Some((
                "receiveShadows".to_string(),
                self.receive_shadows.to_value(),
            )),
            Some((
                "useLightProbes".to_string(),
                self.use_light_probes.to_value(),
            )),
            Some((
                "reflectionProbeUsage".to_string(),
                self.reflection_probe_usage.to_value(),
            )),
            Some(("enableBump".to_string(), self.enable_bump.to_value())),
            Some(("enableHue".to_string(), self.enable_hue.to_value())),
            Some(("windQuality".to_string(), self.wind_quality.to_value())),
            crate::opt_field("enableSubsurface", &self.enable_subsurface),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "height" => (self.height).inspect_value(path, offset, limit),
                "enableSettingOverride" => {
                    (self.enable_setting_override).inspect_value(path, offset, limit)
                }
                "castShadows" => (self.cast_shadows).inspect_value(path, offset, limit),
                "receiveShadows" => (self.receive_shadows).inspect_value(path, offset, limit),
                "useLightProbes" => (self.use_light_probes).inspect_value(path, offset, limit),
                "reflectionProbeUsage" => {
                    (self.reflection_probe_usage).inspect_value(path, offset, limit)
                }
                "enableBump" => (self.enable_bump).inspect_value(path, offset, limit),
                "enableHue" => (self.enable_hue).inspect_value(path, offset, limit),
                "windQuality" => (self.wind_quality).inspect_value(path, offset, limit),
                "enableSubsurface" => (self.enable_subsurface).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.height).inspect_value(path, offset, limit)).map(|v| ("height".to_string(), v)),
            ((self.enable_setting_override).inspect_value(path, offset, limit))
                .map(|v| ("enableSettingOverride".to_string(), v)),
            ((self.cast_shadows).inspect_value(path, offset, limit))
                .map(|v| ("castShadows".to_string(), v)),
            ((self.receive_shadows).inspect_value(path, offset, limit))
                .map(|v| ("receiveShadows".to_string(), v)),
            ((self.use_light_probes).inspect_value(path, offset, limit))
                .map(|v| ("useLightProbes".to_string(), v)),
            ((self.reflection_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("reflectionProbeUsage".to_string(), v)),
            ((self.enable_bump).inspect_value(path, offset, limit))
                .map(|v| ("enableBump".to_string(), v)),
            ((self.enable_hue).inspect_value(path, offset, limit))
                .map(|v| ("enableHue".to_string(), v)),
            ((self.wind_quality).inspect_value(path, offset, limit))
                .map(|v| ("windQuality".to_string(), v)),
            ((self.enable_subsurface).inspect_value(path, offset, limit))
                .map(|v| ("enableSubsurface".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
