//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct OcclusionBakeSettings {
    pub view_cell_size: f32,
    pub bake_mode: u32,
    pub memory_usage: u32,
}

impl crate::Parse for OcclusionBakeSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let view_cell_size = r.read_f32()?;
        let bake_mode = r.read_u32()?;
        let memory_usage = r.read_u32()?;

        Ok(OcclusionBakeSettings {
            view_cell_size,
            bake_mode,
            memory_usage,
        })
    }
}
