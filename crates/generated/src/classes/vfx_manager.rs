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
            rdr.align(4)?;
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
