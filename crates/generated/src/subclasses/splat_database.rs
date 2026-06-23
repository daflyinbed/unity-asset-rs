//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::splat_prototype::SplatPrototype;

#[derive(Debug, Clone)]
pub struct SplatDatabase {
    pub m_splats: Vec<SplatPrototype>,
    pub m_alpha_textures: Vec<PPtr>,
    pub m_alphamap_resolution: i32,
    pub m_base_map_resolution: i32,
}

impl crate::Parse for SplatDatabase {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_splats = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SplatPrototype::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_alpha_textures = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_alphamap_resolution = r.read_i32()?;
        let m_base_map_resolution = r.read_i32()?;

        Ok(SplatDatabase {
            m_splats,
            m_alpha_textures,
            m_alphamap_resolution,
            m_base_map_resolution,
        })
    }
}
