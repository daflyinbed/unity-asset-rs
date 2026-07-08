//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct RendererData {
    pub uv_mesh: PPtr,
    pub terrain_dynamic_uvst: Vector4f,
    pub terrain_chunk_dynamic_uvst: Vector4f,
    pub lightmap_index: u16,
    pub lightmap_index_dynamic: u16,
    pub lightmap_st: Vector4f,
    pub lightmap_st_dynamic: Vector4f,
    pub explicit_probe_set_hash: Option<Hash128>,
}

impl crate::Parse for RendererData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let uv_mesh = crate::PPtr::parse(rdr, ver)?;
        let terrain_dynamic_uvst = Vector4f::parse(rdr, ver)?;
        let terrain_chunk_dynamic_uvst = Vector4f::parse(rdr, ver)?;
        let lightmap_index = rdr.read_u16()?;
        let lightmap_index_dynamic = rdr.read_u16()?;
        let lightmap_st = Vector4f::parse(rdr, ver)?;
        let lightmap_st_dynamic = Vector4f::parse(rdr, ver)?;
        let explicit_probe_set_hash = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(Hash128::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(RendererData {
            uv_mesh,
            terrain_dynamic_uvst,
            terrain_chunk_dynamic_uvst,
            lightmap_index,
            lightmap_index_dynamic,
            lightmap_st,
            lightmap_st_dynamic,
            explicit_probe_set_hash,
        })
    }
}
