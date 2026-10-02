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
        rdr: &mut R,
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
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_agent_type_id = Some(rdr.read_i32()?);
        } else {
        }
        m_radius = Some(rdr.read_f32()?);
        m_speed = Some(rdr.read_f32()?);
        m_acceleration = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            avoidance_priority = Some(rdr.read_i32()?);
        } else {
        }
        m_angular_speed = Some(rdr.read_f32()?);
        m_stopping_distance = Some(rdr.read_f32()?);
        m_auto_traverse_off_mesh_link = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_auto_braking = Some(rdr.read_bool()?);
        } else {
        }
        m_auto_repath = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_height = Some(rdr.read_f32()?);
        m_base_offset = Some(rdr.read_f32()?);
        m_walkable_mask = Some(rdr.read_u32()?);
        m_obstacle_avoidance_type = Some(rdr.read_i32()?);

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

impl crate::ToValue for NavMeshAgent {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Radius".to_string(), self.m_radius.to_value())),
            crate::opt_field("m_AgentTypeID", &self.m_agent_type_id),
            Some(("m_Speed".to_string(), self.m_speed.to_value())),
            Some(("m_Acceleration".to_string(), self.m_acceleration.to_value())),
            Some((
                "m_AngularSpeed".to_string(),
                self.m_angular_speed.to_value(),
            )),
            crate::opt_field("avoidancePriority", &self.avoidance_priority),
            Some((
                "m_StoppingDistance".to_string(),
                self.m_stopping_distance.to_value(),
            )),
            Some((
                "m_AutoTraverseOffMeshLink".to_string(),
                self.m_auto_traverse_off_mesh_link.to_value(),
            )),
            Some(("m_AutoRepath".to_string(), self.m_auto_repath.to_value())),
            Some(("m_Height".to_string(), self.m_height.to_value())),
            crate::opt_field("m_AutoBraking", &self.m_auto_braking),
            Some(("m_BaseOffset".to_string(), self.m_base_offset.to_value())),
            Some((
                "m_WalkableMask".to_string(),
                self.m_walkable_mask.to_value(),
            )),
            Some((
                "m_ObstacleAvoidanceType".to_string(),
                self.m_obstacle_avoidance_type.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Radius" => (self.m_radius).inspect_value(path, offset, limit),
                "m_AgentTypeID" => (self.m_agent_type_id).inspect_value(path, offset, limit),
                "m_Speed" => (self.m_speed).inspect_value(path, offset, limit),
                "m_Acceleration" => (self.m_acceleration).inspect_value(path, offset, limit),
                "m_AngularSpeed" => (self.m_angular_speed).inspect_value(path, offset, limit),
                "avoidancePriority" => (self.avoidance_priority).inspect_value(path, offset, limit),
                "m_StoppingDistance" => {
                    (self.m_stopping_distance).inspect_value(path, offset, limit)
                }
                "m_AutoTraverseOffMeshLink" => {
                    (self.m_auto_traverse_off_mesh_link).inspect_value(path, offset, limit)
                }
                "m_AutoRepath" => (self.m_auto_repath).inspect_value(path, offset, limit),
                "m_Height" => (self.m_height).inspect_value(path, offset, limit),
                "m_AutoBraking" => (self.m_auto_braking).inspect_value(path, offset, limit),
                "m_BaseOffset" => (self.m_base_offset).inspect_value(path, offset, limit),
                "m_WalkableMask" => (self.m_walkable_mask).inspect_value(path, offset, limit),
                "m_ObstacleAvoidanceType" => {
                    (self.m_obstacle_avoidance_type).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_radius).inspect_value(path, offset, limit))
                .map(|v| ("m_Radius".to_string(), v)),
            ((self.m_agent_type_id).inspect_value(path, offset, limit))
                .map(|v| ("m_AgentTypeID".to_string(), v)),
            ((self.m_speed).inspect_value(path, offset, limit)).map(|v| ("m_Speed".to_string(), v)),
            ((self.m_acceleration).inspect_value(path, offset, limit))
                .map(|v| ("m_Acceleration".to_string(), v)),
            ((self.m_angular_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularSpeed".to_string(), v)),
            ((self.avoidance_priority).inspect_value(path, offset, limit))
                .map(|v| ("avoidancePriority".to_string(), v)),
            ((self.m_stopping_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_StoppingDistance".to_string(), v)),
            ((self.m_auto_traverse_off_mesh_link).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoTraverseOffMeshLink".to_string(), v)),
            ((self.m_auto_repath).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoRepath".to_string(), v)),
            ((self.m_height).inspect_value(path, offset, limit))
                .map(|v| ("m_Height".to_string(), v)),
            ((self.m_auto_braking).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoBraking".to_string(), v)),
            ((self.m_base_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_BaseOffset".to_string(), v)),
            ((self.m_walkable_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_WalkableMask".to_string(), v)),
            ((self.m_obstacle_avoidance_type).inspect_value(path, offset, limit))
                .map(|v| ("m_ObstacleAvoidanceType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
