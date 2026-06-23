//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct OcclusionArea {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_size: Vector3f,
    pub m_center: Vector3f,
    pub m_is_view_volume: bool,
    pub m_is_target_volume: bool,
    pub m_target_resolution: i32,
}

impl crate::Parse for OcclusionArea {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_size = Vector3f::parse(r, ver)?;
        let m_center = Vector3f::parse(r, ver)?;
        let m_is_view_volume = r.read_bool()?;
        let m_is_target_volume = r.read_bool()?;
        r.align(4)?;
        let m_target_resolution = r.read_i32()?;

        Ok(OcclusionArea {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_size,
            m_center,
            m_is_view_volume,
            m_is_target_volume,
            m_target_resolution,
        })
    }
}
