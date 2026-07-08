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
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_gi_workflow_mode = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_enable_baked_lightmaps = rdr.read_bool()?;
        let m_enable_realtime_lightmaps = rdr.read_bool()?;
        let m_realtime_environment_lighting = rdr.read_bool()?;
        rdr.align(4)?;
        let m_bounce_scale = rdr.read_f32()?;
        let m_albedo_boost = rdr.read_f32()?;
        let m_indirect_output_scale = rdr.read_f32()?;
        let m_using_shadowmask = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(LightingSettings {
            m_name,
            m_gi_workflow_mode,
            m_enable_baked_lightmaps,
            m_enable_realtime_lightmaps,
            m_realtime_environment_lighting,
            m_bounce_scale,
            m_albedo_boost,
            m_indirect_output_scale,
            m_using_shadowmask,
        })
    }
}
