//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct VfxManager {
    pub m_indirect_shader: PPtr,
    pub m_copy_buffer_shader: PPtr,
    pub m_sort_shader: PPtr,
    pub m_prefix_sum_shader: Option<PPtr>,
    pub m_render_pipe_settings_path: String,
    pub m_strip_update_shader: Option<PPtr>,
    pub m_fixed_time_step: f32,
    pub m_empty_shader: Option<PPtr>,
    pub m_max_delta_time: f32,
    pub m_compiled_version: Option<u32>,
    pub m_max_scrub_time: Option<f32>,
    pub m_runtime_version: Option<u32>,
    pub m_max_capacity: Option<u32>,
    pub m_runtime_resources: Option<PPtr>,
    pub m_batch_empty_lifetime: Option<u32>,
}

impl crate::Parse for VfxManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_indirect_shader = None;
        let mut m_copy_buffer_shader = None;
        let mut m_sort_shader = None;
        let mut m_prefix_sum_shader = None;
        let mut m_render_pipe_settings_path = None;
        let mut m_strip_update_shader = None;
        let mut m_fixed_time_step = None;
        let mut m_empty_shader = None;
        let mut m_max_delta_time = None;
        let mut m_compiled_version = None;
        let mut m_max_scrub_time = None;
        let mut m_runtime_version = None;
        let mut m_max_capacity = None;
        let mut m_runtime_resources = None;
        let mut m_batch_empty_lifetime = None;
        m_indirect_shader = Some(crate::PPtr::parse(rdr, ver)?);
        m_copy_buffer_shader = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_prefix_sum_shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_sort_shader = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 17) {
            m_strip_update_shader = Some(crate::PPtr::parse(rdr, ver)?);
            m_empty_shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_strip_update_shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            m_strip_update_shader = Some(crate::PPtr::parse(rdr, ver)?);
            m_empty_shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_strip_update_shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 32) {
            m_strip_update_shader = Some(crate::PPtr::parse(rdr, ver)?);
            m_empty_shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_strip_update_shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_render_pipe_settings_path = Some(rdr.read_aligned_string()?);
        m_fixed_time_step = Some(rdr.read_f32()?);
        m_max_delta_time = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_max_scrub_time = Some(rdr.read_f32()?);
            m_max_capacity = Some(rdr.read_u32()?);
            m_compiled_version = Some(rdr.read_u32()?);
            m_runtime_version = Some(rdr.read_u32()?);
            m_runtime_resources = Some(crate::PPtr::parse(rdr, ver)?);
            m_batch_empty_lifetime = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_max_scrub_time = Some(rdr.read_f32()?);
            m_compiled_version = Some(rdr.read_u32()?);
            m_runtime_version = Some(rdr.read_u32()?);
            m_runtime_resources = Some(crate::PPtr::parse(rdr, ver)?);
            m_batch_empty_lifetime = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_max_scrub_time = Some(rdr.read_f32()?);
            m_compiled_version = Some(rdr.read_u32()?);
            m_runtime_version = Some(rdr.read_u32()?);
            m_runtime_resources = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_compiled_version = Some(rdr.read_u32()?);
            m_runtime_version = Some(rdr.read_u32()?);
            m_runtime_resources = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_compiled_version = Some(rdr.read_u32()?);
            m_runtime_version = Some(rdr.read_u32()?);
        } else {
        }

        Ok(VfxManager {
            m_indirect_shader: m_indirect_shader.unwrap(),
            m_copy_buffer_shader: m_copy_buffer_shader.unwrap(),
            m_sort_shader: m_sort_shader.unwrap(),
            m_prefix_sum_shader,
            m_render_pipe_settings_path: m_render_pipe_settings_path.unwrap(),
            m_strip_update_shader,
            m_fixed_time_step: m_fixed_time_step.unwrap(),
            m_empty_shader,
            m_max_delta_time: m_max_delta_time.unwrap(),
            m_compiled_version,
            m_max_scrub_time,
            m_runtime_version,
            m_max_capacity,
            m_runtime_resources,
            m_batch_empty_lifetime,
        })
    }
}

impl crate::ToValue for VfxManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_IndirectShader".to_string(),
                self.m_indirect_shader.to_value(),
            )),
            Some((
                "m_CopyBufferShader".to_string(),
                self.m_copy_buffer_shader.to_value(),
            )),
            Some(("m_SortShader".to_string(), self.m_sort_shader.to_value())),
            crate::opt_field("m_PrefixSumShader", &self.m_prefix_sum_shader),
            Some((
                "m_RenderPipeSettingsPath".to_string(),
                self.m_render_pipe_settings_path.to_value(),
            )),
            crate::opt_field("m_StripUpdateShader", &self.m_strip_update_shader),
            Some((
                "m_FixedTimeStep".to_string(),
                self.m_fixed_time_step.to_value(),
            )),
            crate::opt_field("m_EmptyShader", &self.m_empty_shader),
            Some((
                "m_MaxDeltaTime".to_string(),
                self.m_max_delta_time.to_value(),
            )),
            crate::opt_field("m_CompiledVersion", &self.m_compiled_version),
            crate::opt_field("m_MaxScrubTime", &self.m_max_scrub_time),
            crate::opt_field("m_RuntimeVersion", &self.m_runtime_version),
            crate::opt_field("m_MaxCapacity", &self.m_max_capacity),
            crate::opt_field("m_RuntimeResources", &self.m_runtime_resources),
            crate::opt_field("m_BatchEmptyLifetime", &self.m_batch_empty_lifetime),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_IndirectShader" => (self.m_indirect_shader).inspect_value(path, offset, limit),
                "m_CopyBufferShader" => {
                    (self.m_copy_buffer_shader).inspect_value(path, offset, limit)
                }
                "m_SortShader" => (self.m_sort_shader).inspect_value(path, offset, limit),
                "m_PrefixSumShader" => {
                    (self.m_prefix_sum_shader).inspect_value(path, offset, limit)
                }
                "m_RenderPipeSettingsPath" => {
                    (self.m_render_pipe_settings_path).inspect_value(path, offset, limit)
                }
                "m_StripUpdateShader" => {
                    (self.m_strip_update_shader).inspect_value(path, offset, limit)
                }
                "m_FixedTimeStep" => (self.m_fixed_time_step).inspect_value(path, offset, limit),
                "m_EmptyShader" => (self.m_empty_shader).inspect_value(path, offset, limit),
                "m_MaxDeltaTime" => (self.m_max_delta_time).inspect_value(path, offset, limit),
                "m_CompiledVersion" => (self.m_compiled_version).inspect_value(path, offset, limit),
                "m_MaxScrubTime" => (self.m_max_scrub_time).inspect_value(path, offset, limit),
                "m_RuntimeVersion" => (self.m_runtime_version).inspect_value(path, offset, limit),
                "m_MaxCapacity" => (self.m_max_capacity).inspect_value(path, offset, limit),
                "m_RuntimeResources" => {
                    (self.m_runtime_resources).inspect_value(path, offset, limit)
                }
                "m_BatchEmptyLifetime" => {
                    (self.m_batch_empty_lifetime).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_indirect_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_IndirectShader".to_string(), v)),
            ((self.m_copy_buffer_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_CopyBufferShader".to_string(), v)),
            ((self.m_sort_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_SortShader".to_string(), v)),
            ((self.m_prefix_sum_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_PrefixSumShader".to_string(), v)),
            ((self.m_render_pipe_settings_path).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderPipeSettingsPath".to_string(), v)),
            ((self.m_strip_update_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_StripUpdateShader".to_string(), v)),
            ((self.m_fixed_time_step).inspect_value(path, offset, limit))
                .map(|v| ("m_FixedTimeStep".to_string(), v)),
            ((self.m_empty_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_EmptyShader".to_string(), v)),
            ((self.m_max_delta_time).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxDeltaTime".to_string(), v)),
            ((self.m_compiled_version).inspect_value(path, offset, limit))
                .map(|v| ("m_CompiledVersion".to_string(), v)),
            ((self.m_max_scrub_time).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxScrubTime".to_string(), v)),
            ((self.m_runtime_version).inspect_value(path, offset, limit))
                .map(|v| ("m_RuntimeVersion".to_string(), v)),
            ((self.m_max_capacity).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxCapacity".to_string(), v)),
            ((self.m_runtime_resources).inspect_value(path, offset, limit))
                .map(|v| ("m_RuntimeResources".to_string(), v)),
            ((self.m_batch_empty_lifetime).inspect_value(path, offset, limit))
                .map(|v| ("m_BatchEmptyLifetime".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
