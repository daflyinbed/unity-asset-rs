//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct VfxManager {
    pub m_object_hide_flags: u32,
    pub m_indirect_shader: PPtr,
    pub m_copy_buffer_shader: PPtr,
    pub m_sort_shader: PPtr,
    pub m_render_pipe_settings_path: String,
    pub m_fixed_time_step: f32,
    pub m_max_delta_time: f32,
}

impl crate::Parse for VfxManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_indirect_shader = crate::PPtr::parse(r, ver)?;
        let m_copy_buffer_shader = crate::PPtr::parse(r, ver)?;
        let m_sort_shader = crate::PPtr::parse(r, ver)?;
        let m_render_pipe_settings_path = r.read_aligned_string()?;
        r.align(4)?;
        let m_fixed_time_step = r.read_f32()?;
        let m_max_delta_time = r.read_f32()?;

        Ok(VfxManager {
            m_object_hide_flags,
            m_indirect_shader,
            m_copy_buffer_shader,
            m_sort_shader,
            m_render_pipe_settings_path,
            m_fixed_time_step,
            m_max_delta_time,
        })
    }
}
