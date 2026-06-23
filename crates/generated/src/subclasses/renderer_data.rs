//! 自动生成。请勿手动修改。

use crate::PPtr;
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
}

impl crate::Parse for RendererData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let uv_mesh = crate::PPtr::parse(r, ver)?;
        let terrain_dynamic_uvst = Vector4f::parse(r, ver)?;
        let terrain_chunk_dynamic_uvst = Vector4f::parse(r, ver)?;
        let lightmap_index = r.read_u16()?;
        let lightmap_index_dynamic = r.read_u16()?;
        let lightmap_st = Vector4f::parse(r, ver)?;
        let lightmap_st_dynamic = Vector4f::parse(r, ver)?;

        Ok(RendererData {
            uv_mesh,
            terrain_dynamic_uvst,
            terrain_chunk_dynamic_uvst,
            lightmap_index,
            lightmap_index_dynamic,
            lightmap_st,
            lightmap_st_dynamic,
        })
    }
}
