//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct VfxManager {
    pub parent: GlobalGameManager,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_indirect_shader = crate::PPtr::parse(r, ver)?;
        let m_copy_buffer_shader = crate::PPtr::parse(r, ver)?;
        let m_sort_shader = crate::PPtr::parse(r, ver)?;
        let m_prefix_sum_shader = if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_render_pipe_settings_path = r.read_aligned_string()?;
        r.align(4)?;
        let m_strip_update_shader = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_fixed_time_step = r.read_f32()?;
        let m_empty_shader = if *ver >= crate::UnityVersion::new(2021, 3, 32) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_max_delta_time = r.read_f32()?;
        let m_compiled_version = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_max_scrub_time = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_runtime_version = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_max_capacity = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_runtime_resources = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_batch_empty_lifetime = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };

        Ok(VfxManager {
            parent,
            m_indirect_shader,
            m_copy_buffer_shader,
            m_sort_shader,
            m_prefix_sum_shader,
            m_render_pipe_settings_path,
            m_strip_update_shader,
            m_fixed_time_step,
            m_empty_shader,
            m_max_delta_time,
            m_compiled_version,
            m_max_scrub_time,
            m_runtime_version,
            m_max_capacity,
            m_runtime_resources,
            m_batch_empty_lifetime,
        })
    }
}
