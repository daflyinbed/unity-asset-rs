//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PerLodSettings {
    pub height: f32,
    pub cast_shadows: bool,
    pub receive_shadows: bool,
    pub use_light_probes: bool,
    pub reflection_probe_usage: i32,
    pub enable_bump: bool,
    pub enable_hue: bool,
    pub wind_quality: i32,
}

impl crate::Parse for PerLodSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let height = r.read_f32()?;
        let cast_shadows = r.read_bool()?;
        let receive_shadows = r.read_bool()?;
        let use_light_probes = r.read_bool()?;
        r.align(4)?;
        let reflection_probe_usage = r.read_i32()?;
        let enable_bump = r.read_bool()?;
        let enable_hue = r.read_bool()?;
        r.align(4)?;
        let wind_quality = r.read_i32()?;

        Ok(PerLodSettings {
            height,
            cast_shadows,
            receive_shadows,
            use_light_probes,
            reflection_probe_usage,
            enable_bump,
            enable_hue,
            wind_quality,
        })
    }
}
