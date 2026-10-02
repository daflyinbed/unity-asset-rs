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

impl crate::ToValue for GiSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BounceScale".to_string(), self.m_bounce_scale.to_value())),
            Some((
                "m_IndirectOutputScale".to_string(),
                self.m_indirect_output_scale.to_value(),
            )),
            Some(("m_AlbedoBoost".to_string(), self.m_albedo_boost.to_value())),
            crate::opt_field(
                "m_TemporalCoherenceThreshold",
                &self.m_temporal_coherence_threshold,
            ),
            Some((
                "m_EnvironmentLightingMode".to_string(),
                self.m_environment_lighting_mode.to_value(),
            )),
            Some((
                "m_EnableBakedLightmaps".to_string(),
                self.m_enable_baked_lightmaps.to_value(),
            )),
            Some((
                "m_EnableRealtimeLightmaps".to_string(),
                self.m_enable_realtime_lightmaps.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BounceScale" => (self.m_bounce_scale).inspect_value(path, offset, limit),
                "m_IndirectOutputScale" => {
                    (self.m_indirect_output_scale).inspect_value(path, offset, limit)
                }
                "m_AlbedoBoost" => (self.m_albedo_boost).inspect_value(path, offset, limit),
                "m_TemporalCoherenceThreshold" => {
                    (self.m_temporal_coherence_threshold).inspect_value(path, offset, limit)
                }
                "m_EnvironmentLightingMode" => {
                    (self.m_environment_lighting_mode).inspect_value(path, offset, limit)
                }
                "m_EnableBakedLightmaps" => {
                    (self.m_enable_baked_lightmaps).inspect_value(path, offset, limit)
                }
                "m_EnableRealtimeLightmaps" => {
                    (self.m_enable_realtime_lightmaps).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_bounce_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceScale".to_string(), v)),
            ((self.m_indirect_output_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_IndirectOutputScale".to_string(), v)),
            ((self.m_albedo_boost).inspect_value(path, offset, limit))
                .map(|v| ("m_AlbedoBoost".to_string(), v)),
            ((self.m_temporal_coherence_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_TemporalCoherenceThreshold".to_string(), v)),
            ((self.m_environment_lighting_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_EnvironmentLightingMode".to_string(), v)),
            ((self.m_enable_baked_lightmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableBakedLightmaps".to_string(), v)),
            ((self.m_enable_realtime_lightmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableRealtimeLightmaps".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
