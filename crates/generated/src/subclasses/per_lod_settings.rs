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
