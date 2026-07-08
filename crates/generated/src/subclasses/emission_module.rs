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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ParticleSystemEmissionBurst::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ParticleSystemEmissionBurst::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
