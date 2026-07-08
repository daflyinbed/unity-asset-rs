//! 自动生成。请勿手动修改。

use crate::subclasses::lightmap_data::LightmapData;
use crate::subclasses::tetrahedron::Tetrahedron;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct LightProbes {
    pub m_name: String,
    pub baked_positions: Vec<Vector3f>,
    pub baked_coefficients: Vec<LightmapData>,
    pub tetrahedra: Vec<Tetrahedron>,
    pub hull_rays: Vec<Vector3f>,
}

impl crate::Parse for LightProbes {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let baked_positions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let baked_coefficients = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let tetrahedra = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Tetrahedron::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let hull_rays = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(LightProbes {
            m_name,
            baked_positions,
            baked_coefficients,
            tetrahedra,
            hull_rays,
        })
    }
}
