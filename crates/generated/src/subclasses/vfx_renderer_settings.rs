//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxRendererSettings {
    pub motion_vector_generation_mode: i32,
    pub shadow_casting_mode: i32,
    pub receive_shadows: bool,
    pub reflection_probe_usage: i32,
    pub light_probe_usage: i32,
}

impl crate::Parse for VfxRendererSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let motion_vector_generation_mode = r.read_i32()?;
        let shadow_casting_mode = r.read_i32()?;
        let receive_shadows = r.read_bool()?;
        r.align(4)?;
        let reflection_probe_usage = r.read_i32()?;
        let light_probe_usage = r.read_i32()?;

        Ok(VfxRendererSettings {
            motion_vector_generation_mode,
            shadow_casting_mode,
            receive_shadows,
            reflection_probe_usage,
            light_probe_usage,
        })
    }
}
