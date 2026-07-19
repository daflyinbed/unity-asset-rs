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
        let mut time = None;
        let mut min_count = None;
        let mut count_curve = None;
        let mut max_count = None;
        let mut cycle_count = None;
        let mut repeat_interval = None;
        let mut probability = None;
        time = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            count_curve = Some(MinMaxCurve::parse(rdr, ver)?);
        } else {
            min_count = Some(rdr.read_u32()?);
            max_count = Some(rdr.read_u32()?);
        }
        cycle_count = Some(ParticleSystemEmissionBurstCycleCountVariant::parse(
            rdr, ver,
        )?);
        repeat_interval = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            probability = Some(rdr.read_f32()?);
        } else {
        }

        Ok(ParticleSystemEmissionBurst {
            time: time.unwrap(),
            min_count,
            count_curve,
            max_count,
            cycle_count: cycle_count.unwrap(),
            repeat_interval: repeat_interval.unwrap(),
            probability,
        })
    }
}
