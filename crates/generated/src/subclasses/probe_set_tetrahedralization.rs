//! 自动生成。请勿手动修改。

use crate::subclasses::tetrahedron::Tetrahedron;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：ProbeSetTetrahedralizationMTetrahedraVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProbeSetTetrahedralizationMTetrahedraVariant {
    Default(Vec<Tetrahedron>),
    V2017_1(Vec<Tetrahedron>),
}

impl crate::Parse for ProbeSetTetrahedralizationMTetrahedraVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Tetrahedron::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Tetrahedron::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ProbeSetTetrahedralizationMHullRaysVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProbeSetTetrahedralizationMHullRaysVariant {
    Default(Vec<Vector3f>),
    V2017_1(Vec<Vector3f>),
}

impl crate::Parse for ProbeSetTetrahedralizationMHullRaysVariant {
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
pub struct ProbeSetTetrahedralization {
    pub m_tetrahedra: ProbeSetTetrahedralizationMTetrahedraVariant,
    pub m_hull_rays: ProbeSetTetrahedralizationMHullRaysVariant,
}

impl crate::Parse for ProbeSetTetrahedralization {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_tetrahedra = ProbeSetTetrahedralizationMTetrahedraVariant::parse(rdr, ver)?;
        let m_hull_rays = ProbeSetTetrahedralizationMHullRaysVariant::parse(rdr, ver)?;

        Ok(ProbeSetTetrahedralization {
            m_tetrahedra,
            m_hull_rays,
        })
    }
}
