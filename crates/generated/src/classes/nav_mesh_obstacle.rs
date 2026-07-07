//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct NavMeshObstacle {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_radius: Option<f32>,
    pub m_shape: Option<i32>,
    pub m_height: Option<f32>,
    pub m_extents: Option<Vector3f>,
    pub m_move_threshold: Option<f32>,
    pub m_carve: Option<bool>,
    pub m_carve_only_stationary: Option<bool>,
    pub m_center: Option<Vector3f>,
    pub m_time_to_stationary: Option<f32>,
}

impl crate::Parse for NavMeshObstacle {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_radius = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_shape = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_height = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_extents = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_move_threshold = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_carve = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_carve_only_stationary = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_center = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_time_to_stationary = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };

        Ok(NavMeshObstacle {
            m_game_object,
            m_enabled,
            m_radius,
            m_shape,
            m_height,
            m_extents,
            m_move_threshold,
            m_carve,
            m_carve_only_stationary,
            m_center,
            m_time_to_stationary,
        })
    }
}
