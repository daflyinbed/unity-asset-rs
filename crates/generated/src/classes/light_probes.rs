//! 自动生成。请勿手动修改。

use crate::subclasses::light_probe_data::LightProbeData;
use crate::subclasses::light_probe_occlusion::LightProbeOcclusion;
use crate::subclasses::lightmap_data::LightmapData;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;
use crate::subclasses::tetrahedron::Tetrahedron;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：LightProbesMBakedCoefficientsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightProbesMBakedCoefficientsVariant {
    Default(Vec<SphericalHarmonicsL2>),
    V2017_1(Vec<SphericalHarmonicsL2>),
}

impl crate::Parse for LightProbesMBakedCoefficientsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightProbesMBakedLightOcclusionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightProbesMBakedLightOcclusionVariant {
    Default(Vec<LightProbeOcclusion>),
    V2017_1(Vec<LightProbeOcclusion>),
}

impl crate::Parse for LightProbesMBakedLightOcclusionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LightProbeOcclusion::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LightProbeOcclusion::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct LightProbes {
    pub m_name: String,
    pub baked_positions: Option<Vec<Vector3f>>,
    pub m_data: Option<LightProbeData>,
    pub baked_coefficients: Option<Vec<LightmapData>>,
    pub m_baked_coefficients: Option<LightProbesMBakedCoefficientsVariant>,
    pub tetrahedra: Option<Vec<Tetrahedron>>,
    pub m_baked_light_occlusion: Option<LightProbesMBakedLightOcclusionVariant>,
    pub hull_rays: Option<Vec<Vector3f>>,
    pub m_has_been_edited: Option<bool>,
}

impl crate::Parse for LightProbes {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut baked_positions = None;
        let mut m_data = None;
        let mut baked_coefficients = None;
        let mut m_baked_coefficients = None;
        let mut tetrahedra = None;
        let mut m_baked_light_occlusion = None;
        let mut hull_rays = None;
        let mut m_has_been_edited = None;
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_data = Some(LightProbeData::parse(rdr, ver)?);
            m_baked_coefficients = Some(LightProbesMBakedCoefficientsVariant::parse(rdr, ver)?);
            m_baked_light_occlusion = Some(LightProbesMBakedLightOcclusionVariant::parse(rdr, ver)?);
            m_has_been_edited = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_data = Some(LightProbeData::parse(rdr, ver)?);
            m_baked_coefficients = Some(LightProbesMBakedCoefficientsVariant::parse(rdr, ver)?);
            m_baked_light_occlusion = Some(LightProbesMBakedLightOcclusionVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_data = Some(LightProbeData::parse(rdr, ver)?);
            m_baked_coefficients = Some(LightProbesMBakedCoefficientsVariant::parse(rdr, ver)?);
        } else {
            baked_positions = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            baked_coefficients = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            tetrahedra = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Tetrahedron::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            hull_rays = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }

        Ok(LightProbes {
            m_name: m_name.unwrap(),
            baked_positions,
            m_data,
            baked_coefficients,
            m_baked_coefficients,
            tetrahedra,
            m_baked_light_occlusion,
            hull_rays,
            m_has_been_edited,
        })
    }
}
