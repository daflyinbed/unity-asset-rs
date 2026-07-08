//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;


/// 版本枚举：ParticleSystemEmissionBurstCycleCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemEmissionBurstCycleCountVariant {
    Default(u32),
    V2017_3(i32),
}

impl crate::Parse for ParticleSystemEmissionBurstCycleCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2017_3(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ParticleSystemEmissionBurst {
    pub time: f32,
    pub min_count: Option<u32>,
    pub count_curve: Option<MinMaxCurve>,
    pub max_count: Option<u32>,
    pub cycle_count: ParticleSystemEmissionBurstCycleCountVariant,
    pub repeat_interval: f32,
    pub probability: Option<f32>,
}

impl crate::Parse for ParticleSystemEmissionBurst {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = rdr.read_f32()?;
        let min_count = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let count_curve = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = MinMaxCurve::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let max_count = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let cycle_count = ParticleSystemEmissionBurstCycleCountVariant::parse(rdr, ver)?;
        let repeat_interval = rdr.read_f32()?;
        let probability = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(ParticleSystemEmissionBurst {
            time,
            min_count,
            count_curve,
            max_count,
            cycle_count,
            repeat_interval,
            probability,
        })
    }
}
