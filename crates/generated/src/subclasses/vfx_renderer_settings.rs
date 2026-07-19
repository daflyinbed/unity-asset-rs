//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxRendererSettings {
    pub motion_vector_generation_mode: i32,
    pub shadow_casting_mode: i32,
    pub receive_shadows: Option<bool>,
    pub ray_tracing_mode: Option<i32>,
    pub reflection_probe_usage: Option<i32>,
    pub light_probe_usage: Option<i32>,
}

impl crate::Parse for VfxRendererSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut motion_vector_generation_mode = None;
        let mut shadow_casting_mode = None;
        let mut receive_shadows = None;
        let mut ray_tracing_mode = None;
        let mut reflection_probe_usage = None;
        let mut light_probe_usage = None;
        motion_vector_generation_mode = Some(rdr.read_i32()?);
        shadow_casting_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            ray_tracing_mode = Some(rdr.read_i32()?);
            receive_shadows = Some(rdr.read_bool()?);
            rdr.align(4)?;
            reflection_probe_usage = Some(rdr.read_i32()?);
            light_probe_usage = Some(rdr.read_i32()?);
        } else {
            receive_shadows = Some(rdr.read_bool()?);
            rdr.align(4)?;
            reflection_probe_usage = Some(rdr.read_i32()?);
            light_probe_usage = Some(rdr.read_i32()?);
        }

        Ok(VfxRendererSettings {
            motion_vector_generation_mode: motion_vector_generation_mode.unwrap(),
            shadow_casting_mode: shadow_casting_mode.unwrap(),
            receive_shadows,
            ray_tracing_mode,
            reflection_probe_usage,
            light_probe_usage,
        })
    }
}
