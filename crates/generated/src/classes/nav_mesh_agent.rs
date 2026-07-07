//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct NavMeshAgent {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_radius: f32,
    pub m_agent_type_id: Option<i32>,
    pub m_speed: f32,
    pub m_acceleration: f32,
    pub m_angular_speed: f32,
    pub avoidance_priority: Option<i32>,
    pub m_stopping_distance: f32,
    pub m_auto_traverse_off_mesh_link: bool,
    pub m_auto_repath: bool,
    pub m_height: f32,
    pub m_auto_braking: Option<bool>,
    pub m_base_offset: f32,
    pub m_walkable_mask: u32,
    pub m_obstacle_avoidance_type: i32,
}

impl crate::Parse for NavMeshAgent {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_radius = r.read_f32()?;
        let m_agent_type_id = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_speed = r.read_f32()?;
        let m_acceleration = r.read_f32()?;
        let m_angular_speed = r.read_f32()?;
        let avoidance_priority = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_stopping_distance = r.read_f32()?;
        let m_auto_traverse_off_mesh_link = r.read_bool()?;
        let m_auto_repath = r.read_bool()?;
        r.align(4)?;
        let m_height = r.read_f32()?;
        let m_auto_braking = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_base_offset = r.read_f32()?;
        let m_walkable_mask = r.read_u32()?;
        let m_obstacle_avoidance_type = r.read_i32()?;

        Ok(NavMeshAgent {
            m_game_object,
            m_enabled,
            m_radius,
            m_agent_type_id,
            m_speed,
            m_acceleration,
            m_angular_speed,
            avoidance_priority,
            m_stopping_distance,
            m_auto_traverse_off_mesh_link,
            m_auto_repath,
            m_height,
            m_auto_braking,
            m_base_offset,
            m_walkable_mask,
            m_obstacle_avoidance_type,
        })
    }
}
