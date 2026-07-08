//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightmapBakeMode {
    pub lightmap_bake_type: i32,
    pub mixed_lighting_mode: i32,
}

impl crate::Parse for LightmapBakeMode {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let lightmap_bake_type = rdr.read_i32()?;
        let mixed_lighting_mode = rdr.read_i32()?;

        Ok(LightmapBakeMode {
            lightmap_bake_type,
            mixed_lighting_mode,
        })
    }
}
