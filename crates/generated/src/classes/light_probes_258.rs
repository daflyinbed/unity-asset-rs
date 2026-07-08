//! 自动生成。请勿手动修改。

use crate::subclasses::light_probe_data::LightProbeData;
use crate::subclasses::light_probe_occlusion::LightProbeOcclusion;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;


/// 版本枚举：LightProbes_258MDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightProbes_258MDataVariant {
    Default(LightProbeData),
    V2017_1(LightProbeData),
}

impl crate::Parse for LightProbes_258MDataVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = LightProbeData::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = LightProbeData::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightProbes_258MBakedCoefficientsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightProbes_258MBakedCoefficientsVariant {
    Default(Vec<SphericalHarmonicsL2>),
    V2017_1(Vec<SphericalHarmonicsL2>),
}

impl crate::Parse for LightProbes_258MBakedCoefficientsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightProbes_258MBakedLightOcclusionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightProbes_258MBakedLightOcclusionVariant {
    Default(Vec<LightProbeOcclusion>),
    V2017_1(Vec<LightProbeOcclusion>),
}

impl crate::Parse for LightProbes_258MBakedLightOcclusionVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightProbeOcclusion::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightProbeOcclusion::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct LightProbes_258 {
    pub m_name: String,
    pub m_data: LightProbes_258MDataVariant,
    pub m_baked_coefficients: LightProbes_258MBakedCoefficientsVariant,
    pub m_baked_light_occlusion: Option<LightProbes_258MBakedLightOcclusionVariant>,
    pub m_has_been_edited: Option<bool>,
}

impl crate::Parse for LightProbes_258 {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_data = LightProbes_258MDataVariant::parse(r, ver)?;
        let m_baked_coefficients = LightProbes_258MBakedCoefficientsVariant::parse(r, ver)?;
        let m_baked_light_occlusion = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(LightProbes_258MBakedLightOcclusionVariant::parse(r, ver)?)
        } else {
            None
        };
        let m_has_been_edited = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(LightProbes_258 {
            m_name,
            m_data,
            m_baked_coefficients,
            m_baked_light_occlusion,
            m_has_been_edited,
        })
    }
}
