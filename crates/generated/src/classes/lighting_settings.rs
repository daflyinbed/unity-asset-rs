//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightingSettings {
    pub m_name: String,
    pub m_gi_workflow_mode: Option<i32>,
    pub m_enable_baked_lightmaps: bool,
    pub m_enable_realtime_lightmaps: bool,
    pub m_realtime_environment_lighting: bool,
    pub m_bounce_scale: f32,
    pub m_albedo_boost: f32,
    pub m_indirect_output_scale: f32,
    pub m_using_shadowmask: bool,
}

impl crate::Parse for LightingSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_gi_workflow_mode = None;
        let mut m_enable_baked_lightmaps = None;
        let mut m_enable_realtime_lightmaps = None;
        let mut m_realtime_environment_lighting = None;
        let mut m_bounce_scale = None;
        let mut m_albedo_boost = None;
        let mut m_indirect_output_scale = None;
        let mut m_using_shadowmask = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
        } else {
            m_gi_workflow_mode = Some(rdr.read_i32()?);
        }
        m_enable_baked_lightmaps = Some(rdr.read_bool()?);
        m_enable_realtime_lightmaps = Some(rdr.read_bool()?);
        m_realtime_environment_lighting = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_bounce_scale = Some(rdr.read_f32()?);
        m_albedo_boost = Some(rdr.read_f32()?);
        m_indirect_output_scale = Some(rdr.read_f32()?);
        m_using_shadowmask = Some(rdr.read_bool()?);
        rdr.align(4)?;

        Ok(LightingSettings {
            m_name: m_name.unwrap(),
            m_gi_workflow_mode,
            m_enable_baked_lightmaps: m_enable_baked_lightmaps.unwrap(),
            m_enable_realtime_lightmaps: m_enable_realtime_lightmaps.unwrap(),
            m_realtime_environment_lighting: m_realtime_environment_lighting.unwrap(),
            m_bounce_scale: m_bounce_scale.unwrap(),
            m_albedo_boost: m_albedo_boost.unwrap(),
            m_indirect_output_scale: m_indirect_output_scale.unwrap(),
            m_using_shadowmask: m_using_shadowmask.unwrap(),
        })
    }
}

impl crate::ToValue for LightingSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_GIWorkflowMode", &self.m_gi_workflow_mode),
            Some((
                "m_EnableBakedLightmaps".to_string(),
                self.m_enable_baked_lightmaps.to_value(),
            )),
            Some((
                "m_EnableRealtimeLightmaps".to_string(),
                self.m_enable_realtime_lightmaps.to_value(),
            )),
            Some((
                "m_RealtimeEnvironmentLighting".to_string(),
                self.m_realtime_environment_lighting.to_value(),
            )),
            Some(("m_BounceScale".to_string(), self.m_bounce_scale.to_value())),
            Some(("m_AlbedoBoost".to_string(), self.m_albedo_boost.to_value())),
            Some((
                "m_IndirectOutputScale".to_string(),
                self.m_indirect_output_scale.to_value(),
            )),
            Some((
                "m_UsingShadowmask".to_string(),
                self.m_using_shadowmask.to_value(),
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_GIWorkflowMode" => (self.m_gi_workflow_mode).inspect_value(path, offset, limit),
                "m_EnableBakedLightmaps" => {
                    (self.m_enable_baked_lightmaps).inspect_value(path, offset, limit)
                }
                "m_EnableRealtimeLightmaps" => {
                    (self.m_enable_realtime_lightmaps).inspect_value(path, offset, limit)
                }
                "m_RealtimeEnvironmentLighting" => {
                    (self.m_realtime_environment_lighting).inspect_value(path, offset, limit)
                }
                "m_BounceScale" => (self.m_bounce_scale).inspect_value(path, offset, limit),
                "m_AlbedoBoost" => (self.m_albedo_boost).inspect_value(path, offset, limit),
                "m_IndirectOutputScale" => {
                    (self.m_indirect_output_scale).inspect_value(path, offset, limit)
                }
                "m_UsingShadowmask" => (self.m_using_shadowmask).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_gi_workflow_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_GIWorkflowMode".to_string(), v)),
            ((self.m_enable_baked_lightmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableBakedLightmaps".to_string(), v)),
            ((self.m_enable_realtime_lightmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableRealtimeLightmaps".to_string(), v)),
            ((self.m_realtime_environment_lighting).inspect_value(path, offset, limit))
                .map(|v| ("m_RealtimeEnvironmentLighting".to_string(), v)),
            ((self.m_bounce_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_BounceScale".to_string(), v)),
            ((self.m_albedo_boost).inspect_value(path, offset, limit))
                .map(|v| ("m_AlbedoBoost".to_string(), v)),
            ((self.m_indirect_output_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_IndirectOutputScale".to_string(), v)),
            ((self.m_using_shadowmask).inspect_value(path, offset, limit))
                .map(|v| ("m_UsingShadowmask".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
