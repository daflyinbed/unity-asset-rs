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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_radius = None;
        let mut m_shape = None;
        let mut m_height = None;
        let mut m_extents = None;
        let mut m_move_threshold = None;
        let mut m_carve = None;
        let mut m_carve_only_stationary = None;
        let mut m_center = None;
        let mut m_time_to_stationary = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_shape = Some(rdr.read_i32()?);
            m_extents = Some(Vector3f::parse(rdr, ver)?);
            m_move_threshold = Some(rdr.read_f32()?);
            m_carve = Some(rdr.read_bool()?);
            m_carve_only_stationary = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_center = Some(Vector3f::parse(rdr, ver)?);
            m_time_to_stationary = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_radius = Some(rdr.read_f32()?);
            m_height = Some(rdr.read_f32()?);
            m_move_threshold = Some(rdr.read_f32()?);
            m_carve = Some(rdr.read_bool()?);
        } else {
            m_radius = Some(rdr.read_f32()?);
            m_height = Some(rdr.read_f32()?);
        }

        Ok(NavMeshObstacle {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
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
