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
    pub m_instancing_mode: Option<i32>,
    pub m_instancing_capacity: Option<u32>,
}

impl crate::Parse for VisualEffectSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_renderer_settings = VfxRendererSettings::parse(rdr, ver)?;
        let m_culling_flags = rdr.read_i32()?;
        let m_update_mode = rdr.read_i32()?;
        let m_pre_warm_delta_time = rdr.read_f32()?;
        let m_pre_warm_step_count = rdr.read_u32()?;
        let m_initial_event_name = rdr.read_aligned_string()?;
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

impl crate::ToValue for VisualEffectSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_RendererSettings".to_string(),
                self.m_renderer_settings.to_value(),
            )),
            Some((
                "m_CullingFlags".to_string(),
                self.m_culling_flags.to_value(),
            )),
            Some(("m_UpdateMode".to_string(), self.m_update_mode.to_value())),
            Some((
                "m_PreWarmDeltaTime".to_string(),
                self.m_pre_warm_delta_time.to_value(),
            )),
            Some((
                "m_PreWarmStepCount".to_string(),
                self.m_pre_warm_step_count.to_value(),
            )),
            Some((
                "m_InitialEventName".to_string(),
                self.m_initial_event_name.to_value(),
            )),
            crate::opt_field("m_InstancingMode", &self.m_instancing_mode),
            crate::opt_field("m_InstancingCapacity", &self.m_instancing_capacity),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_RendererSettings" => {
                    (self.m_renderer_settings).inspect_value(path, offset, limit)
                }
                "m_CullingFlags" => (self.m_culling_flags).inspect_value(path, offset, limit),
                "m_UpdateMode" => (self.m_update_mode).inspect_value(path, offset, limit),
                "m_PreWarmDeltaTime" => {
                    (self.m_pre_warm_delta_time).inspect_value(path, offset, limit)
                }
                "m_PreWarmStepCount" => {
                    (self.m_pre_warm_step_count).inspect_value(path, offset, limit)
                }
                "m_InitialEventName" => {
                    (self.m_initial_event_name).inspect_value(path, offset, limit)
                }
                "m_InstancingMode" => (self.m_instancing_mode).inspect_value(path, offset, limit),
                "m_InstancingCapacity" => {
                    (self.m_instancing_capacity).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_renderer_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_RendererSettings".to_string(), v)),
            ((self.m_culling_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_CullingFlags".to_string(), v)),
            ((self.m_update_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateMode".to_string(), v)),
            ((self.m_pre_warm_delta_time).inspect_value(path, offset, limit))
                .map(|v| ("m_PreWarmDeltaTime".to_string(), v)),
            ((self.m_pre_warm_step_count).inspect_value(path, offset, limit))
                .map(|v| ("m_PreWarmStepCount".to_string(), v)),
            ((self.m_initial_event_name).inspect_value(path, offset, limit))
                .map(|v| ("m_InitialEventName".to_string(), v)),
            ((self.m_instancing_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_InstancingMode".to_string(), v)),
            ((self.m_instancing_capacity).inspect_value(path, offset, limit))
                .map(|v| ("m_InstancingCapacity".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
