//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::particle_system_emission_burst::ParticleSystemEmissionBurst;

/// 版本枚举：EmissionModuleCnt0Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleCnt0Variant {
    Default(u16),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleCnt0Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleCnt0Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleMBurstCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleMBurstCountVariant {
    Default(u8),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleMBurstCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleMBurstCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleCnt1Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleCnt1Variant {
    Default(u16),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleCnt1Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleCnt1Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleMBurstsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleMBurstsVariant {
    Default(Vec<ParticleSystemEmissionBurst>),
    V2017_1(Vec<ParticleSystemEmissionBurst>),
}

impl crate::Parse for EmissionModuleMBurstsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ParticleSystemEmissionBurst::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ParticleSystemEmissionBurst::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleMBurstsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleCnt2Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleCnt2Variant {
    Default(u16),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleCnt2Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleCnt2Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleCnt3Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleCnt3Variant {
    Default(u16),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleCnt3Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleCnt3Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleCntmax0Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleCntmax0Variant {
    Default(u16),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleCntmax0Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleCntmax0Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleCntmax1Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleCntmax1Variant {
    Default(u16),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleCntmax1Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleCntmax1Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleCntmax2Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleCntmax2Variant {
    Default(u16),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleCntmax2Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleCntmax2Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：EmissionModuleCntmax3Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum EmissionModuleCntmax3Variant {
    Default(u16),
    V5_4(i32),
}

impl crate::Parse for EmissionModuleCntmax3Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for EmissionModuleCntmax3Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EmissionModule {
    pub enabled: bool,
    pub m_type: Option<i32>,
    pub rate_over_time: Option<MinMaxCurve>,
    pub rate: Option<MinMaxCurve>,
    pub rate_over_distance: Option<MinMaxCurve>,
    pub cnt0: Option<EmissionModuleCnt0Variant>,
    pub m_burst_count: EmissionModuleMBurstCountVariant,
    pub cnt1: Option<EmissionModuleCnt1Variant>,
    pub m_bursts: Option<EmissionModuleMBurstsVariant>,
    pub cnt2: Option<EmissionModuleCnt2Variant>,
    pub cnt3: Option<EmissionModuleCnt3Variant>,
    pub time0: Option<f32>,
    pub cntmax0: Option<EmissionModuleCntmax0Variant>,
    pub time1: Option<f32>,
    pub cntmax1: Option<EmissionModuleCntmax1Variant>,
    pub time2: Option<f32>,
    pub cntmax2: Option<EmissionModuleCntmax2Variant>,
    pub time3: Option<f32>,
    pub cntmax3: Option<EmissionModuleCntmax3Variant>,
}

impl crate::Parse for EmissionModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut m_type = None;
        let mut rate_over_time = None;
        let mut rate = None;
        let mut rate_over_distance = None;
        let mut cnt0 = None;
        let mut m_burst_count = None;
        let mut cnt1 = None;
        let mut m_bursts = None;
        let mut cnt2 = None;
        let mut cnt3 = None;
        let mut time0 = None;
        let mut cntmax0 = None;
        let mut time1 = None;
        let mut cntmax1 = None;
        let mut time2 = None;
        let mut cntmax2 = None;
        let mut time3 = None;
        let mut cntmax3 = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            rate_over_time = Some(MinMaxCurve::parse(rdr, ver)?);
            rate_over_distance = Some(MinMaxCurve::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            rate_over_time = Some(MinMaxCurve::parse(rdr, ver)?);
            rate_over_distance = Some(MinMaxCurve::parse(rdr, ver)?);
            cnt0 = Some(EmissionModuleCnt0Variant::parse(rdr, ver)?);
            cnt1 = Some(EmissionModuleCnt1Variant::parse(rdr, ver)?);
            cnt2 = Some(EmissionModuleCnt2Variant::parse(rdr, ver)?);
            cnt3 = Some(EmissionModuleCnt3Variant::parse(rdr, ver)?);
            cntmax0 = Some(EmissionModuleCntmax0Variant::parse(rdr, ver)?);
            cntmax1 = Some(EmissionModuleCntmax1Variant::parse(rdr, ver)?);
            cntmax2 = Some(EmissionModuleCntmax2Variant::parse(rdr, ver)?);
            cntmax3 = Some(EmissionModuleCntmax3Variant::parse(rdr, ver)?);
            time0 = Some(rdr.read_f32()?);
            time1 = Some(rdr.read_f32()?);
            time2 = Some(rdr.read_f32()?);
            time3 = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_type = Some(rdr.read_i32()?);
            rate = Some(MinMaxCurve::parse(rdr, ver)?);
            cnt0 = Some(EmissionModuleCnt0Variant::parse(rdr, ver)?);
            cnt1 = Some(EmissionModuleCnt1Variant::parse(rdr, ver)?);
            cnt2 = Some(EmissionModuleCnt2Variant::parse(rdr, ver)?);
            cnt3 = Some(EmissionModuleCnt3Variant::parse(rdr, ver)?);
            cntmax0 = Some(EmissionModuleCntmax0Variant::parse(rdr, ver)?);
            cntmax1 = Some(EmissionModuleCntmax1Variant::parse(rdr, ver)?);
            cntmax2 = Some(EmissionModuleCntmax2Variant::parse(rdr, ver)?);
            cntmax3 = Some(EmissionModuleCntmax3Variant::parse(rdr, ver)?);
            time0 = Some(rdr.read_f32()?);
            time1 = Some(rdr.read_f32()?);
            time2 = Some(rdr.read_f32()?);
            time3 = Some(rdr.read_f32()?);
        } else {
            m_type = Some(rdr.read_i32()?);
            rate = Some(MinMaxCurve::parse(rdr, ver)?);
            cnt0 = Some(EmissionModuleCnt0Variant::parse(rdr, ver)?);
            cnt1 = Some(EmissionModuleCnt1Variant::parse(rdr, ver)?);
            cnt2 = Some(EmissionModuleCnt2Variant::parse(rdr, ver)?);
            cnt3 = Some(EmissionModuleCnt3Variant::parse(rdr, ver)?);
            time0 = Some(rdr.read_f32()?);
            time1 = Some(rdr.read_f32()?);
            time2 = Some(rdr.read_f32()?);
            time3 = Some(rdr.read_f32()?);
        }
        m_burst_count = Some(EmissionModuleMBurstCountVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_bursts = Some(EmissionModuleMBurstsVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(EmissionModule {
            enabled: enabled.unwrap(),
            m_type,
            rate_over_time,
            rate,
            rate_over_distance,
            cnt0,
            m_burst_count: m_burst_count.unwrap(),
            cnt1,
            m_bursts,
            cnt2,
            cnt3,
            time0,
            cntmax0,
            time1,
            cntmax1,
            time2,
            cntmax2,
            time3,
            cntmax3,
        })
    }
}

impl crate::ToValue for EmissionModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            crate::opt_field("m_Type", &self.m_type),
            crate::opt_field("rateOverTime", &self.rate_over_time),
            crate::opt_field("rate", &self.rate),
            crate::opt_field("rateOverDistance", &self.rate_over_distance),
            crate::opt_field("cnt0", &self.cnt0),
            Some(("m_BurstCount".to_string(), self.m_burst_count.to_value())),
            crate::opt_field("cnt1", &self.cnt1),
            crate::opt_field("m_Bursts", &self.m_bursts),
            crate::opt_field("cnt2", &self.cnt2),
            crate::opt_field("cnt3", &self.cnt3),
            crate::opt_field("time0", &self.time0),
            crate::opt_field("cntmax0", &self.cntmax0),
            crate::opt_field("time1", &self.time1),
            crate::opt_field("cntmax1", &self.cntmax1),
            crate::opt_field("time2", &self.time2),
            crate::opt_field("cntmax2", &self.cntmax2),
            crate::opt_field("time3", &self.time3),
            crate::opt_field("cntmax3", &self.cntmax3),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "rateOverTime" => (self.rate_over_time).inspect_value(path, offset, limit),
                "rate" => (self.rate).inspect_value(path, offset, limit),
                "rateOverDistance" => (self.rate_over_distance).inspect_value(path, offset, limit),
                "cnt0" => (self.cnt0).inspect_value(path, offset, limit),
                "m_BurstCount" => (self.m_burst_count).inspect_value(path, offset, limit),
                "cnt1" => (self.cnt1).inspect_value(path, offset, limit),
                "m_Bursts" => (self.m_bursts).inspect_value(path, offset, limit),
                "cnt2" => (self.cnt2).inspect_value(path, offset, limit),
                "cnt3" => (self.cnt3).inspect_value(path, offset, limit),
                "time0" => (self.time0).inspect_value(path, offset, limit),
                "cntmax0" => (self.cntmax0).inspect_value(path, offset, limit),
                "time1" => (self.time1).inspect_value(path, offset, limit),
                "cntmax1" => (self.cntmax1).inspect_value(path, offset, limit),
                "time2" => (self.time2).inspect_value(path, offset, limit),
                "cntmax2" => (self.cntmax2).inspect_value(path, offset, limit),
                "time3" => (self.time3).inspect_value(path, offset, limit),
                "cntmax3" => (self.cntmax3).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.rate_over_time).inspect_value(path, offset, limit))
                .map(|v| ("rateOverTime".to_string(), v)),
            ((self.rate).inspect_value(path, offset, limit)).map(|v| ("rate".to_string(), v)),
            ((self.rate_over_distance).inspect_value(path, offset, limit))
                .map(|v| ("rateOverDistance".to_string(), v)),
            ((self.cnt0).inspect_value(path, offset, limit)).map(|v| ("cnt0".to_string(), v)),
            ((self.m_burst_count).inspect_value(path, offset, limit))
                .map(|v| ("m_BurstCount".to_string(), v)),
            ((self.cnt1).inspect_value(path, offset, limit)).map(|v| ("cnt1".to_string(), v)),
            ((self.m_bursts).inspect_value(path, offset, limit))
                .map(|v| ("m_Bursts".to_string(), v)),
            ((self.cnt2).inspect_value(path, offset, limit)).map(|v| ("cnt2".to_string(), v)),
            ((self.cnt3).inspect_value(path, offset, limit)).map(|v| ("cnt3".to_string(), v)),
            ((self.time0).inspect_value(path, offset, limit)).map(|v| ("time0".to_string(), v)),
            ((self.cntmax0).inspect_value(path, offset, limit)).map(|v| ("cntmax0".to_string(), v)),
            ((self.time1).inspect_value(path, offset, limit)).map(|v| ("time1".to_string(), v)),
            ((self.cntmax1).inspect_value(path, offset, limit)).map(|v| ("cntmax1".to_string(), v)),
            ((self.time2).inspect_value(path, offset, limit)).map(|v| ("time2".to_string(), v)),
            ((self.cntmax2).inspect_value(path, offset, limit)).map(|v| ("cntmax2".to_string(), v)),
            ((self.time3).inspect_value(path, offset, limit)).map(|v| ("time3".to_string(), v)),
            ((self.cntmax3).inspect_value(path, offset, limit)).map(|v| ("cntmax3".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
