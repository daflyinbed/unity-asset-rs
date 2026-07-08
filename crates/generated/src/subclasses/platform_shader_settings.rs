//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformShaderSettings {
    pub use_screen_space_shadows: Option<bool>,
    pub use_cascaded_shadow_maps: Option<bool>,
}

impl crate::Parse for PlatformShaderSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut use_screen_space_shadows = None;
        let mut use_cascaded_shadow_maps = None;
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            use_cascaded_shadow_maps = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            use_screen_space_shadows = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }

        Ok(PlatformShaderSettings {
            use_screen_space_shadows,
            use_cascaded_shadow_maps,
        })
    }
}
