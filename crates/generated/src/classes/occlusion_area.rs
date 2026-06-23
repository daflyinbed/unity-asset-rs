//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;
use super::component::Component;

#[derive(Debug, Clone)]
pub struct OcclusionArea {
    pub parent: Component,
    pub m_game_object: PPtr,
    pub m_size: Vector3f,
    pub m_center: Vector3f,
    pub m_is_view_volume: bool,
    pub m_is_target_volume: Option<bool>,
    pub m_target_resolution: Option<i32>,
}

impl crate::Parse for OcclusionArea {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Component::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_size = Vector3f::parse(r, ver)?;
        let m_center = Vector3f::parse(r, ver)?;
        let m_is_view_volume = r.read_bool()?;
        let m_is_target_volume = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_target_resolution = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(OcclusionArea {
            parent,
            m_game_object,
            m_size,
            m_center,
            m_is_view_volume,
            m_is_target_volume,
            m_target_resolution,
        })
    }
}
