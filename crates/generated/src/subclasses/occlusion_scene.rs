//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct OcclusionScene {
    pub index_renderers: i32,
    pub size_renderers: i32,
    pub index_portals: i32,
    pub size_portals: i32,
    pub scene: Guid,
}

impl crate::Parse for OcclusionScene {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let index_renderers = rdr.read_i32()?;
        let size_renderers = rdr.read_i32()?;
        let index_portals = rdr.read_i32()?;
        let size_portals = rdr.read_i32()?;
        let scene = Guid::parse(rdr, ver)?;

        Ok(OcclusionScene {
            index_renderers,
            size_renderers,
            index_portals,
            size_portals,
            scene,
        })
    }
}
