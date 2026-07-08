//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_renderer_settings::VfxRendererSettings;


/// 版本枚举：VisualEffectSettingsMRendererSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VisualEffectSettingsMRendererSettingsVariant {
    Default(Box<VfxRendererSettings>),
    V6000_3(Box<VfxRendererSettings>),
}

impl crate::Parse for VisualEffectSettingsMRendererSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = ( { Ok(Box::new(VfxRendererSettings::parse(rdr, ver)?)) } )?;
            Ok(Self::V6000_3(v))
        } else {
            let v = ( { Ok(Box::new(VfxRendererSettings::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct VisualEffectSettings {
    pub m_renderer_settings: VisualEffectSettingsMRendererSettingsVariant,
    pub m_culling_flags: i32,
    pub m_update_mode: i32,
    pub m_pre_warm_delta_time: f32,
    pub m_pre_warm_step_count: u32,
    pub m_initial_event_name: String,
    pub m_instancing_mode: Option<i32>,
    pub m_instancing_capacity: Option<u32>,
}

impl crate::Parse for VisualEffectSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_renderer_settings = VisualEffectSettingsMRendererSettingsVariant::parse(rdr, ver)?;
        let m_culling_flags = rdr.read_i32()?;
        let m_update_mode = rdr.read_i32()?;
        let m_pre_warm_delta_time = rdr.read_f32()?;
        let m_pre_warm_step_count = rdr.read_u32()?;
        let m_initial_event_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_instancing_mode = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_instancing_capacity = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };

        Ok(VisualEffectSettings {
            m_renderer_settings,
            m_culling_flags,
            m_update_mode,
            m_pre_warm_delta_time,
            m_pre_warm_step_count,
            m_initial_event_name,
            m_instancing_mode,
            m_instancing_capacity,
        })
    }
}
