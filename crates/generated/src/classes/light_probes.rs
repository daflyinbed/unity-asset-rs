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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let baked_positions = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let baked_coefficients = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let tetrahedra = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Tetrahedron::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let hull_rays = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(LightProbes {
            m_name,
            baked_positions,
            baked_coefficients,
            tetrahedra,
            hull_rays,
        })
    }
}
