//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_renderer_settings::VfxRendererSettings;

#[derive(Debug, Clone)]
pub struct VisualEffectSettings {
    pub m_renderer_settings: VfxRendererSettings,
    pub m_culling_flags: i32,
    pub m_update_mode: i32,
    pub m_pre_warm_delta_time: f32,
    pub m_pre_warm_step_count: u32,
    pub m_initial_event_name: String,
}

impl crate::Parse for VisualEffectSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_renderer_settings = VfxRendererSettings::parse(r, ver)?;
        r.align(4)?;
        let m_culling_flags = r.read_i32()?;
        let m_update_mode = r.read_i32()?;
        let m_pre_warm_delta_time = r.read_f32()?;
        let m_pre_warm_step_count = r.read_u32()?;
        let m_initial_event_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(VisualEffectSettings {
            m_renderer_settings,
            m_culling_flags,
            m_update_mode,
            m_pre_warm_delta_time,
            m_pre_warm_step_count,
            m_initial_event_name,
        })
    }
}
