//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GiSettings {
    pub m_bounce_scale: f32,
    pub m_indirect_output_scale: f32,
    pub m_albedo_boost: f32,
    pub m_temporal_coherence_threshold: f32,
    pub m_environment_lighting_mode: u32,
    pub m_enable_baked_lightmaps: bool,
    pub m_enable_realtime_lightmaps: bool,
}

impl crate::Parse for GiSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_bounce_scale = r.read_f32()?;
        let m_indirect_output_scale = r.read_f32()?;
        let m_albedo_boost = r.read_f32()?;
        let m_temporal_coherence_threshold = r.read_f32()?;
        let m_environment_lighting_mode = r.read_u32()?;
        let m_enable_baked_lightmaps = r.read_bool()?;
        let m_enable_realtime_lightmaps = r.read_bool()?;
        r.align(4)?;

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
