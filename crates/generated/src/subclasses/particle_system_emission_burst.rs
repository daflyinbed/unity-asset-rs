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

impl crate::ToValue for ParticleSystemEmissionBurstCycleCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ParticleSystemEmissionBurst {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("time".to_string(), self.time.to_value())),
            crate::opt_field("minCount", &self.min_count),
            crate::opt_field("countCurve", &self.count_curve),
            crate::opt_field("maxCount", &self.max_count),
            Some(("cycleCount".to_string(), self.cycle_count.to_value())),
            Some((
                "repeatInterval".to_string(),
                self.repeat_interval.to_value(),
            )),
            crate::opt_field("probability", &self.probability),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "time" => (self.time).inspect_value(path, offset, limit),
                "minCount" => (self.min_count).inspect_value(path, offset, limit),
                "countCurve" => (self.count_curve).inspect_value(path, offset, limit),
                "maxCount" => (self.max_count).inspect_value(path, offset, limit),
                "cycleCount" => (self.cycle_count).inspect_value(path, offset, limit),
                "repeatInterval" => (self.repeat_interval).inspect_value(path, offset, limit),
                "probability" => (self.probability).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.time).inspect_value(path, offset, limit)).map(|v| ("time".to_string(), v)),
            ((self.min_count).inspect_value(path, offset, limit))
                .map(|v| ("minCount".to_string(), v)),
            ((self.count_curve).inspect_value(path, offset, limit))
                .map(|v| ("countCurve".to_string(), v)),
            ((self.max_count).inspect_value(path, offset, limit))
                .map(|v| ("maxCount".to_string(), v)),
            ((self.cycle_count).inspect_value(path, offset, limit))
                .map(|v| ("cycleCount".to_string(), v)),
            ((self.repeat_interval).inspect_value(path, offset, limit))
                .map(|v| ("repeatInterval".to_string(), v)),
            ((self.probability).inspect_value(path, offset, limit))
                .map(|v| ("probability".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
