//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct EnlightenRendererInformation {
    pub renderer: PPtr,
    pub dynamic_lightmap_st_in_system: Vector4f,
    pub system_id: i32,
    pub instance_hash: Hash128,
}

impl crate::Parse for EnlightenRendererInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let renderer = crate::PPtr::parse(rdr, ver)?;
        let dynamic_lightmap_st_in_system = Vector4f::parse(rdr, ver)?;
        let system_id = rdr.read_i32()?;
        let instance_hash = Hash128::parse(rdr, ver)?;

        Ok(EnlightenRendererInformation {
            renderer,
            dynamic_lightmap_st_in_system,
            system_id,
            instance_hash,
        })
    }
}
