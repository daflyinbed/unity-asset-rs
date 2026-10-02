//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;
use crate::subclasses::probe_set_index::ProbeSetIndex;
use crate::subclasses::probe_set_tetrahedralization::ProbeSetTetrahedralization;
use crate::subclasses::vector3f::Vector3f;

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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ProbeSetIndex::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ProbeSetIndex::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LightProbeDataMProbeSetsVariant {
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LightProbeDataMPositionsVariant {
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
#[derive(Debug, Clone)]
pub struct LightProbeData {
    pub m_tetrahedralization: ProbeSetTetrahedralization,
    pub m_probe_sets: LightProbeDataMProbeSetsVariant,
    pub m_positions: LightProbeDataMPositionsVariant,
    pub m_non_tetrahedralized_probe_set_index_map: Vec<(Hash128, i32)>,
}

impl crate::Parse for LightProbeData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_tetrahedralization = ProbeSetTetrahedralization::parse(rdr, ver)?;
        let m_probe_sets = LightProbeDataMProbeSetsVariant::parse(rdr, ver)?;
        let m_positions = LightProbeDataMPositionsVariant::parse(rdr, ver)?;
        let m_non_tetrahedralized_probe_set_index_map = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = Hash128::parse(rdr, ver)?;
                        let b = rdr.read_i32()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(LightProbeData {
            m_tetrahedralization,
            m_probe_sets,
            m_positions,
            m_non_tetrahedralized_probe_set_index_map,
        })
    }
}

impl crate::ToValue for LightProbeData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_Tetrahedralization".to_string(),
                self.m_tetrahedralization.to_value(),
            )),
            Some(("m_ProbeSets".to_string(), self.m_probe_sets.to_value())),
            Some(("m_Positions".to_string(), self.m_positions.to_value())),
            Some((
                "m_NonTetrahedralizedProbeSetIndexMap".to_string(),
                self.m_non_tetrahedralized_probe_set_index_map.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Tetrahedralization" => {
                    (self.m_tetrahedralization).inspect_value(path, offset, limit)
                }
                "m_ProbeSets" => (self.m_probe_sets).inspect_value(path, offset, limit),
                "m_Positions" => (self.m_positions).inspect_value(path, offset, limit),
                "m_NonTetrahedralizedProbeSetIndexMap" => (self
                    .m_non_tetrahedralized_probe_set_index_map)
                    .inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_tetrahedralization).inspect_value(path, offset, limit))
                .map(|v| ("m_Tetrahedralization".to_string(), v)),
            ((self.m_probe_sets).inspect_value(path, offset, limit))
                .map(|v| ("m_ProbeSets".to_string(), v)),
            ((self.m_positions).inspect_value(path, offset, limit))
                .map(|v| ("m_Positions".to_string(), v)),
            ((self.m_non_tetrahedralized_probe_set_index_map).inspect_value(path, offset, limit))
                .map(|v| ("m_NonTetrahedralizedProbeSetIndexMap".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
