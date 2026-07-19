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
        let mut m_bounce_scale = None;
        let mut m_indirect_output_scale = None;
        let mut m_albedo_boost = None;
        let mut m_temporal_coherence_threshold = None;
        let mut m_environment_lighting_mode = None;
        let mut m_enable_baked_lightmaps = None;
        let mut m_enable_realtime_lightmaps = None;
        m_bounce_scale = Some(rdr.read_f32()?);
        m_indirect_output_scale = Some(rdr.read_f32()?);
        m_albedo_boost = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
        } else {
            m_temporal_coherence_threshold = Some(rdr.read_f32()?);
        }
        m_environment_lighting_mode = Some(rdr.read_u32()?);
        m_enable_baked_lightmaps = Some(rdr.read_bool()?);
        m_enable_realtime_lightmaps = Some(rdr.read_bool()?);
        rdr.align(4)?;

        Ok(GiSettings {
            m_bounce_scale: m_bounce_scale.unwrap(),
            m_indirect_output_scale: m_indirect_output_scale.unwrap(),
            m_albedo_boost: m_albedo_boost.unwrap(),
            m_temporal_coherence_threshold,
            m_environment_lighting_mode: m_environment_lighting_mode.unwrap(),
            m_enable_baked_lightmaps: m_enable_baked_lightmaps.unwrap(),
            m_enable_realtime_lightmaps: m_enable_realtime_lightmaps.unwrap(),
        })
    }
}
