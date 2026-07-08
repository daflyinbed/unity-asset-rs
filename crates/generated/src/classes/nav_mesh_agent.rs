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
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_radius = None;
        let mut m_agent_type_id = None;
        let mut m_speed = None;
        let mut m_acceleration = None;
        let mut m_angular_speed = None;
        let mut avoidance_priority = None;
        let mut m_stopping_distance = None;
        let mut m_auto_traverse_off_mesh_link = None;
        let mut m_auto_repath = None;
        let mut m_height = None;
        let mut m_auto_braking = None;
        let mut m_base_offset = None;
        let mut m_walkable_mask = None;
        let mut m_obstacle_avoidance_type = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_agent_type_id = Some(r.read_i32()?);
        } else {
        }
            m_radius = Some(r.read_f32()?);
            m_speed = Some(r.read_f32()?);
            m_acceleration = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            avoidance_priority = Some(r.read_i32()?);
        } else {
        }
            m_angular_speed = Some(r.read_f32()?);
            m_stopping_distance = Some(r.read_f32()?);
            m_auto_traverse_off_mesh_link = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_auto_braking = Some(r.read_bool()?);
        } else {
        }
            m_auto_repath = Some(r.read_bool()?);
            r.align(4)?;
            m_height = Some(r.read_f32()?);
            m_base_offset = Some(r.read_f32()?);
            m_walkable_mask = Some(r.read_u32()?);
            m_obstacle_avoidance_type = Some(r.read_i32()?);

        Ok(NavMeshAgent {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_radius: m_radius.unwrap(),
            m_agent_type_id,
            m_speed: m_speed.unwrap(),
            m_acceleration: m_acceleration.unwrap(),
            m_angular_speed: m_angular_speed.unwrap(),
            avoidance_priority,
            m_stopping_distance: m_stopping_distance.unwrap(),
            m_auto_traverse_off_mesh_link: m_auto_traverse_off_mesh_link.unwrap(),
            m_auto_repath: m_auto_repath.unwrap(),
            m_height: m_height.unwrap(),
            m_auto_braking,
            m_base_offset: m_base_offset.unwrap(),
            m_walkable_mask: m_walkable_mask.unwrap(),
            m_obstacle_avoidance_type: m_obstacle_avoidance_type.unwrap(),
        })
    }
}
