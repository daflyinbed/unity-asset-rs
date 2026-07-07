//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::probe_set_index::ProbeSetIndex;
use crate::subclasses::probe_set_tetrahedralization::ProbeSetTetrahedralization;

#[derive(Debug, Clone)]
pub struct LightProbeData {
    pub m_tetrahedralization: ProbeSetTetrahedralization,
    pub m_probe_sets: Vec<ProbeSetIndex>,
    pub m_positions: Vec<Vector3f>,
    pub m_non_tetrahedralized_probe_set_index_map: Vec<(Hash128, i32)>,
}

impl crate::Parse for LightProbeData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_tetrahedralization = ProbeSetTetrahedralization::parse(r, ver)?;
        let m_probe_sets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ProbeSetIndex::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_positions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_non_tetrahedralized_probe_set_index_map = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Hash128::parse(r, ver)?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(LightProbeData {
            m_tetrahedralization,
            m_probe_sets,
            m_positions,
            m_non_tetrahedralized_probe_set_index_map,
        })
    }
}
