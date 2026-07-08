//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TierGraphicsSettingsEditor {
    pub standard_shader_quality: i32,
    pub rendering_path: i32,
    pub use_cascaded_shadow_maps: bool,
    pub use_reflection_probe_box_projection: bool,
    pub use_reflection_probe_blending: bool,
}

impl crate::Parse for TierGraphicsSettingsEditor {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let standard_shader_quality = rdr.read_i32()?;
        let rendering_path = rdr.read_i32()?;
        let use_cascaded_shadow_maps = rdr.read_bool()?;
        let use_reflection_probe_box_projection = rdr.read_bool()?;
        let use_reflection_probe_blending = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(TierGraphicsSettingsEditor {
            standard_shader_quality,
            rendering_path,
            use_cascaded_shadow_maps,
            use_reflection_probe_box_projection,
            use_reflection_probe_blending,
        })
    }
}
