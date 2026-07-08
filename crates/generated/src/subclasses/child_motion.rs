//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ChildMotion {
    pub m_motion: PPtr,
    pub m_threshold: f32,
    pub m_position: Vector2f,
    pub m_time_scale: f32,
    pub m_cycle_offset: f32,
    pub m_direct_blend_parameter: String,
    pub m_mirror: bool,
}

impl crate::Parse for ChildMotion {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_motion = crate::PPtr::parse(rdr, ver)?;
        let m_threshold = rdr.read_f32()?;
        let m_position = Vector2f::parse(rdr, ver)?;
        let m_time_scale = rdr.read_f32()?;
        let m_cycle_offset = rdr.read_f32()?;
        let m_direct_blend_parameter = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_mirror = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(ChildMotion {
            m_motion,
            m_threshold,
            m_position,
            m_time_scale,
            m_cycle_offset,
            m_direct_blend_parameter,
            m_mirror,
        })
    }
}
