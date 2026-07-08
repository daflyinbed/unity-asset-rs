//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GiSettings {
    pub m_bounce_scale: f32,
    pub m_indirect_output_scale: f32,
    pub m_albedo_boost: f32,
    pub m_temporal_coherence_threshold: Option<f32>,
    pub m_environment_lighting_mode: u32,
    pub m_enable_baked_lightmaps: bool,
    pub m_enable_realtime_lightmaps: bool,
}

impl crate::Parse for GiSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_bounce_scale = rdr.read_f32()?;
        let m_indirect_output_scale = rdr.read_f32()?;
        let m_albedo_boost = rdr.read_f32()?;
        let m_temporal_coherence_threshold = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_environment_lighting_mode = rdr.read_u32()?;
        let m_enable_baked_lightmaps = rdr.read_bool()?;
        let m_enable_realtime_lightmaps = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(GiSettings {
            m_bounce_scale,
            m_indirect_output_scale,
            m_albedo_boost,
            m_temporal_coherence_threshold,
            m_environment_lighting_mode,
            m_enable_baked_lightmaps,
            m_enable_realtime_lightmaps,
        })
    }
}
