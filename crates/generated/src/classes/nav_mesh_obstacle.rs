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

impl crate::ToValue for NavMeshObstacle {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            crate::opt_field("m_Radius", &self.m_radius),
            crate::opt_field("m_Shape", &self.m_shape),
            crate::opt_field("m_Height", &self.m_height),
            crate::opt_field("m_Extents", &self.m_extents),
            crate::opt_field("m_MoveThreshold", &self.m_move_threshold),
            crate::opt_field("m_Carve", &self.m_carve),
            crate::opt_field("m_CarveOnlyStationary", &self.m_carve_only_stationary),
            crate::opt_field("m_Center", &self.m_center),
            crate::opt_field("m_TimeToStationary", &self.m_time_to_stationary),
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
                "m_Shape" => (self.m_shape).inspect_value(path, offset, limit),
                "m_Height" => (self.m_height).inspect_value(path, offset, limit),
                "m_Extents" => (self.m_extents).inspect_value(path, offset, limit),
                "m_MoveThreshold" => (self.m_move_threshold).inspect_value(path, offset, limit),
                "m_Carve" => (self.m_carve).inspect_value(path, offset, limit),
                "m_CarveOnlyStationary" => {
                    (self.m_carve_only_stationary).inspect_value(path, offset, limit)
                }
                "m_Center" => (self.m_center).inspect_value(path, offset, limit),
                "m_TimeToStationary" => {
                    (self.m_time_to_stationary).inspect_value(path, offset, limit)
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
            ((self.m_shape).inspect_value(path, offset, limit)).map(|v| ("m_Shape".to_string(), v)),
            ((self.m_height).inspect_value(path, offset, limit))
                .map(|v| ("m_Height".to_string(), v)),
            ((self.m_extents).inspect_value(path, offset, limit))
                .map(|v| ("m_Extents".to_string(), v)),
            ((self.m_move_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_MoveThreshold".to_string(), v)),
            ((self.m_carve).inspect_value(path, offset, limit)).map(|v| ("m_Carve".to_string(), v)),
            ((self.m_carve_only_stationary).inspect_value(path, offset, limit))
                .map(|v| ("m_CarveOnlyStationary".to_string(), v)),
            ((self.m_center).inspect_value(path, offset, limit))
                .map(|v| ("m_Center".to_string(), v)),
            ((self.m_time_to_stationary).inspect_value(path, offset, limit))
                .map(|v| ("m_TimeToStationary".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
