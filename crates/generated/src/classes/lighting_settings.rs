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
            rdr.align(4)?;
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
