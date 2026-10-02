//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

/// 版本枚举：PlatformEffector2DMUseOneWayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlatformEffector2DMUseOneWayVariant {
    Default(bool),
    V5_1_2(bool),
    V5_2_4(bool),
}

impl crate::Parse for PlatformEffector2DMUseOneWayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 2, 4) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_2_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 1, 2) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_1_2(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlatformEffector2DMUseOneWayVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_1_2(v) => v.to_value(),
            Self::V5_2_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_1_2(v) => (v).inspect_value(path, offset, limit),
            Self::V5_2_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PlatformEffector2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_collider_mask: BitField,
    pub m_use_collider_mask: Option<bool>,
    pub m_one_way: Option<bool>,
    pub m_use_one_way: Option<PlatformEffector2DMUseOneWayVariant>,
    pub m_side_friction: Option<bool>,
    pub m_use_side_friction: Option<bool>,
    pub m_rotational_offset: Option<f32>,
    pub m_side_bounce: Option<bool>,
    pub m_use_side_bounce: Option<bool>,
    pub m_surface_arc: Option<f32>,
    pub m_use_one_way_grouping: Option<bool>,
    pub m_side_angle_variance: Option<f32>,
    pub m_side_arc: Option<f32>,
}

impl crate::Parse for PlatformEffector2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_collider_mask = None;
        let mut m_use_collider_mask = None;
        let mut m_one_way = None;
        let mut m_use_one_way = None;
        let mut m_side_friction = None;
        let mut m_use_side_friction = None;
        let mut m_rotational_offset = None;
        let mut m_side_bounce = None;
        let mut m_use_side_bounce = None;
        let mut m_surface_arc = None;
        let mut m_use_one_way_grouping = None;
        let mut m_side_angle_variance = None;
        let mut m_side_arc = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 2) {
            m_use_collider_mask = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_collider_mask = Some(BitField::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_rotational_offset = Some(rdr.read_f32()?);
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(rdr, ver)?);
            m_use_one_way_grouping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_surface_arc = Some(rdr.read_f32()?);
            m_use_side_friction = Some(rdr.read_bool()?);
            m_use_side_bounce = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_side_arc = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 4) {
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(rdr, ver)?);
            m_use_one_way_grouping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_surface_arc = Some(rdr.read_f32()?);
            m_use_side_friction = Some(rdr.read_bool()?);
            m_use_side_bounce = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_side_arc = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 2) {
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(rdr, ver)?);
            m_surface_arc = Some(rdr.read_f32()?);
            m_use_side_friction = Some(rdr.read_bool()?);
            m_use_side_bounce = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_side_arc = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 2) {
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(rdr, ver)?);
            m_surface_arc = Some(rdr.read_f32()?);
            m_use_side_friction = Some(rdr.read_bool()?);
            m_use_side_bounce = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_use_one_way = Some(PlatformEffector2DMUseOneWayVariant::parse(rdr, ver)?);
            m_use_side_friction = Some(rdr.read_bool()?);
            m_use_side_bounce = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_side_angle_variance = Some(rdr.read_f32()?);
        } else {
            m_one_way = Some(rdr.read_bool()?);
            m_side_friction = Some(rdr.read_bool()?);
            m_side_bounce = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_side_angle_variance = Some(rdr.read_f32()?);
        }

        Ok(PlatformEffector2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_collider_mask: m_collider_mask.unwrap(),
            m_use_collider_mask,
            m_one_way,
            m_use_one_way,
            m_side_friction,
            m_use_side_friction,
            m_rotational_offset,
            m_side_bounce,
            m_use_side_bounce,
            m_surface_arc,
            m_use_one_way_grouping,
            m_side_angle_variance,
            m_side_arc,
        })
    }
}

impl crate::ToValue for PlatformEffector2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_ColliderMask".to_string(),
                self.m_collider_mask.to_value(),
            )),
            crate::opt_field("m_UseColliderMask", &self.m_use_collider_mask),
            crate::opt_field("m_OneWay", &self.m_one_way),
            crate::opt_field("m_UseOneWay", &self.m_use_one_way),
            crate::opt_field("m_SideFriction", &self.m_side_friction),
            crate::opt_field("m_UseSideFriction", &self.m_use_side_friction),
            crate::opt_field("m_RotationalOffset", &self.m_rotational_offset),
            crate::opt_field("m_SideBounce", &self.m_side_bounce),
            crate::opt_field("m_UseSideBounce", &self.m_use_side_bounce),
            crate::opt_field("m_SurfaceArc", &self.m_surface_arc),
            crate::opt_field("m_UseOneWayGrouping", &self.m_use_one_way_grouping),
            crate::opt_field("m_SideAngleVariance", &self.m_side_angle_variance),
            crate::opt_field("m_SideArc", &self.m_side_arc),
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
                "m_ColliderMask" => (self.m_collider_mask).inspect_value(path, offset, limit),
                "m_UseColliderMask" => {
                    (self.m_use_collider_mask).inspect_value(path, offset, limit)
                }
                "m_OneWay" => (self.m_one_way).inspect_value(path, offset, limit),
                "m_UseOneWay" => (self.m_use_one_way).inspect_value(path, offset, limit),
                "m_SideFriction" => (self.m_side_friction).inspect_value(path, offset, limit),
                "m_UseSideFriction" => {
                    (self.m_use_side_friction).inspect_value(path, offset, limit)
                }
                "m_RotationalOffset" => {
                    (self.m_rotational_offset).inspect_value(path, offset, limit)
                }
                "m_SideBounce" => (self.m_side_bounce).inspect_value(path, offset, limit),
                "m_UseSideBounce" => (self.m_use_side_bounce).inspect_value(path, offset, limit),
                "m_SurfaceArc" => (self.m_surface_arc).inspect_value(path, offset, limit),
                "m_UseOneWayGrouping" => {
                    (self.m_use_one_way_grouping).inspect_value(path, offset, limit)
                }
                "m_SideAngleVariance" => {
                    (self.m_side_angle_variance).inspect_value(path, offset, limit)
                }
                "m_SideArc" => (self.m_side_arc).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_collider_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderMask".to_string(), v)),
            ((self.m_use_collider_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_UseColliderMask".to_string(), v)),
            ((self.m_one_way).inspect_value(path, offset, limit))
                .map(|v| ("m_OneWay".to_string(), v)),
            ((self.m_use_one_way).inspect_value(path, offset, limit))
                .map(|v| ("m_UseOneWay".to_string(), v)),
            ((self.m_side_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_SideFriction".to_string(), v)),
            ((self.m_use_side_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_UseSideFriction".to_string(), v)),
            ((self.m_rotational_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_RotationalOffset".to_string(), v)),
            ((self.m_side_bounce).inspect_value(path, offset, limit))
                .map(|v| ("m_SideBounce".to_string(), v)),
            ((self.m_use_side_bounce).inspect_value(path, offset, limit))
                .map(|v| ("m_UseSideBounce".to_string(), v)),
            ((self.m_surface_arc).inspect_value(path, offset, limit))
                .map(|v| ("m_SurfaceArc".to_string(), v)),
            ((self.m_use_one_way_grouping).inspect_value(path, offset, limit))
                .map(|v| ("m_UseOneWayGrouping".to_string(), v)),
            ((self.m_side_angle_variance).inspect_value(path, offset, limit))
                .map(|v| ("m_SideAngleVariance".to_string(), v)),
            ((self.m_side_arc).inspect_value(path, offset, limit))
                .map(|v| ("m_SideArc".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
