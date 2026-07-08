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
        let use_screen_space_shadows = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let use_cascaded_shadow_maps = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(PlatformShaderSettings {
            use_screen_space_shadows,
            use_cascaded_shadow_maps,
        })
    }
}
