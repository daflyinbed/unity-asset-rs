//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;
use crate::subclasses::probe_set_index::ProbeSetIndex;
use crate::subclasses::probe_set_tetrahedralization::ProbeSetTetrahedralization;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：LightProbeDataMTetrahedralizationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightProbeDataMTetrahedralizationVariant {
    Default(Box<ProbeSetTetrahedralization>),
    V2017_1(Box<ProbeSetTetrahedralization>),
}

impl crate::Parse for LightProbeDataMTetrahedralizationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { Ok(Box::new(ProbeSetTetrahedralization::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { Ok(Box::new(ProbeSetTetrahedralization::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightProbeDataMProbeSetsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightProbeDataMProbeSetsVariant {
    Default(Vec<ProbeSetIndex>),
    V2017_1(Vec<ProbeSetIndex>),
}

impl crate::Parse for LightProbeDataMProbeSetsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ProbeSetIndex::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ProbeSetIndex::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightProbeDataMPositionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightProbeDataMPositionsVariant {
    Default(Vec<Vector3f>),
    V2017_1(Vec<Vector3f>),
}

impl crate::Parse for LightProbeDataMPositionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct LightProbeData {
    pub m_tetrahedralization: LightProbeDataMTetrahedralizationVariant,
    pub m_probe_sets: LightProbeDataMProbeSetsVariant,
    pub m_positions: LightProbeDataMPositionsVariant,
    pub m_non_tetrahedralized_probe_set_index_map: Vec<(Hash128, i32)>,
}

impl crate::Parse for LightProbeData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_tetrahedralization = LightProbeDataMTetrahedralizationVariant::parse(rdr, ver)?;
        let m_probe_sets = LightProbeDataMProbeSetsVariant::parse(rdr, ver)?;
        let m_positions = LightProbeDataMPositionsVariant::parse(rdr, ver)?;
        let m_non_tetrahedralized_probe_set_index_map = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Hash128::parse(rdr, ver)?; let b = rdr.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(LightProbeData {
            m_tetrahedralization,
            m_probe_sets,
            m_positions,
            m_non_tetrahedralized_probe_set_index_map,
        })
    }
}
