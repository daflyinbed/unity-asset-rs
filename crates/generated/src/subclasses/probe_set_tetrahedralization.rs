//! 自动生成。请勿手动修改。

use crate::subclasses::tetrahedron::Tetrahedron;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ProbeSetTetrahedralization {
    pub m_tetrahedra: Vec<Tetrahedron>,
    pub m_hull_rays: Vec<Vector3f>,
}

impl crate::Parse for ProbeSetTetrahedralization {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_tetrahedra = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Tetrahedron::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_hull_rays = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(ProbeSetTetrahedralization {
            m_tetrahedra,
            m_hull_rays,
        })
    }
}
