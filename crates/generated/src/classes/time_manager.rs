//! 自动生成。请勿手动修改。

use crate::subclasses::rational_time::RationalTime;


/// 版本枚举：TimeManagerFixedTimestepVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TimeManagerFixedTimestepVariant {
    Default(f32),
    V2023_1(RationalTime),
}

impl crate::Parse for TimeManagerFixedTimestepVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = RationalTime::parse(r, ver)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = r.read_f32()?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let fixed_timestep = TimeManagerFixedTimestepVariant::parse(r, ver)?;
        let maximum_allowed_timestep = r.read_f32()?;
        let m_time_scale = r.read_f32()?;
        let maximum_particle_timestep = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_f32()?)
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
