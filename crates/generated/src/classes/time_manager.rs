//! 自动生成。请勿手动修改。

use crate::subclasses::rational_time::RationalTime;

/// 版本枚举：TimeManagerFixedTimestepVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TimeManagerFixedTimestepVariant {
    Default(f32),
    V2023_1(Box<RationalTime>),
}

impl crate::Parse for TimeManagerFixedTimestepVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = ({ Ok(Box::new(RationalTime::parse(rdr, ver)?)) })?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TimeManagerFixedTimestepVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TimeManager {
    pub fixed_timestep: TimeManagerFixedTimestepVariant,
    pub maximum_allowed_timestep: f32,
    pub m_time_scale: f32,
    pub maximum_particle_timestep: Option<f32>,
}

impl crate::Parse for TimeManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let fixed_timestep = TimeManagerFixedTimestepVariant::parse(rdr, ver)?;
        let maximum_allowed_timestep = rdr.read_f32()?;
        let m_time_scale = rdr.read_f32()?;
        let maximum_particle_timestep = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(TimeManager {
            fixed_timestep,
            maximum_allowed_timestep,
            m_time_scale,
            maximum_particle_timestep,
        })
    }
}

impl crate::ToValue for TimeManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("Fixed Timestep".to_string(), self.fixed_timestep.to_value())),
            Some((
                "Maximum Allowed Timestep".to_string(),
                self.maximum_allowed_timestep.to_value(),
            )),
            Some(("m_TimeScale".to_string(), self.m_time_scale.to_value())),
            crate::opt_field("Maximum Particle Timestep", &self.maximum_particle_timestep),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "Fixed Timestep" => (self.fixed_timestep).inspect_value(path, offset, limit),
                "Maximum Allowed Timestep" => {
                    (self.maximum_allowed_timestep).inspect_value(path, offset, limit)
                }
                "m_TimeScale" => (self.m_time_scale).inspect_value(path, offset, limit),
                "Maximum Particle Timestep" => {
                    (self.maximum_particle_timestep).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.fixed_timestep).inspect_value(path, offset, limit))
                .map(|v| ("Fixed Timestep".to_string(), v)),
            ((self.maximum_allowed_timestep).inspect_value(path, offset, limit))
                .map(|v| ("Maximum Allowed Timestep".to_string(), v)),
            ((self.m_time_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_TimeScale".to_string(), v)),
            ((self.maximum_particle_timestep).inspect_value(path, offset, limit))
                .map(|v| ("Maximum Particle Timestep".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
