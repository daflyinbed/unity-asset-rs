//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct OffMeshLink {
    pub m_game_object: PPtr,
    pub m_start: PPtr,
    pub m_nav_mesh_layer: Option<u32>,
    pub m_enabled: Option<u8>,
    pub m_end: PPtr,
    pub m_area_index: Option<u32>,
    pub m_dt_poly_ref: Option<u32>,
    pub m_agent_type_id: Option<i32>,
    pub m_cost_override: f32,
    pub m_bi_directional: bool,
    pub m_activated: bool,
    pub m_auto_update_positions: Option<bool>,
}

impl crate::Parse for OffMeshLink {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_start = None;
        let mut m_nav_mesh_layer = None;
        let mut m_enabled = None;
        let mut m_end = None;
        let mut m_area_index = None;
        let mut m_dt_poly_ref = None;
        let mut m_agent_type_id = None;
        let mut m_cost_override = None;
        let mut m_bi_directional = None;
        let mut m_activated = None;
        let mut m_auto_update_positions = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_area_index = Some(rdr.read_u32()?);
            m_agent_type_id = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_area_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_nav_mesh_layer = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_nav_mesh_layer = Some(rdr.read_u32()?);
        } else {
        }
        m_start = Some(crate::PPtr::parse(rdr, ver)?);
        m_end = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_dt_poly_ref = Some(rdr.read_u32()?);
        }
        m_cost_override = Some(rdr.read_f32()?);
        rdr.align(4)?;
        m_bi_directional = Some(rdr.read_bool()?);
        m_activated = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_auto_update_positions = Some(rdr.read_bool()?);
        } else {
        }

        Ok(OffMeshLink {
            m_game_object: m_game_object.unwrap(),
            m_start: m_start.unwrap(),
            m_nav_mesh_layer,
            m_enabled,
            m_end: m_end.unwrap(),
            m_area_index,
            m_dt_poly_ref,
            m_agent_type_id,
            m_cost_override: m_cost_override.unwrap(),
            m_bi_directional: m_bi_directional.unwrap(),
            m_activated: m_activated.unwrap(),
            m_auto_update_positions,
        })
    }
}

impl crate::ToValue for OffMeshLink {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Start".to_string(), self.m_start.to_value())),
            crate::opt_field("m_NavMeshLayer", &self.m_nav_mesh_layer),
            crate::opt_field("m_Enabled", &self.m_enabled),
            Some(("m_End".to_string(), self.m_end.to_value())),
            crate::opt_field("m_AreaIndex", &self.m_area_index),
            crate::opt_field("m_DtPolyRef", &self.m_dt_poly_ref),
            crate::opt_field("m_AgentTypeID", &self.m_agent_type_id),
            Some((
                "m_CostOverride".to_string(),
                self.m_cost_override.to_value(),
            )),
            Some((
                "m_BiDirectional".to_string(),
                self.m_bi_directional.to_value(),
            )),
            Some(("m_Activated".to_string(), self.m_activated.to_value())),
            crate::opt_field("m_AutoUpdatePositions", &self.m_auto_update_positions),
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
                "m_Start" => (self.m_start).inspect_value(path, offset, limit),
                "m_NavMeshLayer" => (self.m_nav_mesh_layer).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_End" => (self.m_end).inspect_value(path, offset, limit),
                "m_AreaIndex" => (self.m_area_index).inspect_value(path, offset, limit),
                "m_DtPolyRef" => (self.m_dt_poly_ref).inspect_value(path, offset, limit),
                "m_AgentTypeID" => (self.m_agent_type_id).inspect_value(path, offset, limit),
                "m_CostOverride" => (self.m_cost_override).inspect_value(path, offset, limit),
                "m_BiDirectional" => (self.m_bi_directional).inspect_value(path, offset, limit),
                "m_Activated" => (self.m_activated).inspect_value(path, offset, limit),
                "m_AutoUpdatePositions" => {
                    (self.m_auto_update_positions).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_start).inspect_value(path, offset, limit)).map(|v| ("m_Start".to_string(), v)),
            ((self.m_nav_mesh_layer).inspect_value(path, offset, limit))
                .map(|v| ("m_NavMeshLayer".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_end).inspect_value(path, offset, limit)).map(|v| ("m_End".to_string(), v)),
            ((self.m_area_index).inspect_value(path, offset, limit))
                .map(|v| ("m_AreaIndex".to_string(), v)),
            ((self.m_dt_poly_ref).inspect_value(path, offset, limit))
                .map(|v| ("m_DtPolyRef".to_string(), v)),
            ((self.m_agent_type_id).inspect_value(path, offset, limit))
                .map(|v| ("m_AgentTypeID".to_string(), v)),
            ((self.m_cost_override).inspect_value(path, offset, limit))
                .map(|v| ("m_CostOverride".to_string(), v)),
            ((self.m_bi_directional).inspect_value(path, offset, limit))
                .map(|v| ("m_BiDirectional".to_string(), v)),
            ((self.m_activated).inspect_value(path, offset, limit))
                .map(|v| ("m_Activated".to_string(), v)),
            ((self.m_auto_update_positions).inspect_value(path, offset, limit))
                .map(|v| ("m_AutoUpdatePositions".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
