//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Heightmap {
    pub m_heights: Vec<i16>,
    pub m_precomputed_error: Vec<f32>,
    pub m_min_max_patch_heights: Vec<f32>,
    pub m_default_physic_material: PPtr,
    pub m_width: i32,
    pub m_height: i32,
    pub m_levels: i32,
    pub m_scale: Vector3f,
}

impl crate::Parse for Heightmap {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_heights = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i16() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_precomputed_error = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_min_max_patch_heights = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_default_physic_material = crate::PPtr::parse(r, ver)?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_levels = r.read_i32()?;
        let m_scale = Vector3f::parse(r, ver)?;

        Ok(Heightmap {
            m_heights,
            m_precomputed_error,
            m_min_max_patch_heights,
            m_default_physic_material,
            m_width,
            m_height,
            m_levels,
            m_scale,
        })
    }
}
